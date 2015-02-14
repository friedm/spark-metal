#include <led.h>
#include <gpio.h>

void writeLED(uint state) {
   gpioWrite(GPIOA, LED_RED|LED_BLUE|LED_GREEN, 0);
   if (state != 0xFFFF)
      gpioWrite(GPIOA, state, 1);
}
