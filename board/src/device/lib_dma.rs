use volatile_register::*;

pub static RAM: u32 = 0x2000_0000;

pub static UARTE_TX_BUF_DEF: u32 = RAM;
pub const UARTE_TX_BUF_MAXLEN: u16 = 4;

pub static UARTE_RX_BUF_DEF: u32 = 0x2000_0040 + 0x01 * UARTE_TX_BUF_MAXLEN as u32;
pub const UARTE_RX_BUF_MAXLEN: u8 = 8;

pub static I2C_DATA_BUF: u32 = UARTE_RX_BUF_DEF + 0x01 * UARTE_RX_BUF_MAXLEN as u32;
pub const I2C_DATA_BUF_LEN: u32 = 512;


#[repr(C)]
pub struct DmaBufforBlock   {
    pub uarte_tx: RW<[u8; UARTE_TX_BUF_MAXLEN as usize]>,
    pub uarte_rx: RW<[u8; UARTE_RX_BUF_MAXLEN as usize]>,
    pub i2c: RW<[u8; I2C_DATA_BUF_LEN as usize]>,
}

pub struct DmaBuffor    {
    pub ptr: &'static mut DmaBufforBlock
}

impl DmaBuffor  {
    pub fn new() -> Self    {
        DmaBuffor {
            ptr: unsafe {&mut *(RAM as *mut DmaBufforBlock)},
        }
    }
}

