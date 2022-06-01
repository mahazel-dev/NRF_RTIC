
pub static CAN_TX_BUF: u32 = 0x2000_0000;
pub const CAN_TX_BUF_MAXLEN: u16 = 4;


use volatile_register::*;

#[repr(C)]
pub struct DmaBufforBlock   {
    pub CanTx: RW<[u8; CAN_TX_BUF_MAXLEN as usize]>,
}

pub struct DmaBuffor    {
    pub ptr: &'static mut DmaBufforBlock
}

impl DmaBuffor  {
    pub fn new() -> Self    {
        DmaBuffor {
            ptr: unsafe {&mut *(CAN_TX_BUF as *mut DmaBufforBlock)},
        }
    }
}

