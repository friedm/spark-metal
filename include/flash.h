#include <spark-metal.h>

typedef struct {
   volatile uint ACR;
   volatile uint KEYR;
   volatile uint OPTKEYR;
   volatile uint SR;
   volatile uint CR;
   volatile uint AR;
   volatile uint RESERVED;
   volatile uint OBR;
   volatile uint WRPR;
} FLASH_TypeDef;

#define FLASH_R_BASE (AHBPERIPH_BASE + 0x2000)
#define FLASH ((FLASH_TypeDef *)FLASH_R_BASE)
