use crate::hal_main as hal;

pub use hal::uarte::*;
pub use hal::pac::{uarte0, UARTE0};

use hal::prelude::OutputPin;
use hal::target_constants::EASY_DMA_SIZE;
use core::sync::atomic::{compiler_fence, Ordering::SeqCst};


pub struct Uarte<T>(T);

impl<T> Uarte<T>
where
    T: Instance,
{
    pub fn is_cts(&mut self) -> bool    {
        self.0.events_cts.read().events_cts().bit_is_set()
    }

    pub fn clear_cts_event(&mut self)   {
        self.0.events_cts.reset();
        while self.0.events_cts.read().events_cts().bit_is_set() == true  {}
    }

    pub fn is_ncts(&mut self) -> bool    {
        //self.0.events_ncts.read().events_ncts().bit_is_set()

        self.0.events_endrx.read().events_endrx().bit_is_set()
    }


    pub fn receive(&mut self, rx_buffor: u32, rx_len: u8) -> Result<(), Error> {
        self.start_receive(rx_buffor, rx_len)?;

        // Wait for transmission to end.
        while self.0.events_endrx.read().bits() == 0 {}

        self.finalize_receive();

        /*
        if self.0.rxd.amount.read().bits() != rx_buffer.len() as u32 {
            return Err(Error::Receive);
        }
        */

        Ok(())
    }
/// Start a UARTE read transaction by setting the control
/// values and triggering a read task.
    fn start_receive(&mut self, rx_buffor: u32, rx_len: u8) -> Result<(), Error> {
        if rx_len == 0 {
            return Err(Error::RxBufferTooSmall);
        }
    
        if *&rx_len as usize > EASY_DMA_SIZE {
            return Err(Error::RxBufferTooLong);
        }

        compiler_fence(SeqCst);

        // Set up the DMA read
        self.0.rxd.ptr.write(|w| unsafe { 
            w.ptr().bits(rx_buffor) }); 
        self.0.rxd.maxcnt.write(|w|unsafe {
            w.maxcnt().bits(rx_len as u16)  });
    
        // Start UARTE Receive transaction.
        self.0.tasks_startrx.write(|w|
                // `1` is a valid value to write to task registers.
                unsafe { w.bits(1) });

        Ok(())

    }

    /// Stop an unfinished UART read transaction and flush FIFO to DMA buffer.
    fn cancel_receive(&mut self) {
        // Stop reception.
        self.0.tasks_stoprx.write(|w| unsafe { w.bits(1) });

        // Wait for the reception to have stopped.
        while self.0.events_rxto.read().bits() == 0 {}

        // Reset the event flag.
        self.0.events_rxto.write(|w| w);

        // Ask UART to flush FIFO to DMA buffer.
        self.0.tasks_flushrx.write(|w| unsafe { w.bits(1) });

        // Wait for the flush to complete.
        while self.0.events_endrx.read().bits() == 0 {}

        // The event flag itself is later reset by `finalize_read`.
    }

    /// Finalize a UARTE read transaction by clearing the event.
    pub fn finalize_receive(&mut self) {
    // Reset the event, otherwise it will always read `1` from now on.
    self.0.events_endrx.write(|w| w.events_endrx().clear_bit());

    // Conservative compiler fence to prevent optimizations that do not
    // take in to account actions by DMA. The fence has been placed here,
    // after all possible DMA actions have completed.
    compiler_fence(SeqCst);
}



    pub fn transmit(&mut self, tx_buffor: u32, tx_len: u16) ->  Result<(), Error>  {
        if tx_len == 0 {
            return Err(Error::TxBufferTooSmall);
        }

        if *&tx_len as usize > EASY_DMA_SIZE {
            return Err(Error::TxBufferTooLong);
        }

        // We can only DMA out of RAM.
        //slice_in_ram_or(tx_buffer, Error::BufferNotInRAM)?;

        self.start_transmit(tx_buffor, tx_len);

        // Wait for transmission to end.
        while self.0.events_endtx.read().bits() == 0 {
            // TODO: Do something here which uses less power. Like `wfi`.
        }

        
        // Conservative compiler fence to prevent optimizations that do not
        // take in to account actions by DMA. The fence has been placed here,
        // after all possible DMA actions have completed.
        compiler_fence(SeqCst);


        // Reset the event
        self.0.events_txstopped.reset();

        // Stop transmit and return Result Ok
        self.stop_transmit();

        Ok(())


    }

    fn start_transmit(&mut self, tx_buffor: u32, tx_len: u16) {
        compiler_fence(SeqCst);

        // Reset the events.
        self.0.events_endtx.reset();
        self.0.events_txstopped.reset();


        // Set up the DMA write.
        self.0.txd.ptr.write(|w| unsafe { w.ptr().bits(tx_buffor) });
        // Set length of frame
        self.0.txd.maxcnt.write(|w|unsafe { w.maxcnt().bits(tx_len) });

        // Start UARTE Transmit transaction.
        self.0.tasks_starttx.write(|w| // `1` is a valid value to write to task registers.
            unsafe { w.bits(1) });
    }

    fn stop_transmit(&mut self) {
        // Stop transmit
        self.0.tasks_stoptx.write(|w| unsafe { w.bits(1) });

        // Wait for transmitter is stopped.
        while self.0.events_txstopped.read().bits() == 0 {}
    }



    pub fn new(uarte: T, mut pins: Pins, parity: Parity, baudrate: Baudrate) -> Self {
        // Is the UART already on? It might be if you had a bootloader
        if uarte.enable.read().bits() != 0 {
            uarte.tasks_stoptx.write(|w| unsafe { w.bits(1) });
            while uarte.events_txstopped.read().bits() == 0 {
                // Spin
            }

            // Disable UARTE instance
            uarte.enable.write(|w| w.enable().disabled());
        }

        // Select pins
        uarte.psel.rxd.write(|w| {
            unsafe { w.bits(pins.rxd.psel_bits()) };
            w.connect().connected()
        });
        pins.txd.set_high().unwrap();
        uarte.psel.txd.write(|w| {
            unsafe { w.bits(pins.txd.psel_bits()) };
            w.connect().connected()
        });

        // Optional pin CTS 
        uarte.psel.cts.write(|w| {
            if let Some(ref pin) = pins.cts {
                unsafe { w.bits(pin.psel_bits()) };
                w.connect().connected()
            } else {
                w.connect().disconnected()
            }
        });

        // Optional pin RTS
        uarte.psel.rts.write(|w| {
            if let Some(ref pin) = pins.rts {
                unsafe { w.bits(pin.psel_bits()) };
                w.connect().connected()
            } else {
                w.connect().disconnected()
            }
        });

        // Configure.
        let hardware_flow_control = pins.rts.is_some() && pins.cts.is_some();
        uarte
            .config
            .write(|w| w.hwfc().bit(hardware_flow_control).parity().variant(parity));

        // Configure frequency.
        uarte.baudrate.write(|w| w.baudrate().variant(baudrate));

        let u = Uarte(uarte);

        // Enable UARTE instance.
        u.0.enable.write(|w| w.enable().enabled());

        u.0.intenset.write(|w| w.cts().set());
        //u.0.intenset.write(|w| w.ncts().set());

        u

    }



}

pub use hal::gpio::{ Pin, Output, Input, Floating, PullUp, PushPull};
pub struct Pins {
    pub rxd: Pin<Input<Floating>>,
    pub txd: Pin<Output<PushPull>>,
    pub cts: Option<Pin<Input<PullUp>>>,
    pub rts: Option<Pin<Output<PushPull>>>,
}


/*
pub trait MyUarte {
    //fn write_string(&mut self, string: &str, dma: &mut DmaUarteBuffor);
    //fn write_frame(&mut self, frame: [u8; 8], dma: &mut DmaUarteBuffor);
    //fn new_split(&mut self)
    fn start_transmit(&mut self, tx_buffer: usize);
    fn transmit(&mut self, tx_buffer: usize) -> Result<(), Error>;
}

impl <T> MyUarte for Uarte<T> where T:Instance {
    fn start_transmit(&mut self, tx_buffer: usize) {
    // Conservative compiler fence to prevent optimizations that do not
    // take in to account actions by DMA. The fence has been placed here,
    // before any DMA action has started.
    compiler_fence(SeqCst);

    // Reset the events.
    self.0.events_endtx.reset();
    self.events_txstopped.reset();


    }

    fn transmit(&mut self, tx_buffer: usize) -> Result<(), Error> {
        if tx_buffer == 0 {
            return Err(Error::TxBufferTooSmall);
        }

        if tx_buffer > EASY_DMA_SIZE {
            return Err(Error::TxBufferTooLong);
        }

        Ok(())

    }

}

*/
/* 
impl <T> MyUarte for Uarte<T> where T:Instance {
    fn write_string(&mut self, string: &str, dma: &mut DmaUarteBuffor) {
        let tx_buffor = &dma.TxBlock;
        let buf_size = str_to_ptr(string, tx_buffor);

        let frame = unsafe { from_raw_parts(*tx_buffor as *mut u8, buf_size + 1) };

        let _ = &self.write(frame).unwrap();
    }

    fn write_frame(&mut self, frame: [u8; 8], dma: &mut DmaUarteBuffor) {
        unsafe { dma.TxBlock.write(frame) };

        let frame = unsafe { from_raw_parts(dma.TxBlock as *mut u8, 8) };
        let _ = &self.write(frame).unwrap();

    }
}


 fn str_to_ptr(string: &str, ptr: &*mut [u8; 8]) -> usize {
    let msg: &str;
    let mut frame: [u8; 8] = [0x0A, 0x23, 0x23, 0x23, 0x23, 0x23, 0x23, 0x23];

    if string.len() > 7  { msg = UARTE_TX_BUF_DEF;
                unsafe { ptr.write(frame) }
                msg.len()
    } else {    msg = string ;
                let msg = msg.as_bytes();

                for i in 1..=msg.len() {
                    frame[i] = msg[i - 1];
                }
                unsafe { ptr.write(frame) };
                msg.len()
            }
        }
*/