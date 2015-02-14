#include <gpio.h>

void GPIOInit(GPIO_TypeDef * GPIOx, uint pin, uint mode, uint speed) {
   uint tmpreg = 0, pos = 0, pinpos = 0, currentpin = 0, pinmask = 0;
   uint currentmode = (((uint)mode) & ((uint)0x0F));

   if(((mode) & ((uint)0x10)) != 0x00)
      currentmode |= ((uint)speed);

   if ((((uint)pin) & ((uint)0x00FF)) != 0x00) {
      tmpreg = GPIOx->CRL;
      for (pinpos = 0x00; pinpos < 0x08; pinpos++) {
         pos = ((uint)0x01) << pinpos;

         currentpin = pin & pos;
         if (currentpin = pos) {
            pos = pinpos << 2;
            pinmask = ((uint)0x0F) << pos;
            tmpreg &= ~pinmask;
            tmpreg |= (currentmode << pos);
            if (mode == GPIO_MODE_IPD) {//
               GPIOx->BRR = ((uint)0x01) << pinpos;
            }
            else {
               if (mode == GPIO_MODE_IPU) {
                  GPIOx->BSRR = ((uint)0x01) << pinpos;
               }
            }
         }

      }
      GPIOx->CRL = tmpreg;
   }

   if (pin > 0x00FF) {
      tmpreg = GPIOx->CRH;
      for (pinpos = 0x00; pinpos < 0x08; pinpos++) {
         pos = ((uint)0x01) << (pinpos + 0x08);

         currentpin = ((uint)pin) & ((uint)pos);
         if (currentpin == pos) {
            pos = pinpos << 2;

            pinmask = ((uint)0x0F) << pos;
            tmpreg &= ~pinmask;

            tmpreg |= (currentmode << pos);

            if (mode == GPIO_MODE_IPD) {//GPIO_Mode_IPD
               GPIOx->BRR = ((uint)0x01) << (pinpos + 0x08);
            }
            if (mode == GPIO_MODE_IPU) {//GPIO_Mode_IPU
               GPIOx->BSRR = ((uint)0x01) << (pinpos + 0x08);
            }
         }
      }
      GPIOx->CRH = tmpreg;
   }
}

void gpioWrite(GPIO_TypeDef *gpio, uint pin, uint val) {
   if (val) {
      gpio->BSRR = pin << 16;
   }
   else {
      gpio->BSRR = pin;
   }
}

uint gpioRead(GPIO_TypeDef *gpio, uint pin) {
   uint val = gpio->IDR & pin;
   return val != 0;
}
