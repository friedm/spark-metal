#include <button.h>
#include <gpio.h>

uint buttonRead(void) {
      return !gpioRead(GPIOB, (uint)BUTTON_PIN);
}
