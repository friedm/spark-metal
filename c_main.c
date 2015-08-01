#include <spark-metal.h>
#include <gpio.h>
#include <rcc.h>
#include <led.h>
#include <button.h>

//called by startup.S before main
void systemInit(void) {
   RCC->CR |= (uint)0x00000001;
   RCC->CFGR &= (uint)0xF8FF0000;
   RCC->CR &= (uint)0xFEF6FFFF;
   RCC->CR &= (uint)0xFFFBFFFF;
   RCC->CFGR &= (uint)0xFF80FFFF;

   ClockInit72();
}

void enablePeriph(void) {
   RCC->APB2ENR |= RCC_APB2ENR_IOAEN;
   RCC->APB2ENR |= RCC_APB2ENR_IOBEN;
   RCC->APB2ENR |= RCC_APB2ENR_AFIOEN;
}

void c_main(void) {
//   systemInit();
//   enablePeriph();

   //GPIOInit(GPIOA, LED_RED|LED_BLUE|LED_GREEN, 0x10, 3);
//   GPIOA->BSRR = 0xFFFF;

   //GPIOInit(GPIOB, (uint)BUTTON_PIN, GPIO_MODE_IPU, 2);

   seed(11110);

//   writeLED(LED_WHITE);
   while (1) {
      uint d = rand() % 6;

      if (buttonRead()) {
         switch(d) {
            case 0: writeLED(LED_RED); break;
            case 1: writeLED(LED_GREEN); break;
            case 2: writeLED(LED_BLUE); break;
            case 3: writeLED(LED_CYAN); break;
            case 4: writeLED(LED_YELLOW); break;
            case 5: writeLED(LED_VIOLET); break;
         }
         while (buttonRead()) ;
      }
      delay(5000);
   }
}
