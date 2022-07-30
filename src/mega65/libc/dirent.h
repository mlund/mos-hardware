#include <stdint.h>

unsigned char opendir(void);
struct m65_dirent *readdir(unsigned char);
void closedir(unsigned char);

struct m65_dirent {
  uint32_t d_ino;
  uint16_t d_off;
  uint32_t d_reclen;
  uint16_t d_type;
  char d_name[256];
};

