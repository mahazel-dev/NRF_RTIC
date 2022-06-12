use crate::hal_main as hal;

mod lib_dma;
mod lib_gpiote;
mod lib_nfc;
mod lib_uarte;
mod lib_i2c;
mod lib_gpio;

pub use lib_dma::*;
pub use lib_gpiote::*;
pub use lib_nfc::*;
pub use lib_uarte::*;
pub use lib_i2c::*;
pub use lib_gpio::*;

use hal::pac::{TIMER1, TIMER2, TIMER3};
pub use hal::pac::{interrupt, Interrupt, NVIC_PRIO_BITS, 
    TIMER0,
};

pub use hal::{
    clocks, Clocks,
    Timer, timer::OneShot,};



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
        let pins_0 = gpio::p0::Parts::new(periph.P0);
        let pins_1 = gpio::p1::Parts::new(periph.P1);
        
        // LED config
        let led_1 = pins_0.p0_13.degrade().into_push_pull_output(gpio::Level::High);
        let led_2 = pins_0.p0_14.degrade().into_push_pull_output(gpio::Level::High);
        let led_3 = pins_0.p0_15.degrade().into_push_pull_output(gpio::Level::High);
        let led_4 = pins_0.p0_16.degrade().into_push_pull_output(gpio::Level::High);
        
        // General buttons config
        let button_1 = pins_0.p0_11.degrade().into_pullup_input();
        let button_2 = pins_0.p0_12.degrade().into_pullup_input();
        let button_3 = pins_0.p0_24.degrade().into_pullup_input();
        let button_4 = pins_0.p0_25.degrade().into_pullup_input();
        
        // ********** GPIOTE Configuration **********
        let board_gpiote = Gpiote::new(periph.GPIOTE);
        // Interuppter button
        board_gpiote.port().input_pin(&button_1).low();
        board_gpiote.port().input_pin(&button_2).low();
        board_gpiote.port().input_pin(&button_3).low();
        board_gpiote.port().input_pin(&button_4).low();
        board_gpiote.port().enable_interrupt();


        board_gpiote.channel0().input_pin(&pins_0.p0_07.degrade().into_floating_input()).hi_to_lo();

        // Blocker for button - to delete, LEARN Monotonics
        //let blocker = hal::Timer::one_shot(periph.TIMER0);

        // ********** UARTE configuration **********
        // UARTE unwrap and basic configure
        let board_uarte = Uarte::new(periph.UARTE0,
            uarte::Pins {
                rxd: pins_0.p0_08.degrade().into_floating_input(),
                txd: pins_0.p0_06.degrade().into_push_pull_output(Level::High),
                cts: None, //Some(pins_0.p0_07.degrade().into_floating_input()),
                rts: None, //Some(pins_0.p0_05.degrade().into_push_pull_output(Level::High)),
            },
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );

        
        // ********** I2C Master configuration **********
        let _board_i2c = Twim::new(periph.TWIM0,
            twim::Pins {
                scl: pins_1.p1_01.degrade().into_floating_input(),
                sda: pins_1.p1_02.degrade().into_floating_input(),
            },
            twim::Frequency::K400,
            );


         // ********** New DMA BUFFOR ****************
        let board_dma = DmaBuffor::new();
        unsafe {board_dma.ptr.uarte_tx.write([0x0A, 0x31, 0x32, 0x33]); }

        // ********** NFCT configuration Configuration **********
        let board_nfct = Nfct::new(periph.NFCT);

        let board_timers = Timers {
            tim0: Timer::new(periph.TIMER0),
            _tim1: None,
            _tim2: None,
            _tim3: None,
        };

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


            board_gpiote: board_gpiote,

            board_nfct: board_nfct,

            board_uarte: board_uarte,

            board_dma: board_dma,

            board_timers: board_timers,

        })
        
    } else  {
        Err(())
    }
}

/* */


pub struct Device {
    /// Add LEDs to my board
    pub leds: Leds,
    /// Add Buttons to my board
    pub buttons: Buttons,
    /// Add timer for general delay
    //pub blocking_timer: ButtonBlocker,
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
    // Timers Handler
    pub board_timers: Timers,

}


pub struct Timers  {
    pub tim0: Timer<TIMER0>,
    _tim1: Option<TIMER1>,
    _tim2: Option<TIMER2>,
    _tim3: Option<TIMER3>,
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