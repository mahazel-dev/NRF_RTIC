use crate::hal::prelude::*;
pub use crate::hal::pac::{uart0, UART0};
pub use uart0::{baudrate::BAUDRATE_A as Uart_Baudrate, config::PARITY_A as Uart_Parity};
pub use crate::hal::uarte::Pins as UartPins;

pub struct Uart(UART0);

static mut UART_RX_FRAME: [u8; 6] = [0x36; 6];


impl Uart {
    pub fn new(uart: UART0, mut pins: UartPins, parity: Uart_Parity, baudrate: Uart_Baudrate) -> Self    {
        
        // Is the UART already on? It might be if you had a bootloader
        if uart.enable.read().bits() != 0 {
            uart.tasks_stoptx.write(|w| unsafe { w.bits(1) });
            // Disable UARTE instance
            uart.enable.write(|w| w.enable().disabled());
        }


        // Select pins
        uart.psel.rxd.write(|w| {
            unsafe { w.bits(pins.rxd.psel_bits()) };
            w.connect().connected()
        });

        pins.txd.set_high().unwrap();

        uart.psel.txd.write(|w| {
            unsafe { w.bits(pins.txd.psel_bits()) };
            w.connect().connected()
        });

        // Optional pins
        uart.psel.cts.write(|w| {
            if let Some(ref pin) = pins.cts {
                unsafe { w.bits(pin.psel_bits()) };
                w.connect().connected()
            } else {
                w.connect().disconnected()
            }
        });

        uart.psel.rts.write(|w| {
            if let Some(ref pin) = pins.rts {
                unsafe { w.bits(pin.psel_bits()) };
                w.connect().connected()
            } else {
                w.connect().disconnected()
            }
        });

        // Set parity.
        let hardware_flow_control = pins.rts.is_some() && pins.cts.is_some();
        uart.config
            .write(|w| w.hwfc().bit(hardware_flow_control).parity().variant(parity));

        // Set baud rate.
        uart.baudrate.write(|w| w.baudrate().variant(baudrate));
        
        // Enable UART function.
        uart.enable.write(|w| w.enable().enabled());

        // Fire up receiving data
        uart.intenset.write(|w| w.rxdrdy().set());
        //uart.tasks_startrx.write(|w| unsafe {w.bits(1)});

        let u = Uart(uart);
        u
        
    }

    pub fn transmit_byte(&mut self, byte: u8)  {
        // Fire up transmitting data
        self.0.tasks_starttx.write(|w| unsafe { w.bits(1) });

        // Send byte
        self.0.txd.write(|w| unsafe { w.bits(u32::from(byte)) });

        // Blocker
        while self.0.events_txdrdy.read().bits() == 0   {}
        
        // Unlock event to be able to transmit next byte
        self.0.events_txdrdy.reset();

        // Stop transmitting data
        self.0.tasks_stoptx.write(|w| unsafe { w.bits(1) });
    }

    pub fn transmit_str(&mut self, string: &str)   {
        // Fire up transmitting data
        self.0.tasks_starttx.write(|w| unsafe { w.bits(1) });

        // Create slcie from string 
        let msg = string.as_bytes(); //.iter().map(|byte| byte + 1);

        // Iterate string and send byte
        for sign in msg    {
            // Send byte
            self.0.txd.write(|w| unsafe { w.bits(u32::from(*sign)) });
            // Blocker
            while self.0.events_txdrdy.read().bits() == 0   {}
            // Unlock event to be able to transmit next byte
            self.0.events_txdrdy.reset();
        }
        // Stop transmitting data
        self.0.tasks_stoptx.write(|w| unsafe { w.bits(1) });
    }


    pub fn transmit_frame(&mut self, frame: [u8; 6])   {
        // Fire up transmitting data
        self.0.tasks_starttx.write(|w| unsafe { w.bits(1) });

        // Create buffor
        let msg = frame; //.iter().map(|byte| byte + 1);

        // Iterate array and send byte
        for byte in msg    {
            // Send byte
            self.0.txd.write(|w| unsafe { w.bits(u32::from(byte)) });
            // Blocker
            while self.0.events_txdrdy.read().bits() == 0   {}
            // Unlock event to be able to transmit next byte
            self.0.events_txdrdy.reset();
        }
        // Stop transmitting data
        self.0.tasks_stoptx.write(|w| unsafe { w.bits(1) });
    }

    // Read byte from UART reveceiver (from FIFO)
    pub fn read_byte(&mut self) -> u8 {
        // Read byte from FIFO stack
        let byte = self.0.rxd.read().bits() as u8;
        // Wait blocker
        self.wait_for_byte();
        // Release event interrupt
        self.clear_rxdrdy();
        // Return byte
        byte
    }


    // Read 6 bytes command 
    pub fn read_command(&mut self)  {
        let mut x: [u8; 6] = [0x00; 6];
        // Fire up transmitting data
        self.0.tasks_startrx.write(|w| unsafe {w.bits(1)});

        /// TO CHANGE
        for i in 0..6   {
            x[i] = self.read_byte();
        }

        unsafe { UART_RX_FRAME = x ;}
        self.transmit_frame(unsafe { UART_RX_FRAME } );
        self.0.tasks_stoprx.write(|w| unsafe {w.bits(1)});

    }

    pub fn clear_rxdrdy(&mut self)  {
        self.0.events_rxdrdy.reset();
    }

    pub fn wait_for_byte(&mut self)  { //add timeout
        while self.0.events_rxdrdy.read().events_rxdrdy().bit_is_clear() {}
    }
}




