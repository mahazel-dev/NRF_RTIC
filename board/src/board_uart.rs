use crate::hal::prelude::*;
pub use crate::hal::pac::{uart0, UART0};
pub use uart0::{baudrate::BAUDRATE_A as Uart_Baudrate, config::PARITY_A as Uart_Parity};
pub use crate::hal::uarte::Pins as UartPins;

pub struct Uart(UART0);



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

        let u = Uart(uart);
        u
    }


    pub fn write_byte(&mut self, byte: u8)  {
        self.0.txd.write(|w| unsafe { w.bits(u32::from(byte)) });
    }

    pub fn write_str(&mut self, string: &str)   {
        self.0.tasks_starttx.write(|w| unsafe { w.bits(1) });

        let msg = string.as_bytes(); //.iter().map(|byte| byte + 1);

        for sign in msg    {
            self.write_byte(*sign);
            while self.0.events_txdrdy.read().bits() == 0   {}
            self.0.events_txdrdy.reset();
        }
        
        self.0.tasks_stoptx.write(|w| unsafe { w.bits(1) });
    }
}



