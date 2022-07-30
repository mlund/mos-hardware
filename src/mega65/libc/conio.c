/*  CONIO.H style Text mode support  for the Mega65 libC

    Copyright (c) 2020-2021 Hernán Di Pietro

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU Lesser General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Lesser General Public License for more details.

    You should have received a copy of the GNU Lesser General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

#include "conio.h"
#include "memory.h"
#include <string.h>

#define VIC_BASE 0xD000UL
#define IS_H640 (PEEK(VIC_BASE + 0x31) & 128)
#define IS_V400 (PEEK(VIC_BASE + 0x31) & 8)
#define SET_H640() POKE(VIC_BASE + 0x31, PEEK(VIC_BASE + 0x31) | 128)
#define CLEAR_H640() POKE(VIC_BASE + 0x31, PEEK(VIC_BASE + 0x31) & 127)
#define SET_V400() POKE(VIC_BASE + 0x31, PEEK(VIC_BASE + 0x31) | 8)
#define CLEAR_V400() POKE(VIC_BASE + 0x31, PEEK(VIC_BASE + 0x31) & 0xF7)
#define IS_16BITCHARSET (PEEK(VIC_BASE + 0x54) & 1)
#define SET_16BITCHARSET() POKE(VIC_BASE + 0x54, PEEK(VIC_BASE + 0x54) | 1)
#define CLEAR_16BITCHARSET() POKE(VIC_BASE + 0x54, PEEK(VIC_BASE + 0x54) & 0xFE)
#define SET_HOTREGS() POKE(VIC_BASE + 0x5D, PEEK(VIC_BASE + 0x5D) | 128)
#define CLEAR_HOTREGS() POKE(VIC_BASE + 0x5D, PEEK(VIC_BASE + 0x5D) & 127)
#define IS_EXTATTR() (PEEK(VIC_BASE + 0x31) & 32)
#define SET_EXTATTR() POKE(VIC_BASE + 0x31, PEEK(VIC_BASE + 0x31) | 32)
#define CLEAR_EXTATTR() POKE(VIC_BASE + 0x31, PEEK(VIC_BASE + 0x31) & 0xDF)
#define SCREEN_RAM_BASE_B0 (PEEK(VIC_BASE + 0x60)) // LSB
#define SCREEN_RAM_BASE_B1 (PEEK(VIC_BASE + 0x61))
#define SCREEN_RAM_BASE_B2 (PEEK(VIC_BASE + 0x62))
#define SCREEN_RAM_BASE_B3 (PEEK(VIC_BASE + 0x63) & 7) // upper nybble
#define SCREEN_RAM_BASE                                                                                                     \
  (((long)SCREEN_RAM_BASE_B3 << 24) | ((long)SCREEN_RAM_BASE_B2 << 16) | ((long)SCREEN_RAM_BASE_B1 << 8)                    \
      | (SCREEN_RAM_BASE_B0))
#define COLOR_RAM_BASE 0xFF80000UL

#define PRINTF_IN_FORMAT_SPEC 0x1
#define PRINTF_FLAGS_LEADINGZERO 0x2
#define PRINTF_STATE_INIT 0
#define PRINTF_STATE_ESCAPE 1

// cprintf Screen Control Escape Codes.
//
// See ESCAPE_HASH() for how to generate.
//
typedef struct tagESCAPE_CODE {
  unsigned char arg;
  void (*fn)(unsigned char);
} ESCAPE_CODE;

// use 198 bytes of C64 tape buffer as petscii2screencode conversion buffer
// in order to save bank 0 memory
static char* p2sbuf = (char*)0x334;

static ESCAPE_CODE escapeCode[255];
static unsigned char g_curTextColor = COLOUR_WHITE;
static unsigned char g_curX = 0;
static unsigned char g_curY = 0;
static unsigned char g_curScreenW = 0;
static unsigned char g_curScreenH = 0;
static const unsigned char hexDigits[] = { '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 0x41, 0x42, 0x43, 0x44, 0x45,
  0x46 };

// Drawing characters for `box` call
//
//                                      NONE, INNER   MID    OUTER    ROUND
const unsigned char chTopLeft[] = { 0x20, 0x20, 0x70, 0x4F, 0x55 };
const unsigned char chTopRight[] = { 0x20, 0x20, 0x6E, 0x50, 0x49 };
const unsigned char chBottomLeft[] = { 0x20, 0x20, 0x6D, 0x4C, 0x4A };
const unsigned char chBottomRight[] = { 0x20, 0x20, 0x7D, 0x7A, 0x4B };
const unsigned char chHorzTop[] = { 0x20, 0x64, 0x43, 0x77, 0x43 };
const unsigned char chHorzBottom[] = { 0x20, 0x63, 0x43, 0x6F, 0x43 };
const unsigned char chVertRight[] = { 0x20, 0x74, 0x5D, 0x6A, 0x5D };
const unsigned char chVertLeft[] = { 0x20, 0x6A, 0x5D, 0x74, 0x5D };

// Hash function for cprintf ESCAPE codes

static unsigned char hash(const unsigned char* str, const unsigned char maxLen)
{
  unsigned long hash = 277;
  register unsigned char c;
  register unsigned char len = 0;
  while ((c = *str++) && (len < maxLen)) {
    len++;
    hash = ((hash << 5) + hash) + c;
  }
  return hash;
}

static void clrscr_(unsigned char)
{
  clrscr();
  gohome();
} // Callable from Escape Code table
static void gohome_(unsigned char)
{
  gohome();
} // Callable from Escape Code table
static void escNOP(unsigned char)
{ /* do nothing */
}

void conioinit(void)
{
  register unsigned char i = 0;

  // Make sure we go to VIC-IV IO mode

  POKE(0xD02fL, 0x47);
  POKE(0xd02fL, 0x53);

  sethotregs(0);
  setlowercase();

  g_curScreenW = IS_H640 ? 80 : 40;
  g_curScreenH = IS_V400 ? 50 : 25;

  flushkeybuf();

  for (i = 0; i < sizeof(escapeCode) / sizeof(escapeCode[0]); ++i) {
    escapeCode[i].fn = escNOP;
    escapeCode[i].arg = 0x0;
  }

  // Setup escape codes according to it's hashed strings.
  // We know that for those codes and with k=277 there are no collisions.
  // Adding new codes should verify no collisions are added by changing k
  // or by using another algorithm.

  escapeCode[1].fn = moveleft;
  escapeCode[7].fn = moveright;
  escapeCode[10].fn = moveup;
  escapeCode[22].fn = clrscr_;
  escapeCode[30].fn = gohome_;
  escapeCode[49].fn = underline;
  escapeCode[57].fn = textcolor;
  escapeCode[57].arg = COLOUR_GREY1;
  escapeCode[58].fn = textcolor;
  escapeCode[58].arg = COLOUR_GREY2;
  escapeCode[59].fn = textcolor;
  escapeCode[59].arg = COLOUR_GREY3;
  escapeCode[64].fn = textcolor;
  escapeCode[64].arg = COLOUR_CYAN;
  escapeCode[68].fn = textcolor;
  escapeCode[68].arg = COLOUR_LIGHTBLUE;
  escapeCode[72].fn = textcolor;
  escapeCode[72].arg = COLOUR_LIGHTGREEN;
  escapeCode[96].fn = blink;
  escapeCode[96].arg = 1;
  escapeCode[139].fn = revers;
  escapeCode[140].fn = textcolor;
  escapeCode[140].arg = COLOUR_PURPLE;
  escapeCode[147].fn = underline;
  escapeCode[147].arg = 1;
  escapeCode[151].fn = textcolor;
  escapeCode[151].arg = COLOUR_BROWN;
  escapeCode[158].fn = blink;
  escapeCode[168].fn = textcolor;
  escapeCode[168].arg = COLOUR_WHITE;
  escapeCode[173].fn = revers;
  escapeCode[173].arg = 1;
  escapeCode[191].fn = textcolor;
  escapeCode[191].arg = COLOUR_YELLOW;
  escapeCode[199].fn = textcolor;
  escapeCode[199].arg = COLOUR_PINK;
  escapeCode[206].fn = textcolor;
  escapeCode[206].arg = COLOUR_BLACK;
  escapeCode[215].fn = textcolor;
  escapeCode[215].arg = COLOUR_ORANGE;
  escapeCode[216].fn = textcolor;
  escapeCode[216].arg = COLOUR_BLUE;
  escapeCode[220].fn = textcolor;
  escapeCode[220].arg = COLOUR_GREEN;
  escapeCode[240].fn = textcolor;
  escapeCode[240].arg = COLOUR_RED;
  escapeCode[249].fn = movedown;
}

char petsciitoscreencode(char c)
{
  if (c >= 64 && c <= 95) {
    return c - 64;
  }

  if (c >= 192) {
    return c - 128;
  }

  if (c >= 96 && c < 192) {
    return c - 32;
  }

  if (c == '_') {
    return 100;
  }

  return c;
}

char* petsciitoscreencode_s(char* s)
{
  char* src = s;
  char* dest = p2sbuf;
  while (*dest++ = petsciitoscreencode(*src++))
    ;
  return p2sbuf;
}

void setscreenaddr(long address)
{
  POKE(VIC_BASE + 0x60, address & 0x0000FFUL);
  POKE(VIC_BASE + 0x61, (address & 0xFF00UL) >> 8);
  POKE(VIC_BASE + 0x62, (address & 0xFF0000UL) >> 16);
  POKE(VIC_BASE + 0x63, (PEEK(VIC_BASE + 0x63) & 0xF) | ((address & 0xF000000UL) >> 24));
}

long getscreenaddr(void)
{
  return SCREEN_RAM_BASE;
}

void setcharsetaddr(long address)
{
  POKE(VIC_BASE + 0x68, address & 0x0000FFUL);
  POKE(VIC_BASE + 0x69, (address & 0xFF00UL) >> 8);
  POKE(VIC_BASE + 0x6A, (address & 0xFF0000UL) >> 16);
}

long getcharsetaddr(void)
{
  return ((long)PEEK(VIC_BASE + 0x68)) | ((long)PEEK(VIC_BASE + 0x69) << 8) | (((long)PEEK(VIC_BASE + 0x6A) << 16));
}

void setcolramoffset(unsigned int offset)
{
  POKE(VIC_BASE + 0x64, offset & 0x00FFUL);
  POKE(VIC_BASE + 0x65, (offset & 0xFF00UL) >> 8);
}

unsigned int getcolramoffset(void)
{
  return ((unsigned int)PEEK(VIC_BASE + 0x64) | ((unsigned int)PEEK(VIC_BASE + 0x65)) << 8);
}

void setscreensize(unsigned char w, unsigned char h)
{
  if (w == 80) {
    SET_H640();
    POKE(0xd04c, 0x50); // compensate for vic-iii h640 horizontal positioning bug
  }
  else if (w == 40) {
    CLEAR_H640();
    POKE(0xd04c, 0x4e);
  }

  if (h == 50)
    SET_V400();
  else if (h == 25)
    CLEAR_V400();

  // Cache values.
  if (w == 40 || w == 80)
    g_curScreenW = w;
  if (h == 25 || h == 50)
    g_curScreenH = h;
}

void getscreensize(unsigned char* width, unsigned char* height)
{
  *width = g_curScreenW;
  *height = g_curScreenH;
}

void set16bitcharmode(unsigned char f)
{
  if (f)
    SET_16BITCHARSET();
  else
    CLEAR_16BITCHARSET();
}

void sethotregs(unsigned char f)
{
  if (f)
    SET_HOTREGS();
  else
    CLEAR_HOTREGS();
}

void setextendedattrib(unsigned char f)
{
  if (f)
    SET_EXTATTR();
  else
    CLEAR_EXTATTR();
}

void setlowercase(void)
{
  setcharsetaddr(0x2d800);
}

void setuppercase(void)
{
  setcharsetaddr(0x2d000);
}

void togglecase(void)
{
  POKE(0xD018U, PEEK(0xD018U) ^ 0x02);
}

void clrscr()
{
  const unsigned int cBytes = (unsigned int)g_curScreenW * g_curScreenH * (IS_16BITCHARSET ? 2 : 1);
  lfill(SCREEN_RAM_BASE, ' ', cBytes);
  lfill(COLOR_RAM_BASE, g_curTextColor, cBytes);
}

void bordercolor(unsigned char c)
{
  POKE(VIC_BASE + 0x20, c);
}

void bgcolor(unsigned char c)
{
  POKE(VIC_BASE + 0x21, c);
}

void textcolor(unsigned char c)
{
  g_curTextColor = (g_curTextColor & 0xF0) | (c & 0xf);
}

void cellcolor(unsigned char x, unsigned char y, unsigned char c)
{
  lpoke(COLOR_RAM_BASE + (y * (unsigned int)g_curScreenW) + x, c);
}

void revers(unsigned char enable)
{
  if (enable)
    g_curTextColor |= ATTRIB_REVERSE;
  else
    g_curTextColor &= ~ATTRIB_REVERSE;
}

void highlight(unsigned char enable)
{
  if (enable)
    g_curTextColor |= ATTRIB_HIGHLIGHT;
  else
    g_curTextColor &= ~ATTRIB_HIGHLIGHT;
}

void blink(unsigned char enable)
{
  if (enable)
    g_curTextColor |= ATTRIB_BLINK;
  else
    g_curTextColor &= ~ATTRIB_BLINK;
}

void underline(unsigned char enable)
{
  if (enable)
    g_curTextColor |= ATTRIB_UNDERLINE;
  else
    g_curTextColor &= ~ATTRIB_UNDERLINE;
}

void altpal(unsigned char enable)
{
  if (enable)
    g_curTextColor |= (ATTRIB_HIGHLIGHT | ATTRIB_REVERSE);
  else
    g_curTextColor &= ~(ATTRIB_HIGHLIGHT | ATTRIB_REVERSE);
}

void clearattr(void)
{
  g_curTextColor &= 0x0F;
}

void gohome(void)
{
  gotoxy(0, 0);
}

void gotoxy(unsigned char x, unsigned char y)
{
  g_curX = x;
  g_curY = y;
}

void gotox(unsigned char x)
{
  g_curX = x;
}

void gotoy(unsigned char y)
{
  g_curY = y;
}

unsigned char wherex(void)
{
  return g_curX;
}

unsigned char wherey(void)
{
  return g_curY;
}

void cputc(unsigned char c)
{
  cputcxy(g_curX, g_curY, c);
}

void cputnc(unsigned char len, unsigned char c)
{
  cputncxy(g_curX, g_curY, len, c);
}

void moveup(unsigned char count)
{
  g_curY -= count;
}

void movedown(unsigned char count)
{
  g_curY += count;
}

void moveleft(unsigned char count)
{
  g_curX -= count;
}

void moveright(unsigned char count)
{
  g_curX += count;
}

unsigned char _cprintf(const unsigned char translateCodes, const unsigned char* fmt, ...)
{
  unsigned char printfState = PRINTF_STATE_INIT;
  unsigned char escHash = 0;
  unsigned char cch = 0;

  while (*fmt) {
    switch (printfState) {
    case PRINTF_STATE_INIT:
      switch (*fmt) {
      case '{':
        printfState = PRINTF_STATE_ESCAPE;
        break;

      case '\t': // Tab
        // moveleft((g_curX + 7) / 8) * 8, 1);
        break;

      case '\n': // New-line
        gotoxy(0, g_curY + 1);
        break;

      default:
        cputc(translateCodes ? petsciitoscreencode(*fmt) : *fmt);
      }
      break;

    case PRINTF_STATE_ESCAPE:
      if (*fmt == '{') // print literal
      {
        cputc(*fmt);
        printfState = PRINTF_STATE_INIT;
        break;
      }

      cch = 0;
      while (fmt && (*fmt != '}')) {
        fmt++;
        cch++;
      }

      if (*fmt != '}') // bailout.
        return 255;

      escHash = hash(fmt - cch, cch);
      escapeCode[escHash].fn(escapeCode[escHash].arg);
      printfState = PRINTF_STATE_INIT;
      break;
    }

    fmt++;
  }
}

void cputhex(long n, unsigned char prec)
{
  unsigned char buffer[10];
  buffer[0] = '$';
  buffer[1] = hexDigits[(n & 0xF0000000UL) >> 28];
  buffer[2] = hexDigits[(n & 0x0F000000UL) >> 24];
  buffer[3] = hexDigits[(n & 0x00F00000UL) >> 20];
  buffer[4] = hexDigits[(n & 0x000F0000UL) >> 16];
  buffer[5] = hexDigits[(n & 0x0000F000UL) >> 12];
  buffer[6] = hexDigits[(n & 0x00000F00UL) >> 8];
  buffer[7] = hexDigits[(n & 0x000000F0UL) >> 4];
  buffer[8] = hexDigits[(n & 0x0000000FUL)];
  buffer[9] = '\0';
  buffer[8 - prec] = '$';
  cputs(&buffer[8 - prec]);
}

void cputdec(long n, unsigned char padding, unsigned char leadingZeros)
{
  unsigned char buffer[11];
  unsigned char rem = 0, i = 0;
  char digit = 9;
  padding = 0; // NOTE: done to suppress compiler warning
  buffer[10] = '\0';
  do {
    rem = n % 10;
    n /= 10;
    buffer[digit--] = hexDigits[rem];
  } while (((int)digit >= 0) && (n != 0));

  while (((int)digit >= 0) && (leadingZeros--)) {
    buffer[digit--] = hexDigits[0];
  }

  cputs(&buffer[digit + 1]);
}

void cputs(const unsigned char* s)
{
  cputsxy(g_curX, g_curY, s);
}

void cputsxy(unsigned char x, unsigned char y, const unsigned char* s)
{
  const unsigned char len = strlen((const char*)s);
  const unsigned int offset = (y * (unsigned int)g_curScreenW) + x;
  lcopy((long)s, SCREEN_RAM_BASE + offset, len);
  lfill(COLOR_RAM_BASE + offset, g_curTextColor, len);
  g_curY = y + ((x + len) / g_curScreenW);
  g_curX = (x + len) % g_curScreenW;
}

void cputcxy(unsigned char x, unsigned char y, unsigned char c)
{
  const unsigned int offset = (y * (unsigned int)g_curScreenW) + x;
  lpoke(SCREEN_RAM_BASE + offset, c);
  lpoke(COLOR_RAM_BASE + offset, g_curTextColor);
  g_curX = (x == g_curScreenW - 1) ? 0 : (x + 1);
  g_curY = (x == g_curScreenW - 1) ? (y + 1) : y;
}

void cputncxy(unsigned char x, unsigned char y, unsigned char count, unsigned char c)
{
  const unsigned int offset = (y * (unsigned int)g_curScreenW) + x;
  lfill(SCREEN_RAM_BASE + offset, c, count);
  lfill(COLOR_RAM_BASE + offset, g_curTextColor, count);
  g_curY = y + ((x + count) / g_curScreenW);
  g_curX = (x + count) % g_curScreenW;
}

void fillrect(const RECT* rc, unsigned char ch, unsigned char col)
{
  register unsigned char i = 0;
  const unsigned char len = rc->right - rc->left;
  for (i = rc->top; i <= rc->bottom; ++i) {
    const unsigned int offset = (i * (unsigned int)g_curScreenW) + rc->left;
    lfill(SCREEN_RAM_BASE + offset, ch, len);
    lfill(COLOR_RAM_BASE + offset, col, len);
  }
}

void box(const RECT* rc, unsigned char color, unsigned char style, unsigned char clear, unsigned char shadow)
{
  register unsigned char i = 0;
  const unsigned char len = rc->right - rc->left;
  unsigned char prevCol = g_curTextColor;

  textcolor(color);
  if (clear)
    fillrect(rc, ' ', g_curTextColor);

  cputcxy(rc->left, rc->top, chTopLeft[style]);
  cputcxy(rc->left, rc->bottom, chBottomLeft[style]);
  cputcxy(rc->right, rc->top, chTopRight[style]);
  cputcxy(rc->right, rc->bottom, chBottomRight[style]);

  for (i = 1; i < len; ++i) {
    cputcxy(rc->left + i, rc->top, chHorzTop[style]);
    cputcxy(rc->left + i, rc->bottom, chHorzBottom[style]);
  }

  for (i = rc->top + 1; i <= rc->bottom - 1; ++i) {
    cputcxy(rc->left, i, chVertLeft[style]);
    cputcxy(rc->right, i, chVertRight[style]);
  }

  if (shadow && rc->bottom < g_curScreenH && rc->right < g_curScreenW) {
    lfill(COLOR_RAM_BASE + ((rc->bottom + 1) * (unsigned int)g_curScreenW) + (1 + rc->left), COLOUR_DARKGREY, len);
    for (i = rc->top + 1; i <= rc->bottom + 1; ++i)
      cellcolor(rc->right + 1, i, COLOUR_DARKGREY);
  }
  textcolor(prevCol);
}

void hline(unsigned char x, unsigned char y, unsigned char len, unsigned char style)
{
  cputncxy(x, y, len, style);
}

void vline(unsigned char x, unsigned char y, unsigned char len, unsigned char style)
{
  register unsigned char i;
  for (i = 0; i < len; ++i) {
    cputcxy(x, y + i, style);
  }
}

unsigned char cgetc(void)
{
  unsigned char k;
  while ((k = PEEK(0xD610U)) == 0)
    ;
  POKE(0xD610U, 0);
  return k;
}

unsigned char getkeymodstate(void)
{
  return PEEK(0xD611U);
}

unsigned char kbhit(void)
{
  return PEEK(0xD610U);
}

void flushkeybuf(void)
{
  while (PEEK(0xD610U))
    POKE(0xD610U, 0);
}

unsigned char cinput(unsigned char* buffer, unsigned char buflen, unsigned char flags)
{
  register unsigned char numch = 0, i, ch;
  const int sx = wherex();
  const int sy = wherey();

  if (buffer == NULL || buflen == 0)
    return 0;

  flushkeybuf();

  for (i = 0; i < buflen; ++i)
    buffer[i] = '\0';

  while (1) {
    cputsxy(sx, sy, buffer);
    blink(1);
    cputc(224);
    blink(0);
    ch = cgetc();

    if (ch == 13) {
      break;
    }

    if (ch == 20 && numch > 0) {
      moveleft(1);
      cputc(' ');
      buffer[--numch] = '\0';
    }
    else if (numch < buflen - 1) {
      if ((((ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z')) && (flags & CINPUT_ACCEPT_LETTER))
          || ((ch >= '0' && ch <= '9') && (flags & CINPUT_ACCEPT_NUMERIC)) || (flags & CINPUT_ACCEPT_ALL)) {
        if ((ch >= 0x61 && ch <= 0x7a) && (PEEK(0x0D18) & ~2) && (flags & ~CINPUT_NO_AUTOTRANSLATE)) {
          ch -= 0x20;
        }

        buffer[numch++] = ch;
      }
    }
  }

  return numch;
}

void setpalbank(unsigned char bank)
{
  POKE(0xD070U, (PEEK(0xD070U) & ~0x30) | ((bank & 0x3) << 4));
}

void setpalbanka(unsigned char bank)
{
  POKE(0xD070U, (PEEK(0xD070U) & ~0x3) | (bank & 0x3));
}

unsigned char getpalbank(void)
{
  return (PEEK(0xD070U) & 0x30) >> 4;
}

unsigned char getpalbanka(void)
{
  return PEEK(0xD070U) & 0x3;
}

void setmapedpal(unsigned char bank)
{
  POKE(0xD070U, (PEEK(0xD070U) & ~0xC0) | ((bank & 0x3) << 6));
}

unsigned char getmapedpal(void)
{
  return PEEK(0xD070U) >> 6;
}

void setpalentry(unsigned char c, unsigned char r, unsigned char g, unsigned char b)
{
  POKE(0xD100U + c, r);
  POKE(0xD200U + c, g);
  POKE(0xD300U + c, b);
}
