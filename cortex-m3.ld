__Stack_Size = 1024;

PROVIDE(_Stack_Size = __Stack_Size);
__Stack_Init = _estack - __Stack_Size;

PROVIDE(_Stack_Init = __Stack_Init);

_Minimum_Stack_Size = 0x100;

MEMORY
{
   RAM (xrw)      : ORIGIN = 0x20000000, LENGTH = 20K
   FLASH (rx)     : ORIGIN = 0x08000000, LENGTH = 128K
   FLASHB1 (rx)   : ORIGIN = 0x00000000, LENGTH = 0
   EXTMEMB0 (rx)  : ORIGIN = 0x00000000, LENGTH = 0
   EXTMEMB1 (rx)  : ORIGIN = 0x00000000, LENGTH = 0
   EXTMEMB2 (rx)  : ORIGIN = 0x00000000, LENGTH = 0
   EXTMEMB3 (rx)  : ORIGIN = 0x00000000, LENGTH = 0
}

_estack = 0x20005000;

SECTIONS
{
   .isr_vector :
   {
      . = ALIGN(4);
      KEEP(*(.isr_vector))
      . = ALIGN(4);
   } >FLASH

   .flashtext :
   {
      . = ALIGN(4);
      *(.flashtext)
      . = ALIGN(4);
   } >FLASH

   .text :
   {
      . = ALIGN(4);

      *(.text)
      *(.text.*)
      *(.rodata)
      *(.rodata*)
      *(.glue_7)
      *(.glue_7t)

      . = ALIGN(4);

      _etext = . ;
      _sidata = _etext;
   } >FLASH

   .data : AT (_sidata)
   {
      . = ALIGN(4);
      _sdata = . ;
      
      *(.data)
      *(.data.*)

      . = ALIGN(4);

      _edata = . ;
   } >RAM

   .bss :
   {
      . = ALIGN(4);
      _sbss = . ;

      *(.bss)
      *(COMMON)

      . = ALIGN(4);
      _ebss = . ;
   } >RAM

   PROVIDE (end = _ebss);
   PROVIDE (_end = _ebss);

   ._usrstack : 
   {
      . = ALIGN(4);
      _susrstack = . ;

      . = . + _Minimum_Stack_Size;

      . = ALIGN(4);
      _eusrstack = . ;
   } >RAM
}
