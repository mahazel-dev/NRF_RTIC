
static UARTE_TX_BUF: i32 = 0x2000_1000;
pub static UARTE_TX_BUF_DEF: &str= "\n#######";
static UARTE_RX_BUF: i32 = 0x2000_1000;


pub struct DmaUarteBuffor   {
    pub tx_block: *mut [u8; 8],
    pub rx_block: *mut [u8; 8],
    pub tx_block_len: usize,
    pub rx_block_len: usize,

}
unsafe impl Send for DmaUarteBuffor {}


impl DmaUarteBuffor {
    pub fn new()  -> DmaUarteBuffor   {
            DmaUarteBuffor { 
                tx_block: unsafe {&mut *((UARTE_TX_BUF) as *mut [u8; 8])},
                rx_block: unsafe {&mut *(UARTE_RX_BUF  as *mut [u8; 8])},
                tx_block_len: 8,        rx_block_len: 8,}
    }
}