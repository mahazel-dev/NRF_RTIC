#![no_main]
#![no_std]

use rtic::app;
use panic_rtt_target as _;

use systick_monotonic::{Systick, fugit::Duration};

#[app(device = board, peripherals = false, dispatchers = [RTC1])] 
mod app {
    use rtt_target::{rtt_init_print};
    use rtt_target::rprintln;
    use board::*;

    /*
    To be removed
    use nrf52840_hal as hal;
    use hal::gpiote::Gpiote; from cargo board
    */

    #[monotonic(binds = SysTick, default = true)]    
    //type MyMono = Systick<100>;

    #[local]
    struct LocalResources {
        led: Led,
    }

    #[shared]
    struct SharedResources {
        gpiote: Gpiote,
        #[lock_free]
        counter_blink: u16,
    }

    #[init]
    fn init(_ctx: init::Context) 
    -> (SharedResources, LocalResources, init::Monotonics) {
        rtt_init_print!();

        let my_board = board::init_board().unwrap();
        let led = my_board.leds._2;
        let gpiote = my_board.gpiote;
        
        //rtt_target::rprintln!("dupa");

        ( 
            SharedResources { gpiote: gpiote,
            counter_blink: 0 }, 
            LocalResources {led: led}, 
            init::Monotonics()
        )
    }

    #[task(local = [led],
        shared = [counter_blink])]
    fn task1(cx: task1::Context)   {
        cx.local.led.toggle();
        //let value = cx.shared.counter_blink.ta
        rprintln!("LED toggled {}", cx.shared.counter_blink);
    }


    #[task(binds = GPIOTE,
        shared = [gpiote, counter_blink])]
    fn inter(mut cx: inter::Context)    {
        cx.shared.gpiote.lock(|gpiote|  {
            gpiote.reset_events();
            
            task1::spawn().unwrap();
            *cx.shared.counter_blink += 1;
            rprintln!("Entered interrupt {}' time", cx.shared.counter_blink);
            /*
            cx.shared.counter_blink.lock(|counter_blink| {
                *counter_blink += 1;
            });
            */
        });
    }


}

