pub static UARTE_TX_BUF_DEF: u32 = 0x2000_0000;
pub const UARTE_TX_BUF_MAXLEN: u16 = 4;

pub static UARTE_RX_BUF_DEF: u32 = (0x2000_0040 + 0x01 * UARTE_RX_BUF_MAXLEN as u32);
pub const UARTE_RX_BUF_MAXLEN: u8 = 8;


use volatile_register::*;

#[repr(C)]
pub struct DmaBufforBlock   {
    pub uarte_tx: RW<[u8; UARTE_TX_BUF_MAXLEN as usize]>,
    //pub uarte_rx: RW<[u8; UARTE_RX_BUF_MAXLEN as usize]>,
}

pub struct DmaBuffor    {
    pub ptr: &'static mut DmaBufforBlock
}

impl DmaBuffor  {
    pub fn new() -> Self    {
        DmaBuffor {
            ptr: unsafe {&mut *(UARTE_TX_BUF_DEF as *mut DmaBufforBlock)},
        }
    }
}

