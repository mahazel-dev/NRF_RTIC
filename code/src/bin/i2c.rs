#![no_std]
#![no_main]

use rtic::app;
use panic_probe as _;
use defmt_rtt as _;


#[app(device = board, peripherals = false, dispatchers = [SWI0_EGU0,
                                                        SWI1_EGU1])] 
mod app {
    use board::{*, UARTE_RX_BUF_DEF, UARTE_RX_BUF_MAXLEN};
    use systick_monotonic::*;

    #[monotonic(binds = SysTick, default = true)]    
    type MyMono = Systick<10>;

    #[local]
    struct LocalResources {
    }

    #[shared]
    struct SharedResources {
    }

    #[init]
    fn init(_ctx: init::Context) 
    -> (SharedResources, LocalResources, init::Monotonics) {
        let my_board = board::init_board().unwrap();
        defmt::info!("Board initialized\n----------");

        let clk = _ctx.core.SYST;
        let mono = Systick::new(clk, 64_000_000);

        let leds = my_board.leds;
        let buttons = my_board.buttons;

        let uarte = my_board.board_uarte;

        defmt::info!("Peripherials turned on\n----------");

        ( 
            SharedResources {
            },
            LocalResources  {
            },
            init::Monotonics(mono),
        )
    }

}


