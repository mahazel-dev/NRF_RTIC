use nrf52840_hal::prelude::OutputPin;

pub use crate::hal::uarte::*;
pub use crate::hal::pac::{uarte0, UARTE0};
use crate::hal::target_constants::EASY_DMA_SIZE;

use core::sync::atomic::{compiler_fence, Ordering::SeqCst};

pub struct CanProtocol<T>(T);

impl<T> CanProtocol<T>
where
    T: Instance,
{
    pub fn transmit(&mut self, tx_buffor: u32, tx_len: u16) ->  Result<(), Error>  {
        if tx_len == 0 {
            return Err(Error::TxBufferTooSmall);
        }

        if *&tx_len as usize > EASY_DMA_SIZE {
            return Err(Error::TxBufferTooLong);
        }

        // We can only DMA out of RAM.
        //slice_in_ram_or(tx_buffer, Error::BufferNotInRAM)?;

        //defmt::debug!("Before start_transmit");
        self.start_transmit(tx_buffor, tx_len);
        // Wait for transmission to end.
        while self.0.events_endtx.read().bits() == 0 {
            // TODO: Do something here which uses less power. Like `wfi`.
        }
        //defmt::debug!("After start_transmit and loop");


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

        // Optional pins
        uarte.psel.cts.write(|w| {
            if let Some(ref pin) = pins.cts {
                unsafe { w.bits(pin.psel_bits()) };
                w.connect().connected()
            } else {
                w.connect().disconnected()
            }
        });

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

        let mut u = CanProtocol(uarte);

        // Enable UARTE instance.
        u.0.enable.write(|w| w.enable().enabled());

        u

    }


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