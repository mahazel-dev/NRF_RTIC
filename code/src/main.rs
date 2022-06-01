#![no_std]
#![no_main]

use rtic::app;
use panic_probe as _;
use defmt_rtt as _;

#[app(device = board, peripherals = false, dispatchers = [SWI0_EGU0,
                                                        SWI1_EGU1])] 
mod app {
    use board::*;
    use systick_monotonic::*;

    #[monotonic(binds = SysTick, default = true)]    
    type MyMono = Systick<10>;

    #[local]
    struct LocalResources {
        buttons: Buttons,
        system_on: bool,
        nfct: Nfct,
        can_protocol: CanProtocol<UARTE0>,
    }

    #[shared]
    struct SharedResources {
        #[lock_free]
        leds: Leds,
        #[lock_free]
        gpiote: Gpiote,
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

        let can_protocol = my_board.board_can;

        defmt::info!("Peripherials turned on\n----------");

        let system_on = true;
        system_on::spawn_after(1.secs()).unwrap();

        ( 
            SharedResources {
                gpiote: my_board.board_gpiote,
                leds: leds,
            },
            LocalResources  {
                system_on,
                buttons: buttons,
                nfct: my_board.board_nfct,
                can_protocol,
                //uarte: my_board.uarte_board,
            },
            init::Monotonics(mono),
        )
    }

    // Task to indicate that uC is working
    #[task(shared = [leds])]
    fn system_diode(cx: system_diode::Context)  {
        cx.shared.leds._4.toggle();
        system_on::spawn().unwrap();
    }
    #[task(local = [system_on])]
    fn system_on(cx: system_on::Context)    {
        defmt::trace!("system_on_function");
        if *cx.local.system_on  {
            system_diode::spawn_after(500.millis()).ok();
        }
    }



    // Interrupt handler for GPIOTE
    #[task(binds = GPIOTE, shared = [gpiote])]
    fn GPIOTE_interrupt(cx: GPIOTE_interrupt::Context)  {
        // If button was pushed
        if cx.shared.gpiote.port().is_event_triggered() {
            debounce::spawn().unwrap();
        }
        cx.shared.gpiote.reset_events();
    }

    // Task for GPIOTE service
    #[task(local = [buttons,
        can_protocol,
        ],
        shared = [leds,
        ])]
    fn debounce(cx: debounce::Context)  {
        // Map resources
        let buttons = cx.local.buttons;
        let leds = cx.shared.leds;
        // Add condition for each port event
        if buttons._1.is_pushed() { leds._1.toggle();
            defmt::info!("button1 pushed");
            cx.local.can_protocol.transmit(0x2000_0000, 4).unwrap();
        }
            //cx.shared.uart.read_command();}
        else if buttons._2.is_pushed() { leds._2.toggle();
            defmt::info!("button2 pushed");}
        else if buttons._3.is_pushed() { leds._3.toggle();}
        else if buttons._4.is_pushed() { leds._3.toggle();}
    }


    // Interrupt handler for NFCT
    #[task(binds = NFCT, local = [nfct])]
    fn nfc(cx: nfc::Context)   {
        let nfc = cx.local.nfct;
        if nfc.field_detected()  {
            defmt::info!("Field detect interrupt entered:");
        }
        nfc.reset_events();
    }

    // Interrupt handler for uart reception
    //#[task(binds = UARTE0_UART0, shared = [uart])]

}


