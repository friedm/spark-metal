pub const LED_BLUE: usize = 0x0100;
pub const LED_GREEN: usize = 0x0400;
pub const LED_RED: usize = 0x0200;
pub const LED_VIOLET: usize = LED_BLUE|LED_RED;
pub const LED_CYAN: usize = LED_BLUE|LED_GREEN;
pub const LED_YELLOW: usize = LED_RED|LED_GREEN;
pub const LED_WHITE: usize = LED_RED|LED_GREEN|LED_BLUE;
pub const LED_OFF: usize = 0xFFFF;
