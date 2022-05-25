pub use crate::hal::Uarte;
pub use crate::hal::uarte::*;
pub use crate::hal::pac::{uarte0, UARTE0};

use volatile_register::*;

static mut UARTE_TX_BUF: i32 = (0x2000_0000 + 0x50);
pub static UARTE_TX_BUF_DEF: &str= "\n#######";//[0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38];

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
                RxBlock: unsafe {&mut *((0x2000_0000 + 0x100)  as *mut [u8; 8])},
                TxBlockLen: 8,        RxBlockLen: 8,}
    }
}

/* 
pub struct DmaUarteBuffor   {
    pub TxBlock: *mut [u8; 8],
    pub RxBlock: *mut [u8; 8],
    pub RxBlockLen: usize,
    pub TxBlockLen: usize,

}
unsafe impl Send for DmaUarteBuffor {}  

impl DmaUarteBuffor {
    pub fn new()  -> DmaUarteBuffor   {
            DmaUarteBuffor { TxBlock: unsafe {&mut *((0x2000_0000 + 0x40) as *mut [u8; 8])},
                RxBlock: unsafe {&mut *((0x2000_0000 + 0x80)  as *mut [u8; 8])},
            TxBlockLen: 8,        RxBlockLen: 8,}
    }
}

*/



pub fn str_to_ptr(string: &str, ptr: &*mut [u8; 8]) {
    let msg: &str;

    let mut frame: [u8; 8] = [0x0A, 0x23, 0x23, 0x23, 0x23, 0x23, 0x23, 0x23,];

    if string.len() > 7  { msg = UARTE_TX_BUF_DEF;}
    else { msg = string };
    let msg = msg.as_bytes();

    for i in 1..=msg.len() {
        frame[8 - i] = msg[msg.len() - i];
    }

    
    unsafe { ptr.write(frame) } ; 
}