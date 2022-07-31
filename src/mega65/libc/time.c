/*
  The MEGA65 really only has a Real-Time Clock (RTC), so we just have handy functions for that.

  The RTC varies between revisions of the MEGA65, though, so we have to take that into account.
*/

#include "memory.h"
#include "time.h"
#include "targets.h"
#include "hal.h"

#define I2CDELAY 5000L

unsigned char bcd_work;

/*
  While the double dabble algorithm is more time efficient, we mostly care about saving
  space, so use a simple loop.  Getting/setting time should not be called particularly often.
 */
unsigned char tobcd(unsigned char in)
{
  bcd_work = 0;
  while (in > 9) {
    bcd_work += 0x10;
    in -= 10;
  }
  bcd_work += in;
  return bcd_work;
}

unsigned char unbcd(unsigned char in)
{
  bcd_work = 0;
  while (in & 0xf0) {
    bcd_work += 10;
    in -= 0x10;
  }
  bcd_work += in;
  return bcd_work;
}

void getrtc(struct m65_tm* tm)
{
  if (!tm)
    return;

  tm->tm_sec = 0;
  tm->tm_min = 0;
  tm->tm_hour = 0;
  tm->tm_mday = 0;
  tm->tm_mon = 0;
  tm->tm_year = 0;
  tm->tm_wday = 0;
  tm->tm_isdst = 0;

  switch (detect_target()) {
  case TARGET_MEGA65R2:
  case TARGET_MEGA65R3:
    tm->tm_sec = unbcd(lpeek_debounced(0xffd7110));
    tm->tm_min = unbcd(lpeek_debounced(0xffd7111));
    tm->tm_hour = lpeek_debounced(0xffd7112);
    if (tm->tm_hour & 0x80) {
      tm->tm_hour = unbcd(tm->tm_hour & 0x3f);
    }
    else {
      if (tm->tm_hour & 0x20) {
        tm->tm_hour = unbcd(tm->tm_hour & 0x1f) + 12;
      }
      else {
        tm->tm_hour = unbcd(tm->tm_hour & 0x1f);
      }
    }
    tm->tm_mday = unbcd(lpeek_debounced(0xffd7113)) - 1;
    tm->tm_mon = unbcd(lpeek_debounced(0xffd7114));
    // RTC is based on 2000, not 1900
    tm->tm_year = unbcd(lpeek_debounced(0xffd7115)) + 100;
    tm->tm_wday = unbcd(lpeek_debounced(0xffd7116));
    tm->tm_isdst = lpeek_debounced(0xffd7117) & 0x20;
    break;
  case TARGET_MEGAPHONER1:
    break;
  default:
    return;
  }
}

void setrtc(struct m65_tm* tm)
{
  if (!tm)
    return;

  switch (detect_target()) {
  case TARGET_MEGA65R2:
  case TARGET_MEGA65R3:
    // Unlock RTC registers
    usleep(I2CDELAY);
    lpoke(0xffd7118, 0x41);

    usleep(I2CDELAY);
    lpoke(0xffd7110, tobcd(tm->tm_sec));
    usleep(I2CDELAY);
    lpoke(0xffd7111, tobcd(tm->tm_min));
    if (lpeek_debounced(0xffd7112) & 0x80) {
      usleep(I2CDELAY);
      lpoke(0xffd7112, tobcd(tm->tm_hour) | 0x80);
    }
    else {
      if (tm->tm_hour >= 12) {
        // PM
        usleep(I2CDELAY);
        lpoke(0xffd7112, tobcd(tm->tm_hour - 12) | 0x20);
      }
      else {
        // AM
        usleep(I2CDELAY);
        lpoke(0xffd7112, tobcd(tm->tm_hour));
      }
    }

    usleep(I2CDELAY);
    lpoke(0xffd7113, tobcd(tm->tm_mday + 1));
    usleep(I2CDELAY);
    lpoke(0xffd7114, tobcd(tm->tm_mon));
    if (tm->tm_year >= 100 && tm->tm_year <= 355) {
      usleep(I2CDELAY);
      lpoke(0xffd7115, tobcd(tm->tm_year - 100));
    }
    usleep(I2CDELAY);
    lpoke(0xffd7116, tobcd(tm->tm_wday));
    usleep(I2CDELAY);
    if (tm->tm_isdst) {
      lpoke(0xffd7117, lpeek_debounced(0xffd7117) | 0x20);
    }
    else {
      lpoke(0xffd7117, lpeek_debounced(0xffd7117) & (0xff - 0x20));
    }

    // Re-lock RTC registers
    usleep(I2CDELAY);
    lpoke(0xffd7118, 0x01);

    break;
  case TARGET_MEGAPHONER1:
    break;
  default:
    return;
  }
}
