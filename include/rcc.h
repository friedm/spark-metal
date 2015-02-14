#include <spark-metal.h>

#define RCC_APB2ENR_AFIOEN ((uint)0x00000001)
#define RCC_APB2ENR_IOAEN ((uint)0x00000004)
#define RCC_APB2ENR_IOBEN ((uint)0x00000008)

typedef struct {
   volatile uint CR;
   volatile uint CFGR;
   volatile uint CIR;
   volatile uint APB2RSTR;
   volatile uint APB1RSTR;
   volatile uint AHBENR;
   volatile uint APB2ENR;
   volatile uint APB1ENR;
   volatile uint BDCR;
   volatile uint CSR;
} RCC_TypeDef;

#define RCC_BASE (AHBPERIPH_BASE + 0x1000)
#define RCC ((RCC_TypeDef *)RCC_BASE)

#define FLASH_ACR_PRFTBE ((uint)0x10)
#define FLASH_ACR_LATENCY ((uint)0x03)
#define FLASH_ACR_LATENCY_2 ((uint)0x02)

void clkInit(void);
