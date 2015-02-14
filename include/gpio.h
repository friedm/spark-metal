#include <spark-metal.h>

typedef struct {
   volatile uint CRL;
   volatile uint CRH;
   volatile uint IDR;
   volatile uint ODR;
   volatile uint BSRR;
   volatile uint BRR;
   volatile uint LCKR; 
} GPIO_TypeDef;

#define APB2PERIPH_BASE (uint)(PERIPH_BASE + 0x10000)
#define GPIOA ((GPIO_TypeDef *)(APB2PERIPH_BASE + 0x0800))
#define GPIOB ((GPIO_TypeDef *)(APB2PERIPH_BASE + 0x0C00))
#define GPIOC ((GPIO_TypeDef *)(APB2PERIPH_BASE + 0x1000))
#define GPIOD ((GPIO_TypeDef *)(APB2PERIPH_BASE + 0x1400))
#define GPIOE ((GPIO_TypeDef *)(APB2PERIPH_BASE + 0x1800))
#define GPIOF ((GPIO_TypeDef *)(APB2PERIPH_BASE + 0x1C00))
#define GPIOG ((GPIO_TypeDef *)(APB2PERIPH_BASE + 0x2000))

#define GPIO_MODE_IPD (uint)0x28
#define GPIO_MODE_IPU (uint)0x48
#define GPIO_MODE_IN_FLOATING (uint)0x04
#define GPIO_MODE_AIN (uint)0x0;
#define GPIO_MODE_OUT_OD (uint)0x14;
#define GPIO_MODE_OUT_PP (uint)0x10;
#define GPIO_MODE_AF_OD (uint)0x1C;
#define GPIO_MODE_AF_PP (uint)0x18;

//10MHz - 1, 2MHz - 2, 50MHz - 3

void GPIOInit(GPIO_TypeDef * GPIOx, uint pin, uint mode, uint speed);
void gpioWrite(GPIO_TypeDef *gpio, uint pin, uint val);
uint gpioRead(GPIO_TypeDef *gpio, uint pin);
