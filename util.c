#include <util.h>

uint w;
uint z;

void seed(uint a) {
   w = a;
   z = (a % 11) * 99;
}

uint rand(void) {
   z = 36969 * (z & 65535) + (z >> 16);
   w = 18000 * (w & 65535) + (w >> 16);
   return (z << 16) + w;
}

void delay(uint cycles) {
   while(cycles > 0) {
      cycles--;
   }
}
