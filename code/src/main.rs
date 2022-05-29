#![no_main]
#![no_std]

use rtic::app;
use panic_rtt_target as _;


#[app(device = board, peripherals = false, dispatchers = [SWI0_EGU0,
                                                        SWI1_EGU1])] 
mod app {
    use board::*;
    use systick_monotonic::*;
    use rtt_target::rtt_init_print;

    
    #[monotonic(binds = SysTick, default = true)]    
    type MyMono = Systick<10>;

    #[local]
    struct LocalResources {
        buttons: Buttons,
        system_on: bool,
        nfct: Nfct,
    }

    #[shared]
    struct SharedResources {
        #[lock_free]
        leds: Leds,
        #[lock_free]
        gpiote: Gpiote,
        #[lock_free]
        uart: Uart,
    }

    #[init]
    fn init(_ctx: init::Context) 
    -> (SharedResources, LocalResources, init::Monotonics) {
        rtt_init_print!();
        let my_board = board::init_board().unwrap();
        rtt_target::rprintln!("Board initialized\n----------");
        

        let clk = _ctx.core.SYST;
        let mono = Systick::new(clk, 64_000_000);

        let leds = my_board.leds;
        let buttons = my_board.buttons;
        let uart = my_board.board_uart;
        //let dmaBuffor = my_board.uarte_buffor;

        let system_on = true;
        system_on::spawn_after(1.secs()).unwrap();

        ( 
            SharedResources {
                gpiote: my_board.board_gpiote,
                leds: leds,
                uart: uart, 
            },
            
            LocalResources  {
                system_on,
                buttons: buttons,
                nfct: my_board.board_nfct,
                //uarte: my_board.uarte_board,
            },
            
            init::Monotonics(mono),
        )
    }

    #[task(local = [buttons,
        ],
        shared = [leds,
        uart,
        ])]
    fn debounce(cx: debounce::Context)  {
        let buttons = cx.local.buttons;
        let leds = cx.shared.leds;
        
        //cx.shared.uart.write_byte(0x43);
        //let uarte = cx.local.uarte;
              
        let mut msg: &str = "\nEntered GPIO";
        cx.shared.uart.write_str(msg);
  
        if buttons._1.is_pushed() { leds._1.toggle();
            msg = "\nLED1 toggled"

        }
        else if buttons._2.is_pushed() { leds._2.toggle(); 
            msg = "\nLED2 toggled"}
        else if buttons._3.is_pushed() { leds._3.toggle(); 
            msg = "\nLED3 toggled"}

        cx.shared.uart.write_str(msg);
        //cx.shared.uart.write_frame([0x38, 0x38, 0x38, 0x38, 0x38, 0x38]);
    }

    #[task(binds = GPIOTE, shared = [gpiote])]
    fn GPIOTE_interrupt(cx: GPIOTE_interrupt::Context)  {
        debounce::spawn().unwrap();
        cx.shared.gpiote.reset_events();
    }

    #[task(shared = [leds])]
    fn system_diode(cx: system_diode::Context)  {
        cx.shared.leds._4.toggle();
        system_on::spawn().unwrap();
    }


    #[task(local = [system_on])]
    fn system_on(cx: system_on::Context)    {
        if *cx.local.system_on  {
            system_diode::spawn_after(500.millis()).ok();
        }
    }

    #[task(binds = NFCT, local = [nfct])]
    fn nfc(cx: nfc::Context)   {
        let nfc = cx.local.nfct;
        if nfc.field_detected()  {
            debounce::spawn().unwrap();
        }
    }

    #[task(binds = UARTE0_UART0, shared = [uart])]
    fn uart_read(cx: uart_read::Context)    {
        let byte = cx.shared.uart.read();
        cx.shared.uart.write_str("\nentered interrupt");
        match byte  {
            Some(byte)  => cx.shared.uart.write_byte(byte),
            None => cx.shared.uart.write_str("\nSomething went wrong"),
        }
    }
}


