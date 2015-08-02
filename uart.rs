use core::str::StrExt;

use util::do_nothing;

pub const UART_1_BASE: usize = 0x40013800;
pub const UART_2_BASE: usize = 0x40004400;
pub const UART_3_BASE: usize = 0x40004800;

///Reads and writes from the uart device at the specified memory location.
pub struct Uart {
    pub uart_base: usize
}

impl Uart {
    pub fn from(uart_base: usize) -> Uart {
        Uart {
            uart_base: uart_base
        }
    }

    pub fn putc(&self, c: u8) {
        let mut buffer_ptr = self.uart_base as *mut u8;
        let lsr_ptr = unsafe {buffer_ptr.offset(5)};

        unsafe {
            while *lsr_ptr & 0b01000000 == 0 { do_nothing(); }//wait for buffer to empty
            *buffer_ptr = c;//write to buffer
        }
    }

    pub fn getc(&self) -> u8 {
        let mut buffer_ptr = self.uart_base as *mut u8;
        let lsr_ptr = unsafe {buffer_ptr.offset(5)};

        unsafe {
            while *lsr_ptr & 0b1 == 0 { do_nothing(); }
            if *lsr_ptr & 0b10001110 == 0 {//no err
                return *buffer_ptr;
            }
        }
        0
    }

    pub fn checkc(&self) -> bool {
        let mut lsr_ptr = unsafe {(self.uart_base as *mut u8).offset(5)};

        unsafe { *lsr_ptr & 0b1 != 0 }
    }

    pub fn print(&self, s: &str) {
        for byte in s.as_bytes() {
            self.putc(*byte);
        }
    }
}
