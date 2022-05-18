#![no_main]
#![no_std]

use rtic::app;
use panic_rtt_target as _;


#[app(device = board, peripherals = false, dispatchers = [SWI0_EGU0,
                                                        SWI1_EGU1])] 
mod app {
    use rtt_target::{rtt_init_print, rprintln};
    use board::*;

    //use systick_monotonic::{Systick, fugit};
    #[monotonic(binds = SysTick, default = true)]    
    // type MyMono = Systick<1000>;


    #[local]
    struct LocalResources {
        led: Led,
        button: Button,
        uarte: uarte_struct,
    }

    #[shared]
    struct SharedResources {
        gpiote: Gpiote,
        #[lock_free]
        counter_blink: u16,
        #[lock_free]
        counter_interrupt: u16,   
        #[lock_free]
        debounce: ButtonBlocker,

    }

    #[init]
    fn init(_ctx: init::Context) 
    -> (SharedResources, LocalResources, init::Monotonics) {
        rtt_init_print!();

        let my_board = board::init_board().unwrap();
        
        rtt_target::rprintln!("Board initialized");

        ( 
            SharedResources { gpiote: my_board.gpiote,
                counter_interrupt: 0,
                counter_blink: 0,
                debounce: my_board.blocking_timer}, 

            LocalResources {led: my_board.leds._4,
                            button: my_board.buttons._4,
                            uarte: my_board.uarte_board},
            
            init::Monotonics(),
        )
    }





    #[task(binds = GPIOTE,
        shared = [gpiote, counter_blink, counter_interrupt, debounce],
        local = [led, button, uarte])]
    fn blink_diode(mut cx: blink_diode::Context)    {
        cx.shared.gpiote.lock(|gpiote|  {
            *cx.shared.counter_interrupt += 1;
            rprintln!("Entered interrupt {}' time", cx.shared.counter_interrupt);
            gpiote.reset_events();


            cx.shared.debounce.wait(TimeDuration::Mili(70));
            if cx.local.button.is_pushed()  {

                *cx.shared.counter_blink += 1;
                cx.local.led.toggle();
                rprintln!("LED toggled {}", cx.shared.counter_blink);
            }
        });
    }


}

