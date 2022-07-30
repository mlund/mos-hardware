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

    Version   0.10
    Date      2021-06-21

    CHANGELOG

    v0.4        Added getscreensize, setscreensize, setextendattribute,
                set16bitcharmode, moveup,moveleft,moveright, movedown, gohome,
                flushkeybuf.
                Cache screen sizes for faster calls.
                Added cprintf escape codes for formatted screen colors and attributes.
                Added proper initialisation function.
                Fixed a bug where screen was fixed at $8000!

    v0.5        Added fillrect, box, cgets, wherex, wherey, togglecase functions.
                Fixed moveXXXX to do multiple steps.  Minor optimizations in cputs/cputc.

    v0.6        Added vline, hline  to draw lines. cputnc,cputncxy for repeating characters.

    v0.6.5      Added LaTEX style comments for doc generation.

    v0.7        Added cinput function. Fixed cputncxy color fill. BOX_STYLE_NONE added.

    v0.8        Fixed documentation. COLOR_RAM_BASE is 0xFF80000UL to access full 32/64KB space.
                Added setcolramoffset, getcolramoffset, setcharsetaddr, getcharsetaddr.

    v0.8.1      Fixed Latex documentation.

    v0.9        Added sethotregs.

    v0.9.1      Fixed cinput buffer overrun bug and documentation.

    v0.9.2      Fixed CC65 2.19 mismatched header types.

    v0.10       Added setpalbank, getpalbank, setpalbanka, getpalbanka, setmapedpal, getmapedpal, setpalentry.

*/

#ifndef M65LIBC_CONIO_H
#define M65LIBC_CONIO_H

/*------------------------------------------------------------------------
  Color and attributes
  -----------------------------------------------------------------------*/
#define ATTRIB_BLINK 0x10
#define ATTRIB_REVERSE 0x20
#define ATTRIB_UNDERLINE 0x80
#define ATTRIB_HIGHLIGHT 0x40

#define COLOUR_BLACK 0
#define COLOUR_WHITE 1
#define COLOUR_RED 2
#define COLOUR_CYAN 3
#define COLOUR_PURPLE 4
#define COLOUR_GREEN 5
#define COLOUR_BLUE 6
#define COLOUR_YELLOW 7
#define COLOUR_ORANGE 8
#define COLOUR_BROWN 9
#define COLOUR_PINK 10
#define COLOUR_GREY1 11
#define COLOUR_DARKGREY 11
#define COLOUR_GREY2 12
#define COLOUR_GREY 12
#define COLOUR_MEDIUMGREY 12
#define COLOUR_LIGHTGREEN 13
#define COLOUR_LIGHTBLUE 14
#define COLOUR_GREY3 15
#define COLOUR_LIGHTGREY 15

/*------------------------------------------------------------------------
  Keyboard ASCII codes
  -----------------------------------------------------------------------*/
#define ASC_A
#define ASC_D
#define ASC_E
#define ASC_F
#define ASC_G
#define ASC_H
#define ASC_I
#define ASC_J
#define ASC_K
#define ASC_L
#define ASC_M
#define ASC_N
#define ASC_O
#define ASC_P
#define ASC_Q
#define ASC_R
#define ASC_S
#define ASC_T
#define ASC_U
#define ASC_V
#define ASC_W
#define ASC_X
#define ASC_Y
#define ASC_Z
#define ASC_F1
#define ASC_F3
#define ASC_F5
#define ASC_F7
#define ASC_F9
#define ASC_F11
#define ASC_F13
#define ASC_CRSR_RIGHT
#define ASC_CRSR_LEFT
#define ASC_CRSR_UP
#define ASC_CRSR_DOWN

/*------------------------------------------------------------------------
  Keyboard modifiers
  -----------------------------------------------------------------------*/
#define KEYMOD_RSHIFT 1
#define KEYMOD_LSHIFT 2
#define KEYMOD_CTRL 4
#define KEYMOD_MEGA 8
#define KEYMOD_ALT 16
#define KEYMOD_NOSCRL 32
#define KEYMOD_CAPSLOCK 64

/*------------------------------------------------------------------------
  Box styles
  -----------------------------------------------------------------------*/
#define BOX_STYLE_NONE 0
#define BOX_STYLE_INNER 1
#define BOX_STYLE_MID 2
#define BOX_STYLE_OUTER 3
#define BOX_STYLE_ROUND 4

/*------------------------------------------------------------------------
  Line styles
  -----------------------------------------------------------------------*/
#define HLINE_STYLE_TOP_THIN 0x63
#define HLINE_STYLE_BTM_THIN 0x64
#define HLINE_STYLE_TOP_NORMAL 0x77
#define HLINE_STYLE_BTM_NORMAL 0x6F
#define HLINE_STYLE_TOP1_8 0x45 // 1/8
#define HLINE_STYLE_TOP3_8 0x44 // 3/8
#define HLINE_STYLE_BTM1_8 0x52 // 1/8
#define HLINE_STYLE_BTM3_8 0x46 // 3/8
#define HLINE_STYLE_MID 0x40
#define HLINE_STYLE_CHECKER 0x68
#define VLINE_STYLE_LEFT_NORMAL 0x74
#define VLINE_STYLE_RIGHT_NORMAL 0x6A
#define VLINE_STYLE_MID 0x42
#define VLINE_STYLE_CHECKER 0x5C

/*------------------------------------------------------------------------
  Input character modes
  -----------------------------------------------------------------------*/
#define CINPUT_ACCEPT_NUMERIC 1
#define CINPUT_ACCEPT_LETTER 2
#define CINPUT_ACCEPT_ALL 4
#define CINPUT_NO_AUTOTRANSLATE 8
#define CINPUT_ACCEPT_ALPHA CINPUT_ACCEPT_NUMERIC | CINPUT_ACCEPT_LETTER

/*------------------------------------------------------------------------
  Public structs
  -----------------------------------------------------------------------*/
typedef struct tagRECT {
  unsigned char left, top, right, bottom;
} RECT;

/*------------------------------------------------------------------------
  Screen configuration and setup
  -----------------------------------------------------------------------*/

/** \m65libsummary{conionit}{Initialises the library internal state}
    \m65libsyntax    {void conioinit(void)}
    \m65libremarks{This must be called before using any conio library function.}
*/
void conioinit(void);

void setlowercase(void);
void setuppercase(void);

/** \m65libsummary{setscreenaddr}{Sets the screen RAM start address}
    \m65libsyntax    {void setscreenaddr(long addr);}
    \m65libparam     {addr}{The address to set as start of screen RAM}
    \m65example   {
      // Set beginning of screen RAM at $48000
      setscreenaddr(0x48000UL);
    }
    \m65libremarks{No bounds check is performed on the selected address}
*/
void setscreenaddr(long addr);

/** \m65libsummary{getscreenaddr}{Returns the screen RAM start address}
    \m65libsyntax    {long getscreenaddr(void);}
    \m65libretval    {The current screen RAM address start address.}
*/
long getscreenaddr(void);

/** \m65libsummary{setcolramoffset}{Sets the color RAM start offset value}
    \m65libsyntax    {void setcolramoffset(long offset);}
    \m65libparam     {addr}{The offset from the beginning of the color RAM address ($FF80000)}
    \m65libremarks{No bounds check is performed on the resulting address. Do not exceed the available Color RAM size}
*/
void setcolramoffset(unsigned int addr);

/** \m65libsummary{getcolramoffset}{Returns the color RAM start offset value}
    \m65libsyntax    {long getscreenaddr(void);}
    \m65libretval    {The current color RAM start offset value.}
*/
unsigned int getcolramoffset(void);

/** \m65libsummary{setcharsetaddr}{Sets the character set start address}
    \m65libsyntax    {void setcharsetaddr(long addr);}
    \m65libparam     {addr}{The address to set as start of character set}
    \m65libremarks   {No bounds check is performed on the selected address}
*/
void setcharsetaddr(long addr);

/** \m65libsummary{getcharsetaddr}{Returns the current character set start address}
    \m65libsyntax    {long getscreenaddr(void);}
    \m65libretval    {The current character set start address.}
*/
long getcharsetaddr(void);

/**
    \m65libsummary{clrscr}{Clear the text screen. }
    \m65libsyntax    {void clrscr(void)}
    \m65example   {
      // Clear screen to white
      textcolor(COLOUR_WHITE);
      clrscr();
    }
    \m65libremarks{Color RAM will be cleared with current text color}
*/
void clrscr(void);

/** \m65libsummary{getscreensize}{Returns the dimensions of the text screen}
    \m65libsyntax    {void getscreensize(unsigned char* width, unsigned char* height)}
    \m65libparam     {width}{Pointer to location where width will be returned}
    \m65libparam     {height}{Pointer to location where height will be returned}
*/
void getscreensize(unsigned char* width, unsigned char* height);

/** \m65libsummary{setscreensize}{Sets the dimensions of the text screen}
    \m65libsyntax    {void setscreensize(unsigned char width, unsigned char height)}
    \m65libparam     {width}{The width in columns (40 or 80)}
    \m65libparam     {height}{The height in rows (25 or 50)}
    \m65libremarks   {Currently only 40/80 and 25/50 are accepted. Other values are ignored.}
*/
void setscreensize(unsigned char width, unsigned char height);

/** \m65libsummary{set16bitcharmode}{Sets or clear the 16-bit character mode}
    \m65libsyntax    {void set16bitcharmode(unsigned char f)}
    \m65libparam     {f}{Set true to set the 16-bit character mode}
    \m65libremarks   {This will trigger a video parameter reset if HOTREG is ENABLED. See sethotregs function.}
*/
void set16bitcharmode(unsigned char f);

/** \m65libsummary{sethotregs}{Sets or clear the hot-register behavior of the VIC-IV chip.}
    \m65libsyntax    {void set16bitcharmode(unsigned char f)}
    \m65libparam     {f}{Set true to enable the hotreg behavior}
    \m65libremarks   {When this mode is ENABLED a video mode reset will be triggered when touching $D011, $D016, $D018, $D031
   or the VIC-II bank bits of $DD00. }
*/
void sethotregs(unsigned char f);

/** \m65libsummary{setextendedattrib}{Sets or clear the VIC-III extended attributes mode to support blink, underline, bold
   and highlight.} \m65libsyntax    {void setextendedattrib(unsigned char f)} \m65libparam     {f}{Set true to set the
   extended attributes mode}
*/
void setextendedattrib(unsigned char f);

/** \m65libsummary{togglecase}{Set lower case character set}
    \m65libsyntax    {void setlowercase(void)}
*/
void setlowercase(void);

/** \m65libsummary{togglecase}{Set upper case character set}
    \m65libsyntax    {void setuppercase(void)}
*/
void setuppercase(void);

/** \m65libsummary{togglecase}{Toggle the current character set case}
    \m65libsyntax    {void togglecase(void)}
*/
void togglecase(void);

/*------------------------------------------------------------------------
  Color and Attributes
  -----------------------------------------------------------------------*/

/** \m65libsummary{bordercolor}{Sets the current border color}
    \m65libsyntax    {void bordercolor(unsigned char c)}
    \m65libparam     {c}{The color to set}
*/
void bordercolor(unsigned char c);

/** \m65libsummary{bgcolor}{Sets the current screen (background) color}
    \m65libsyntax    {void bgcolor(unsigned char c)}
    \m65libparam     {c}{The color to set}
*/
void bgcolor(unsigned char c);

/** \m65libsummary{textcolor}{Sets the current text color}
    \m65libsyntax    {void textcolor(unsigned char c)}
    \m65libparam     {c}{The color to set}
    \m65libremarks   {This function preserves attributes in the upper 4-bits if extended attributes are enabled. See
   setextendedattrib. }
*/
void textcolor(unsigned char c);

/** \m65libsummary{revers}{Enable the reverse attribute}
    \m65libsyntax    {void revers(unsigned char c)}
    \m65libparam     {enable}{0 to disable, 1 to enable}
    \m65libremarks   {Extended attributes mode must be active. See setextendedattrib.}
*/
void revers(unsigned char enable);

/** \m65libsummary{highlight}{Enable the highlight attribute}
    \m65libsyntax    {void highlight(unsigned char c)}
    \m65libparam     {enable}{0 to disable, 1 to enable}
    \m65libremarks   {Extended attributes mode must be active. See setextendedattrib.}
*/
void highlight(unsigned char enable);

/** \m65libsummary{blink}{Enable the blink attribute}
    \m65libsyntax    {void blink(unsigned char c)}
    \m65libparam     {enable}{0 to disable, 1 to enable}
    \m65libremarks   {Extended attributes mode must be active. See setextendedattrib.}
*/
void blink(unsigned char enable);

/** \m65libsummary{underline}{Enable the underline attribute}
    \m65libsyntax    {void underline(unsigned char c)}
    \m65libparam     {enable}{0 to disable, 1 to enable}
    \m65libremarks   {Extended attributes mode must be active. See setextendedattrib.}
*/
void underline(unsigned char enable);

/** \m65libsummary{altpal}{Enable the alternate-palette attribute}
    \m65libsyntax    {void altpal(unsigned char c)}
    \m65libparam     {enable}{0 to disable, 1 to enable}
    \m65libremarks   {Extended attributes mode must be active. See setextendedattrib.}
*/
void altpal(unsigned char enable);

/** \m65libsummary{clearattr}{Clear all text attributes}
    \m65libsyntax    {void clearattr())}
    \m65libremarks   {Extended attributes mode must be active. See setextendedattrib.}
*/
void clearattr(void);

/** \m65libsummary{cellcolor}{Sets the color of a character cell}
    \m65libsyntax    {void cellcolor(unsigned char x, unsigned char y, unsigned char c)}
    \m65libparam     {x}{The cell X-coordinate}
    \m65libparam     {y}{The cell Y-coordinate}
    \m65libparam     {c}{The color to set}
    \m65libremarks   {No screen bounds checks are performed; out of screen behavior is undefined }
*/
void cellcolor(unsigned char x, unsigned char y, unsigned char c);

/*------------------------------------------------------------------------
  Palette management
  -----------------------------------------------------------------------*/
/** \m65libsummary{setpalbank}{Set current text/bitmap palette bank (BTPALSEL).}
    \m65libsyntax    {void setpalbank(unsigned char bank)}
    \m65libparam     {bank}{The palette bank to set. Valid values are 0, 1, 2 or 3.}
    \m65libremarks   {Use setpalbanka to set alternate text/bitmap palette}
*/
void setpalbank(unsigned char bank);

/** \m65libsummary{setpalbanka}{Set alternate text/bitmap palette bank.}
    \m65libsyntax    {void setpalbanka(unsigned char bank)}
    \m65libparam     {bank}{The palette bank to set. Valid values are 0, 1, 2 or 3.}
    \m65libremarks   {Use setpalbank to set main text/bitmap palette}
*/
void setpalbanka(unsigned char bank);

/** \m65libsummary{getpalbank}{Get selected text/bitmap palette bank.}
    \m65libsyntax    {unsigned char getpalbank(void)}
    \m65libremarks   {Use getpalbanka to get alternate text/bitmap selected palette}
    \m65libretval    {The current selected main text/bitmap palette bank.}
*/
unsigned char getpalbank(void);

/** \m65libsummary{getpalbanka}{Get selected alternate text/bitmap palette bank.}
    \m65libsyntax    {unsigned char getpalbanka(void)}
    \m65libremarks   {Use getpalbank to get main text/bitmap selected palette}
    \m65libretval    {The current selected alternate text/bitmap palette bank.}
*/
unsigned char getpalbanka(void);

/** \m65libsummary{setmapedpal}{Set maped-in palette bank at $D100-$D3FF.}
    \m65libsyntax    {void setmapedpal(unsigned char bank)}
    \m65libparam     {bank}{The palette bank to map-in. Valid values are 0, 1, 2 or 3.}
*/
void setmapedpal(unsigned char bank);

/** \m65libsummary{getmapedpal}{Get maped-in  palette bank at $D100-$D3FF.}
    \m65libsyntax    {unsigned char getmapedpal(void)}
*/
unsigned char getmapedpal(void);

/** \m65libsummary{setpalentry}{Set color entry for the maped-in palette}
    \m65libsyntax    {void setpalentry(unsigned char c, unsigned char r, unsigned char g, unsigned char b)}
    \m65libparam     {c}{The palette entry index (0-255)}
    \m65libparam     {r}{The red component value}
    \m65libparam     {g}{The green component value}
    \m65libparam     {b}{The blue component value}
    \m65libremarks   {Use setmapedmal to bank-in the palette to modify}
*/
void setpalentry(unsigned char c, unsigned char r, unsigned char g, unsigned char b);

/*------------------------------------------------------------------------
  Screen draw operations
  -----------------------------------------------------------------------*/

/** \m65libsummary{fillrect}{Fill a rectangular area with character and color value}
    \m65libsyntax    {void fillrect(const RECT *rc, unsigned char ch, unsigned char col)}
    \m65libparam     {rc}{A RECT structure specifying the box coordinates}
    \m65libparam     {ch}{A char code to fill the rectangle}
    \m65libparam     {col}{The color to fill}
    \m65libremarks   {No screen bounds checks are performed; out of screen behavior is undefined }
*/
void fillrect(const RECT* rc, unsigned char ch, unsigned char col);

/** \m65libsummary{box}{Draws a box with graphic characters}
    \m65libsyntax    {void box(const RECT *rc, unsigned char color, unsigned char style, unsigned char clear, unsigned char
   shadow)} \m65libparam     {rc}{A RECT structure specifying the box coordinates} \m65libparam     {color}{The color to use
   for the graphic characters} \m65libparam     {style}{The style for the box borders. Can be set to BOX_STYLE_NONE,
   BOX_STYLE_ROUNDED, BOX_STYLE_INNER, BOX_STYLE_OUTER, BOX_STYLE_MID } \m65libparam     {clear}{Set to 1 to clear the box
   interior with the selected color} \m65libparam     {shadow}{Set to 1 to draw a drop shadow} \m65libremarks   {No screen
   bounds checks are performed; out of screen behavior is undefined }
*/
void box(const RECT* rc, unsigned char color, unsigned char style, unsigned char clear, unsigned char shadow);

/** \m65libsummary{hline}{Draws an horizontal line.}
    \m65libsyntax    {void hline(unsigned char x, unsigned char y, unsigned char len, unsigned char style)}
    \m65libparam     {x}{The line start X-coordinate}
    \m65libparam     {y}{The line start Y-coordinate}
    \m65libparam     {len}{The line length}
    \m65libparam     {style}{The style for the line. See HLINE_ constants for available styles. }
    \m65libremarks   {No screen bounds checks are performed; out of screen behavior is undefined }
*/
void hline(unsigned char x, unsigned char y, unsigned char len, unsigned char style);

/** \m65libsummary{vline}{Draws a vertical line.}
    \m65libsyntax    {void vline(unsigned char x, unsigned char y, unsigned char len, unsigned char style)}
    \m65libparam     {x}{The line start X-coordinate}
    \m65libparam     {y}{The line start Y-coordinate}
    \m65libparam     {len}{The line length}
    \m65libparam     {style}{The style for the line. See VLINE_ constants for available styles. }
    \m65libremarks   {No screen bounds checks are performed; out of screen behavior is undefined }
*/
void vline(unsigned char x, unsigned char y, unsigned char len, unsigned char style);

/*------------------------------------------------------------------------
  Cursor Movement
  -----------------------------------------------------------------------*/

/** \m65libsummary{gohome}{Set the current position at home (0,0 coordinate)}
    \m65libsyntax    {void gohome(void)}
*/
void gohome(void);

/** \m65libsummary{gotoxy}{Set the current position at X,Y coordinates}
    \m65libsyntax    {void gotoxy(unsigned char x, unsigned char y)}
    \m65libparam     {x}{The new X-coordinate}
    \m65libparam     {y}{The new Y-coordinate}
    \m65libremarks   {No screen bounds checks are performed; out of screen behavior is undefined }
*/
void gotoxy(unsigned char x, unsigned char y);

/** \m65libsummary{gotox}{Set the current position X-coordinate}
    \m65libsyntax    {void gotox(unsigned char x)}
    \m65libparam     {x}{The new X-coordinate}
    \m65libremarks   {No screen bounds checks are performed; out of screen behavior is undefined }
*/
void gotox(unsigned char x);

/** \m65libsummary{gotoy}{Set the current position Y-coordinate}
    \m65libsyntax    {void gotoy(unsigned char y)}
    \m65libparam     {y}{The new Y-coordinate}
    \m65libremarks   {No screen bounds checks are performed; out of screen behavior is undefined }
*/
void gotoy(unsigned char y);

/** \m65libsummary{moveup}{Move current position up}
    \m65libsyntax    {void moveup(unsigned char count)}
    \m65libparam     {count}{The number of positions to move}
    \m65libremarks   {No screen bounds checks are performed; out of screen behavior is undefined }
*/
void moveup(unsigned char count);

/** \m65libsummary{movedown}{Move current position down}
    \m65libsyntax    {void movedown(unsigned char count)}
    \m65libparam     {count}{The number of positions to move}
    \m65libremarks   {No screen bounds checks are performed; out of screen behavior is undefined }
*/
void movedown(unsigned char count);

/** \m65libsummary{moveleft}{Move current position left}
    \m65libsyntax    {void moveleft(unsigned char count)}
    \m65libparam     {count}{The number of positions to move}
    \m65libremarks   {No screen bounds checks are performed; out of screen behavior is undefined }
*/
void moveleft(unsigned char count);

/** \m65libsummary{moveright}{Move current position right}
    \m65libsyntax    {void moveright(unsigned char count)}
    \m65libparam     {count}{The number of positions to move}
    \m65libremarks   {No screen bounds checks are performed; out of screen behavior is undefined }
*/
void moveright(unsigned char count);

/** \m65libsummary{wherex}{Return the current position X coordinate}
    \m65libsyntax    {unsigned char wherex(void)}
    \m65libretval    {The current position X coordinate}
*/
unsigned char wherex(void);

/** \m65libsummary{wherey}{Return the current position Y coordinate}
    \m65libsyntax    {unsigned char wherey(void)}
    \m65libretval    {The current position Y coordinate}
*/
unsigned char wherey(void);

/*------------------------------------------------------------------------
  PETSCII conversion output
  -----------------------------------------------------------------------*/

char petsciitoscreencode(char c);
char* petsciitoscreencode_s(char* s);

/** \m65libsummary{pcputc}{Output a single petscii character to screen at current position}
    \m65libsyntax    {void cputc(unsigned char c)}
    \m65libparam     {c}{The petscii character to output}
*/

#define pcputc(c) cputc(petsciitoscreencode(c))

/** \m65libsummary{pcputsxy}{Output a petscii string at X,Y coordinates}
    \m65libsyntax    {void pcputsxy (unsigned char x, unsigned char y, const unsigned char* s)}
    \m65libparam     {x}{The X coordinate where string will be printed}
    \m65libparam     {y}{The Y coordinate where string will be printed}
    \m65libparam     {s}{The petscii string to print}
    \m65libremarks   {No pointer check is performed.  If s is null or invalid, behavior is undefined }
*/

#define pcputsxy(x, y, s) cputsxy(x, y, petsciitoscreencode_s(s))

/** \m65libsummary{cputcxy}{Output a single petscii character at X,Y coordinates}
    \m65libsyntax    {void pcputcxy (unsigned char x, unsigned char y, unsigned char c)}
    \m65libparam     {x}{The X coordinate where character will be printed}
    \m65libparam     {y}{The Y coordinate where character will be printed}
    \m65libparam     {c}{The petscii character to print}
*/

#define pcputcxy(x, y, c) cputcxy(x, y, petsciitoscreencode(c))

/** \m65libsummary{pcputs}{Output a petscii string at current position}
    \m65libsyntax    {void pcputs(const unsigned char* s)}
    \m65libparam     {s}{The string to print}
    \m65libremarks   {No pointer check is performed.  If s is null or invalid, behavior is undefined }
    */

#define pcputs(s) cputs((unsigned char*)petsciitoscreencode_s(s));

/*------------------------------------------------------------------------
  Text output
  -----------------------------------------------------------------------*/

/** \m65libsummary{cputc}{Output a single screen code character to screen at current position}
    \m65libsyntax    {void cputc(unsigned char c)}
    \m65libparam     {c}{The screen code of the character to output}
*/
void cputc(unsigned char c);

/** \m65libsummary{cputnc}{Output N copies of a character at current position}
    \m65libsyntax    {void cputnc(unsigned char count, unsigned char c)}
    \m65libparam     {c}{The screen code of the characters to output}
    \m65libparam     {count}{The count of characters to print}
*/
void cputnc(unsigned char count, unsigned char c);

/** \m65libsummary{cputhex}{Output an hex-formatted number at current position}
    \m65libsyntax    {void cputhex(long n, unsigned char prec)}
    \m65libparam     {n}{The number to write}
    \m65libparam     {prec}{The precision of the hex number, in digits. Leading zeros will be printed accordingly}
    \m65libremarks   {The $ symbol will be automatically added at beginning of string}
*/
void cputhex(long n, unsigned char prec);

/** \m65libsummary{cputdec}{Output a decimal number at current position}
    \m65libsyntax    {void cputdec(long n, unsigned char padding, unsigned char leadingZ)}
    \m65libparam     {n}{The number to write}
    \m65libparam     {padding}{The padding space to add before number}
    \m65libparam     {leadingZ}{The leading zeros to print}
*/
void cputdec(long n, unsigned char padding, unsigned char leadingZ);

/** \m65libsummary{cputs}{Output screen codes at current position}
    \m65libsyntax    {void cputs(const unsigned char* s)}
    \m65libparam     {s}{Am array of screen codes to print}
    \m65libremarks   {This function works with screen codes only. To output ordinary ASCII/PETSCII strings,
    use the "pcputs" macro. No pointer check is performed.  If s is null or invalid, behavior is undefined. }
*/
void cputs(const unsigned char* s);

/** \m65libsummary{cputsxy}{Output multiple screen codes at X,Y coordinates}
    \m65libsyntax    {void cputsxy (unsigned char x, unsigned char y, const unsigned char* s)}
    \m65libparam     {x}{The X coordinate where string will be printed}
    \m65libparam     {y}{The Y coordinate where string will be printed}
    \m65libparam     {s}{An array of screen codes to print}
    \m65libremarks   {This function works with screen codes only. To output ordinary ASCII/PETSCII strings,
    use the "pcputsxy" macro. No pointer check is performed.  If s is null or invalid, behavior is undefined. }
*/
void cputsxy(unsigned char x, unsigned char y, const unsigned char* s);

/** \m65libsummary{cputcxy}{Output a single character at X,Y coordinates}
    \m65libsyntax    {void cputcxy (unsigned char x, unsigned char y, unsigned char c)}
    \m65libparam     {x}{The X coordinate where character will be printed}
    \m65libparam     {y}{The Y coordinate where character will be printed}
    \m65libparam     {c}{The screen code of the character to print}
*/
void cputcxy(unsigned char x, unsigned char y, unsigned char c);

/** \m65libsummary{cputncxy}{Output N copies of a single character at X,Y coordinates}
    \m65libsyntax    {void cputncxy (unsigned char x, unsigned char y, unsigned char count, unsigned char c)}
    \m65libparam     {x}{The X coordinate where character will be printed}
    \m65libparam     {y}{The Y coordinate where character will be printed}
    \m65libparam     {count}{The number of characters to output}
    \m65libparam     {c}{The screen code of the characters to print}
*/
void cputncxy(unsigned char x, unsigned char y, unsigned char count, unsigned char c);

// making raw _cprintf available here to be used by cprintf and pcprintf
// don't use this call directly as it might go away in a future release  of the library
unsigned char _cprintf(const unsigned char translateCodes, const unsigned char* fmt, ...);

/** \m65libsummary{cprintf}{Prints formatted output. \\
    Escape strings can be used to modify attributes, move cursor, etc similar to PRINT in CBM BASIC.
    }

    \m65libsyntax    {unsigned char cprintf (const unsigned char* format, ...)}
    \m65libparam     {format}{The string to output. The available escape codes are: \\
    %<
    \textbf{Cursor positioning} \\
    \begin{tabular}{ll}
    \textbackslash t             & Go to next tab position (multiple of 8s)            \\
    \textbackslash r             & Carriage Return            \\
    \textbackslash n             & New line          \\
    \end{tabular}

    \begin{tabular}{llll}
    \texttt{\{clr\}}   &     Clear screen      &  \texttt{\{home\}}  & Move cursor to home (top-left) \\
    \texttt{\{d\}}     &    Move cursor down   & \texttt{\{u\}}      & Move cursor up \\
    \texttt{\{r\}}     &    Move cursor right  & \texttt{\{l\}}     & Move cursor left \\

    \end{tabular}

    \textbf{Attributes} \\
    \begin{tabular}{llll}
    \texttt{\{rvson\}}  &  Reverse attribute ON   & \texttt{\{rvsoff\}} &  Reverse attribute OFF \\
    \texttt{\{blon\}}   &  Blink attribute ON     & \texttt{\{bloff\}}  &  Blink attribute OFF    \\
    \texttt{\{ulon\}}   &  Underline attribute ON & \texttt{\{uloff\}}  &  Underline attribute OFF \\
    \end{tabular}

    \textbf{Colors (default palette)} \\
    \begin{tabular}{llll}
    \texttt{\{blk\}}  & \texttt{\{wht\}}  &  \texttt{\{red\}} &  \texttt{\{cyan\}}  \\
    \texttt{\{pur\}}  & \texttt{\{grn\}}  &  \texttt{\{blu\}} &  \texttt{\{yel\}}   \\
    \texttt{\{ora\}}  & \texttt{\{brn\}}  &  \texttt{\{pink\}} & \texttt{\{gray1\}} \\
    \texttt{\{gray2\}} &  \texttt{\{lblu\}} &  \texttt{\{lgrn\}} & \texttt{\{gray3\}}

    \end{tabular}

    %>}

    \m65libremarks   {This function works with screen codes only! To output ordinary ASCII/PETSCII strings,
    use the "pcprintf" macro. Currently no argument replacement is done with the variable arguments.}
*/

#define cprintf(s) _cprintf(0, s)

/** \m65libsummary{pcprintf}{Prints formatted petscii string output.}

    \m65libsyntax    {see cprintf}

*/

#define pcprintf(s) _cprintf(1, s);

/*------------------------------------------------------------------------
  Keyboard input
  -----------------------------------------------------------------------*/
/** \m65libsummary{cgetc}{ Waits until a character is in the keyboard buffer and returns it }
    \m65libsyntax    {unsigned char cgetc (void);}
    \m65libretval    {The last character in the keyboard buffer }
    \m65libremarks   {Returned values are ASCII character codes}
*/
unsigned char cgetc(void);

/** \m65libsummary{kbhit}{ Returns the character in the keyboard buffer }
    \m65libsyntax    {unsigned char kbhit (void);}
    \m65libretval    {The character code in the keyboard buffer,  0 otherwise. }
    \m65libremarks   {Returned values are ASCII character codes}
*/
unsigned char kbhit(void);

/** \m65libsummary{getkeymodstate}{
   Return the key modifiers state.}
    \m65libsyntax    {unsigned char getkeymodstate(void)}
    \m65libretval    {A byte with the key modifier state bits,
    where bits:
   %<
    \begin{tabular}{lll}
    \textbf{Bit} & \textbf{Meaning} & \textbf{Constant}        \\
    0   & Right SHIFT State & \texttt{KEYMOD\_RSHIFT} \\
    1   & Left  SHIFT state & \texttt{KEYMOD\_LSHIFT} \\
    2   & CTRL state        & \texttt{KEYMOD\_CTRL}  \\
    3   & MEGA state        & \texttt{KEYMOD\_MEGA} \\
    4   & ALT state         & \texttt{KEYMOD\_ALT} \\
    5   & NOSCRL state      & \texttt{KEYMOD\_NOSCRL} \\
    6   & CAPSLOCK state    & \texttt{KEYMOD\_CAPSLOCK} \\
    7   & Reserved          & - \\
    \end{tabular}
    %>}
*/
unsigned char getkeymodstate(void);

/** \m65libsummary{flushkeybuf}{Flush the keyboard buffer}
    \m65libsyntax    {void flushkeybuf(void)}
*/
void flushkeybuf(void);

/** \m65libsummary{cinput}{Get input from keyboard, printing incoming characters at current position.}
    \m65libsyntax    {unsigned char cinput(char* buffer, unsigned char buflen, unsigned char flags)}
    \m65libparam     {buffer}{Target character buffer preallocated by caller}
    \m65libparam     {buflen}{Target buffer length in characters, including the null character terminator}
    \m65libparam     {flags}{Flags for input:  (default is accept all printable characters)
            %<
            \texttt{CINPUT\_ACCEPT\_NUMERIC} \\
            Accepts numeric characters. \\ \\
            \texttt{CINPUT\_ACCEPT\_LETTER}  \\
            Accepts letters.  \\ \\
            \texttt{CINPUT\_ACCEPT\_SYM}  \\
            Accepts symbols.  \\ \\
            \texttt{CINPUT\_ACCEPT\_ALL}\\
            Accepts all. Equals to \texttt{CINPUT\_ACCEPT\_NUMERIC \textbar CINPUT\_ACCEPT\_LETTER \textbar
   CINPUT\_ACCEPT\_SYM} \\ \\
            \texttt{CINPUT\_ACCEPT\_ALPHA} \\
            Accepts alphanumeric characters. Equals to \texttt{CINPUT\_ACCEPT\_NUMERIC \textbar CINPUT\_ACCEPT\_LETTER} \\ \\
            \texttt{CINPUT\_NO\_AUTOTRANSLATE}\\
            Disables the feature that makes cinput to autodisplay uppercase characters when standard lowercase character set
   is selected  and the user enters letters without the SHIFT key, that would display graphic characters instead of
   alphabetic ones. \\
            %>}

   \m65libretval    {Count of successfully read characters in buffer}
*/

unsigned char cinput(unsigned char* buffer, unsigned char buflen, unsigned char flags);

#endif // M65LIBC_CONIO_H
