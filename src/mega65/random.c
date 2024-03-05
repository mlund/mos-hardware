#include <stdint.h>
#include <stdio.h>

/** Read random byte from MEGA65 hardware random number generator */
__attribute__((leaf)) uint8_t mega65_hardware_rand_c(void) {
  uint8_t random_byte;
  asm volatile("RNG_DATA = $d7ef \n" // hardware random number generator
               "RNG_BUSY = $d7fe \n" // M7 is set if RNG is busy
               "1: bit RNG_BUSY  \n" // N = M7, V = M6. Numeric labels are local
               "   bmi 1b        \n" // loop backwards "b" if N is set
               "   ld%0 RNG_DATA \n" // load into A, X, or Y
               : "=R"(random_byte)   // write-only output
               :                     // no input
               : "v"                 // clobbers; due to `bit` opcode
  );
  return random_byte;
}