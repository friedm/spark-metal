#include <spark-metal.h>
#include <gpio.h>

#define UART_BASE (uint)(APB2PERIPH_BASE + 0x3800)

void uartWrite(char *);
char uartReadChar();
char * uartReadLine();
