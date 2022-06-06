use crate::hal_main as hal;

mod lib_dma;
mod lib_gpiote;
mod lib_nfc;
mod lib_uarte;

pub use lib_dma::*;
pub use lib_gpiote::*;
pub use lib_nfc::*;
pub use lib_uarte::*;

pub use hal::{gpio, gpio::*,
    clocks, Clocks,
    Timer, timer::OneShot,};

pub use hal::pac::{interrupt, Interrupt, NVIC_PRIO_BITS, 
    TIMER0,
};

pub fn init_board()   -> Result<Device, ()>   {
    if let Some(periph) = hal::pac::Peripherals::take() {

        // ********** CLOCK Configuration ********** 
        static mut CLOCKS: Option<Clocks<clocks::ExternalOscillator,
        clocks::ExternalOscillator, clocks::LfOscStarted>> = None;

        let board_clocks = Clocks::new(periph.CLOCK).enable_ext_hfosc();
        let board_clocks = board_clocks.set_lfclk_src_external(clocks::LfOscConfiguration::NoExternalNoBypass);
        let board_clocks = board_clocks.start_lfclk();

        // 64MhZ external crystal, 32kHz external crystal sd
        unsafe { CLOCKS.get_or_insert(board_clocks) };

        //defmt::debug!("Initializing the board's peripherals");

        // ********** GPIO Configuration ********** 
        let pins = gpio::p0::Parts::new(periph.P0);
        
        // LED config
        let led_1 = pins.p0_13.degrade().into_push_pull_output(gpio::Level::High);
        let led_2 = pins.p0_14.degrade().into_push_pull_output(gpio::Level::High);
        let led_3 = pins.p0_15.degrade().into_push_pull_output(gpio::Level::High);
        let led_4 = pins.p0_16.degrade().into_push_pull_output(gpio::Level::High);
        
        // General buttons config
        let button_1 = pins.p0_11.degrade().into_pullup_input();
        let button_2 = pins.p0_12.degrade().into_pullup_input();
        let button_3 = pins.p0_24.degrade().into_pullup_input();
        let button_4 = pins.p0_25.degrade().into_pullup_input();
        
        // ********** GPIOTE Configuration **********
        let board_gpiote = Gpiote::new(periph.GPIOTE);
        // Interuppter button
        board_gpiote.port().input_pin(&button_1).low();
        board_gpiote.port().input_pin(&button_2).low();
        board_gpiote.port().input_pin(&button_3).low();
        board_gpiote.port().input_pin(&button_4).low();
        board_gpiote.port().enable_interrupt();

 

        // Blocker for button - to delete, LEARN Monotonics
        let blocker = hal::Timer::one_shot(periph.TIMER0);

        // ********** UARTE configuration Configuration **********
        // UARTE unwrap and basic configure
        let board_uarte = Uarte::new(periph.UARTE0,
            Pins {
                rxd: pins.p0_08.degrade().into_floating_input(),
                txd: pins.p0_06.degrade().into_push_pull_output(gpio::Level::High),
                cts: Some(pins.p0_07.degrade().into_pullup_input()),
                //cts: None,
                //rts: None,
                rts: Some(pins.p0_05.degrade().into_push_pull_output(gpio::Level::High)),
            },
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        /*
        board_gpiote.channel0()
            .input_pin(&pins.p0_07.degrade().into_floating_input())
            .hi_to_lo()
            .enable_interrupt();
        */


        //let dma_uarte = DmaCanBuffor::new(4, 4);
 
        let board_dma = DmaBuffor::new();
        
        //unsafe {dma_uarte.TxBlock.write([0x31, 0x31, 0x31, 0x31, 0x31, 0x31, 0x31, 0x31]);}
        //unsafe { *dma_can.tx_block = [0x0A, 0x31, 0x32, 0x33]; } //, 0x34, 0x35, 0x36, 0x37]};

        unsafe {board_dma.ptr.uarte_tx.write([0x0A, 0x31, 0x32, 0x33]); }


        // ********** NFCT configuration Configuration **********
        let board_nfct = Nfct::new(periph.NFCT);


        // **********!! Return Result<Device, Err) !!**********    
        Ok(Device {
            leds: Leds  {
                _1: Led { inner: led_1 },
                _2: Led { inner: led_2 },
                _3: Led { inner: led_3 },
                _4: Led { inner: led_4 },
            },

            buttons: Buttons    {
                _1: Button { inner: button_1 },
                _2: Button { inner: button_2 },
                _3: Button { inner: button_3 },
                _4: Button { inner: button_4 },
            },

            blocking_timer: ButtonBlocker   {
                inner: blocker,
            },

            board_gpiote: board_gpiote,

            board_nfct: board_nfct,

            board_uarte: board_uarte,

            board_dma: board_dma,

        })
        
    } else  {
        Err(())
    }
}

/* */


pub struct Device   {
    /// Add LEDs to my board
    pub leds: Leds,
    /// Add Buttons to my board
    pub buttons: Buttons,
    /// Add timer for general delay
    pub blocking_timer: ButtonBlocker,
    // Add GPIOTE feature
    pub board_gpiote: Gpiote,
    // Add Uart feature
    //pub board_uart: Uart,
    // Add UARTE 
    pub board_uarte: Uarte<UARTE0>,
    // Add NFCT feature
    pub board_nfct: Nfct,
    // DMA Handler
    pub board_dma: DmaBuffor,

}


use embedded_hal::digital::v2::
    {OutputPin as _, InputPin as _,
        StatefulOutputPin};

pub struct Leds {
    // LED1: pin P0.13, green
    pub _1: Led,
    // LED2: pin P0.14, green
    pub _2: Led,
    // LED3: pin P0.15, green
    pub _3: Led,
    // LED4: pin P0.16, green
    pub _4: Led,
}

pub struct Led  {
    pub inner: Pin<Output<PushPull>>
}

impl Led    {
    /// Turns on LED 
    pub fn on(&mut self)    {
        
        /*defmt::trace!(
            "setting P{}.{} low (LED on)",

            if self.inner.port() == Port::Port1 {
                '1'
            }   else {
                '0'
            },
            self.inner.pin()
        );*/
        
        
        let _ = self.inner.set_low();
    }

    /// Turns off LED 
    pub fn off(&mut self)    {
        
        /*defmt::trace!(
            "setting P{}.{} low (LED on)",

            if self.inner.port() == Port::Port1 {
                '1'
            }   else {
                '0'
            },
            self.inner.pin()
        );*/
        
        let _ = self.inner.set_high();
    }

    pub fn toggle(&mut self)    {
        if self.is_on() {
            self.off();
        } else {
            self.on();
        }
    }

    /// Returns `true` if the LED is in the OFF state
    pub fn is_off(&self) -> bool {
        self.inner.is_set_high() == Ok(true)
    }
    
    /// Returns `true` if the LED is in the ON state
    pub fn is_on(&self) -> bool {
         !self.is_off()
    }
}

pub struct Buttons {
        // Button1: pin P0.11, green
        pub _1: Button,
        // Button2: pin P0.12, green
        pub _2: Button,
        // Button3: pin P0.24, green
        pub _3: Button,
        // Button4: pin P0.25, green
        pub _4: Button,
}

pub struct Button   {
    pub inner: Pin<Input<PullUp>>
}

impl Button {
    pub fn is_pushed(&self) -> bool   {
        self.inner.is_high() != Ok(true)
    }
}


// Lets try to implement start later
pub struct ButtonBlocker {
    pub inner: hal::Timer<TIMER0, OneShot>
}

use hal::prelude::{_embedded_hal_blocking_delay_DelayMs,
    _embedded_hal_blocking_delay_DelayUs};

impl ButtonBlocker  {
    pub fn wait(&mut self, duration: TimeDuration)   {
        //defmt::trace!("blocking for {:?} ...", duration);

        const SEC_AS_MILI: u16 = 1000;
        match duration {
            TimeDuration::Micro(micro) => self.inner.delay_us(micro),
            TimeDuration::Mili(mili) => self.inner.delay_ms(mili),
            TimeDuration::Sec(sec) => {
                for _i in 0..sec {self.inner.delay_ms(SEC_AS_MILI);}
            },
        };
    }
}

pub enum TimeDuration {
    Micro(u32),
    Mili(u32),
    Sec(u16),
}