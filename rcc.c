#include <rcc.h>
#include <flash.h>

//configure clock for 72MHz
void ClockInit72(void) {
   volatile uint StartUpCounter = 0, HSEStatus = 0;

   RCC->CR |= ((uint)0x00010000);//turn hse on

   do {
      HSEStatus = RCC->CR & (uint)0x00020000;//check if hse is ready
      StartUpCounter++;
   } while ((HSEStatus == 0) && (StartUpCounter != (uint)0x0500));

   if ((RCC->CR & (uint)0x00020000) != 0x0400) {
      HSEStatus = (uint)0x01;
   }
   else HSEStatus = (uint)0x00;

   if (HSEStatus == (uint)0x01) {
      FLASH->ACR |= FLASH_ACR_PRFTBE;

      FLASH->ACR &= (uint)((uint)~FLASH_ACR_LATENCY);
      FLASH->ACR |= (uint)FLASH_ACR_LATENCY_2;


      RCC->CFGR |= (uint)0x00000000;//RCC_CFGR_HPRE_DIV1
      RCC->CFGR |= (uint)0x00000000;//RCC_CFGR_PPRE2_DIV1
      RCC->CFGR |= (uint)0x00000400;//RCC_CFGR_PPRE1_DIV2

      RCC->CFGR &= (uint)(~((uint)0x00010000 | (uint)0x00020000 | (uint)0x003C0000));
      RCC->CFGR |= (uint)((uint)0x00010000 | 0x001C0000); 

      RCC->CR |= (uint)0x01000000;

      while((RCC->CR & (uint)0x02000000) == 0);

      RCC->CFGR &= (uint)(~((uint)0x00000003));//RCC_CFGR_SW
      RCC->CFGR |= (uint)0x00000002;//RCC_CFGR_SW_PLL

      while ((RCC->CFGR & (uint)0x0000000C) != (uint)0x08);//RCC_CFGR_SWS
   }
   else {
   }
}

