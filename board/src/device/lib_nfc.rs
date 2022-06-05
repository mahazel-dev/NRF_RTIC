use crate::hal_main::pac::{ NFCT as NFC, nfct, nfct::*};

pub struct Nfct(NFC);

impl Nfct   {
    pub fn new(periph: NFC)  -> Self   {
        // Enable NFC sense field mode
        periph.tasks_sense.write(|p| unsafe {p.bits(1)});
        // Turning on interrupt SENSE event
        periph.inten.write(|p| unsafe { p.bits(2)});

        let u = Nfct(periph);

        u
    }

    pub fn field_detected(&mut self)   -> bool {
        self.0.events_fielddetected.read().events_fielddetected().bit()
    }

    pub fn reset_events(&mut self)  {
        self.0.events_fielddetected.reset();
    }
}