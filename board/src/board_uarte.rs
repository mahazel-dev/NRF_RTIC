pub use crate::hal::Uarte;
pub use crate::hal::uarte::*;
pub use crate::hal::pac::{uarte0, UARTE0};

use core::slice::from_raw_parts;

//use volatile_register::*;

static UARTE_TX_BUF: i32 = 0x2000_1000;
pub static UARTE_TX_BUF_DEF: &str= "\n#######";
static UARTE_RX_BUF: i32 = 0x2000_1000;


pub struct DmaUarteBuffor   {
    pub TxBlock: *mut [u8; 8],
    pub RxBlock: *mut [u8; 8],
    pub RxBlockLen: usize,
    pub TxBlockLen: usize,

}
unsafe impl Send for DmaUarteBuffor {}


impl DmaUarteBuffor {
    pub fn new()  -> DmaUarteBuffor   {
            DmaUarteBuffor { 
                TxBlock: unsafe {&mut *((UARTE_TX_BUF) as *mut [u8; 8])},
                RxBlock: unsafe {&mut *(UARTE_RX_BUF  as *mut [u8; 8])},
                TxBlockLen: 8,        RxBlockLen: 8,}
    }
}

pub trait MyUarte {
    fn write_string(&mut self, string: &str, dma: &mut DmaUarteBuffor);
    fn write_frame(&mut self, frame: [u8; 8], dma: &mut DmaUarteBuffor);
}

impl <T> MyUarte for Uarte<T> where T:Instance {
    fn write_string(&mut self, string: &str, dma: &mut DmaUarteBuffor) {
        let tx_buffor = &dma.TxBlock;
        let buf_size = str_to_ptr(string, tx_buffor);

        let frame = unsafe { from_raw_parts(*tx_buffor as *mut u8, buf_size + 1) };

        let _ = &self.write(frame).unwrap();
    }

    fn write_frame(&mut self, frame: [u8; 8], dma: &mut DmaUarteBuffor) {
        unsafe { dma.TxBlock.write(frame) };

        let frame = unsafe { from_raw_parts(dma.TxBlock as *mut u8, 8) };
        let _ = &self.write(frame).unwrap();

    }
}


 fn str_to_ptr(string: &str, ptr: &*mut [u8; 8]) -> usize {
    let msg: &str;
    let mut frame: [u8; 8] = [0x0A, 0x23, 0x23, 0x23, 0x23, 0x23, 0x23, 0x23];

    if string.len() > 7  { msg = UARTE_TX_BUF_DEF;
                unsafe { ptr.write(frame) }
                msg.len()
    } else {    msg = string ;
                let msg = msg.as_bytes();

                for i in 1..=msg.len() {
                    frame[i] = msg[i - 1];
                }
                unsafe { ptr.write(frame) };
                msg.len()
            }
        }
