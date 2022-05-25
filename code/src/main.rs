#![no_main]
#![no_std]

use rtic::app;
use panic_rtt_target as _;


#[app(device = board, peripherals = false, dispatchers = [SWI0_EGU0,
                                                        SWI1_EGU1])] 
mod app {
    use board::*;
    use systick_monotonic::*;
    use rtt_target::{rtt_init_print, rprintln};

    
    #[monotonic(binds = SysTick, default = true)]    
    type MyMono = Systick<10>;

    #[local]
    struct LocalResources {
        buttons: Buttons,
        system_on: bool,
        uarte: Uarte<UARTE0>,
    }

    #[shared]
    struct SharedResources {
        #[lock_free]
        leds: Leds,
        #[lock_free]
        gpiote: Gpiote,
        #[lock_free]
        uarte_buffor: DmaUarteBuffor,
        // tx_status: TxBuffor,
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
        let system_on = true;
        system_on::spawn_after(1.secs()).unwrap();

        let dmaBuffor = my_board.uarte_buffor;

        ( 
            SharedResources {
                gpiote: my_board.gpiote,
                //tx_status: my_board.tx_buffor,
                leds: leds,
                uarte_buffor: dmaBuffor}, 

            LocalResources  {
                system_on,
                buttons: buttons,
                uarte: my_board.uarte_board,
            },
            
            init::Monotonics(mono),
        )
    }

    #[task(local = [buttons,
            uarte,],
        shared = [leds,
        uarte_buffor])]

    fn debounce(cx: debounce::Context)  {
        let buttons = cx.local.buttons;
        let leds = cx.shared.leds;

        let uarte = cx.local.uarte;        
        let uarte_tx = cx.shared.uarte_buffor.TxBlock;

        let mut msg = "GPIOTE";
        if buttons._1.is_pushed() { leds._1.toggle();
            msg = "LED1_T"}
        else if buttons._2.is_pushed() { leds._2.toggle(); 
            msg = "LED2_T"}
        else if buttons._3.is_pushed() { leds._3.toggle(); 
            msg = "LED3_T"}
        

       
        str_to_ptr(msg, &uarte_tx);
        let frame = unsafe { slice::from_raw_parts(uarte_tx as *mut u8, 8) };
        
        uarte.write(frame).unwrap();
        
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


}


