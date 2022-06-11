use crate::hal_main as hal;
pub use hal::{gpio, gpio::*};

use embedded_hal::digital::v2::
    {OutputPin as _, InputPin as _,
        StatefulOutputPin};


     
        
pub struct Leds {
    // LED1: pin P0.13, green
    pub _1: Led,
    // LED2: pin P0.14, green
    pub _2: Led,
    // LED3: pin P0.15, green
    pub _3: Led,
    // LED4: pin P0.16, green
    pub _4: Led,
}

pub struct Led  {
    pub inner: Pin<Output<PushPull>>
}

impl Led    {
    /// Turns on LED 
    pub fn on(&mut self)    {
        let _ = self.inner.set_low();
    }

    /// Turns off LED 
    pub fn off(&mut self)    {    
        let _ = self.inner.set_high();
    }

    pub fn toggle(&mut self)    {
        if self.is_on() {
            let _ = self.off();
        } else {
            let _ = self.on();
        }
    }

    /// Returns `true` if the LED is in the OFF state
    pub fn is_off(&self) -> bool {
        self.inner.is_set_high() == Ok(true)
    }
    
    /// Returns `true` if the LED is in the ON state
    pub fn is_on(&self) -> bool {
         !self.is_off()
    }
}

pub struct Buttons {
        // Button1: pin P0.11, green
        pub _1: Button,
        // Button2: pin P0.12, green
        pub _2: Button,
        // Button3: pin P0.24, green
        pub _3: Button,
        // Button4: pin P0.25, green
        pub _4: Button,
}

pub struct Button   {
    pub inner: Pin<Input<PullUp>>
}

impl Button {
    pub fn is_pushed(&self) -> bool   {
        self.inner.is_high() != Ok(true)
    }
}