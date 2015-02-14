#include <spark-metal.h>

//RGB LED states
#define LED_BLUE (uint)0x0100
#define LED_GREEN (uint)0x0400
#define LED_RED (uint)0x0200
#define LED_VIOLET LED_BLUE|LED_RED
#define LED_CYAN LED_BLUE|LED_GREEN
#define LED_YELLOW LED_RED|LED_GREEN
#define LED_WHITE LED_RED|LED_GREEN|LED_BLUE
#define LED_OFF (uint)0xFFFF

void writeLED(uint color);
