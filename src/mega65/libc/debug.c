#include "memory.h"
#include "debug.h"

unsigned char the_char;
void debug_msg(char* m)
{
  // Write debug message to serial monitor
  while (*m) {
    the_char = *m;
    asm volatile("lda the_char\n"
                 "sta $d643\n"
                 "nop" ::: "a");
    m++;
  }
  asm volatile("lda 0x0d\n"
               "sta $d643\n"
               "nop\n"
               "lda 0x0a\n"
               "sta $d643\n"
               "nop" ::: "a");
};
