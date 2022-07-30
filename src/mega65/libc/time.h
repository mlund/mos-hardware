/*
  The MEGA65 really only has a Real-Time Clock (RTC), so we just have handy functions for that.

  The RTC varies between revisions of the MEGA65, though, so we have to take that into account.
*/

#ifndef _TIME_H_
#define _TIME_H_

struct m65_tm {
  unsigned char tm_sec;   /* Seconds (0-60) */
  unsigned char tm_min;   /* Minutes (0-59) */
  unsigned char tm_hour;  /* Hours (0-23) */
  unsigned char tm_mday;  /* Day of the month (1-31) */
  unsigned char tm_mon;   /* Month (0-11) */
  unsigned short tm_year; /* Year - 1900 (in practice, never < 2000) */
  unsigned char tm_wday;  /* Day of the week (0-6, Sunday = 0) */
  int tm_yday;            /* Day in the year (0-365, 1 Jan = 0) */
  unsigned char tm_isdst; /* Daylight saving time */
};

void getrtc(struct m65_tm* tm);
void setrtc(struct m65_tm* tm);

#endif
