#![no_std]
use nrf52840_hal as hal_main;
pub use defmt_rtt as _;

pub mod device;
pub use device::*;


