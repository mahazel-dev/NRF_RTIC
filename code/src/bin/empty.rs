#![no_std]
#![no_main]

use rtic::app;
use panic_probe as _;
use defmt_rtt as _;


#[app(device = board, peripherals = false, dispatchers = [SWI0_EGU0,
                                                        SWI1_EGU1])] 
mod app {
    use board::*;

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


        defmt::info!("Peripherials turned on\n----------");

        ( 
            SharedResources {
            },
            LocalResources  {
            },
            init::Monotonics(),
        )
    }

    #[task()]
    fn example(cx: example::Context)  {
    }

}


