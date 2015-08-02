pub const USB_BASE: usize = 0x40005C00;

pub struct Usb {
    usb_base: usize
}

impl Usb {
    pub fn from(usb_base: usize) -> Usb {
        Usb {
            usb_base: usb_base
        }
    }
}
