#include <stdio.h>
#include "memory.h"
#include "targets.h"

unsigned char detect_target(void)
{
  // We use the different I2C device blocks to identify the various hardware targets

  return lpeek(0xffd3629);

  return TARGET_UNKNOWN;
}
