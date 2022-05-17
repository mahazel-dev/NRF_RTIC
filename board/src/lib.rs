#![no_std]

use hal::timer::OneShot;
use nrf52840_hal as hal;

pub use hal::{gpio, gpio::*, gpiote, gpiote::*,
    clocks, Clocks,
    Timer};

pub use hal::pac::{interrupt, Interrupt, NVIC_PRIO_BITS, 
    TIMER0};

use embedded_hal::digital::v2::
    {OutputPin as _, InputPin as _,
        StatefulOutputPin};

use hal::prelude::{_embedded_hal_blocking_delay_DelayMs,
        _embedded_hal_blocking_delay_DelayUs};

//pub use hal::ieee802154;

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

        defmt::debug!("Initializing the board's peripherals");

        // ********** GPIO Configuration ********** 
        let pins = gpio::p0::Parts::new(periph.P0);
        let gpiote = gpiote::Gpiote::new(periph.GPIOTE);
        
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
        
        // Interuppter button
        gpiote.port().input_pin(&button_4).low();
        gpiote.port().enable_interrupt();

        // Blocker
        let blocker = hal::Timer::one_shot(periph.TIMER0); 
        
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

            gpiote: gpiote,
        })
        
    } else  {
        Err(())
    }
}

pub struct Device   {
    /// Add LEDs to my board
    pub leds: Leds,
    /// Add Buttons to my board
    pub buttons: Buttons,
    /// Add timer for general delay
    pub blocking_timer: ButtonBlocker,
    // Add GPIOTE feature
    pub gpiote: Gpiote,
}


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
        
        defmt::trace!(
            "setting P{}.{} low (LED on)",

            if self.inner.port() == Port::Port1 {
                '1'
            }   else {
                '0'
            },
            self.inner.pin()
        );
        
        
        let _ = self.inner.set_low();
    }

    /// Turns off LED 
    pub fn off(&mut self)    {
        
        defmt::trace!(
            "setting P{}.{} low (LED on)",

            if self.inner.port() == Port::Port1 {
                '1'
            }   else {
                '0'
            },
            self.inner.pin()
        );
        
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