/*  Simple mouse support for the Mega65 libC

    Copyright (c) 2020 Paul Gardner-Stephen

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

    Version   0.1
    Date      2020-07-01
*/

#include "memory.h"

unsigned short mouse_min_x = 0;
unsigned short mouse_min_y = 0;
unsigned short mouse_max_x = 319;
unsigned short mouse_max_y = 199;
unsigned short mouse_x = 0;
unsigned short mouse_y = 0;
unsigned char mouse_sprite_number = 0xff;
unsigned char mouse_pot_x = 0;
unsigned char mouse_pot_y = 0;
char mouse_click_flag = 0;

void mouse_set_bounding_box(unsigned short x1, unsigned short y1, unsigned short x2, unsigned short y2)
{
  mouse_min_x = x1;
  mouse_min_y = y1;
  mouse_max_x = x2;
  mouse_max_y = y2;
}

void mouse_bind_to_sprite(unsigned char sprite_num)
{
  mouse_sprite_number = sprite_num;
}

void mouse_clip_position(void)
{
  if (mouse_x < mouse_min_x)
    mouse_x = mouse_min_x;
  if (mouse_y < mouse_min_y)
    mouse_y = mouse_min_y;
  if (mouse_x > mouse_max_x)
    mouse_x = mouse_max_x;
  if (mouse_y > mouse_max_y)
    mouse_y = mouse_max_y;
}

char mouse_clicked(void)
{
  if (!(PEEK(0xDC01) & 0x10))
    mouse_click_flag = 1;
  if (mouse_click_flag) {
    mouse_click_flag = 0;
    return 1;
  }
}

void mouse_update_pointer(void)
{
  if (mouse_sprite_number < 8) {
    POKE(0xD000 + (mouse_sprite_number << 1), mouse_x & 0xff);
    if (mouse_x & 0x100)
      POKE(0xD010, PEEK(0xD010) | (1 << mouse_sprite_number));
    else
      POKE(0xD010, PEEK(0xD010) & (0xFF - (1 << mouse_sprite_number)));
    if (mouse_x & 0x200)
      POKE(0xD05F, PEEK(0xD05F) | (1 << mouse_sprite_number));
    else
      POKE(0xD05F, PEEK(0xD05F) & (0xFF - (1 << mouse_sprite_number)));

    POKE(0xD001 + (mouse_sprite_number << 1), mouse_y & 0xff);
    if (mouse_y & 0x100)
      POKE(0xD077, PEEK(0xD077) | (1 << mouse_sprite_number));
    else
      POKE(0xD077, PEEK(0xD077) & (0xFF - (1 << mouse_sprite_number)));
    if (mouse_y & 0x200)
      POKE(0xD05F, PEEK(0xD05F) | (1 << mouse_sprite_number));
    else
      POKE(0xD078, PEEK(0xD078) & (0xFF - (1 << mouse_sprite_number)));
  }
}

void mouse_update_position(unsigned short* mx, unsigned short* my)
{
  unsigned char delta;
  delta = PEEK(0xD620) - mouse_pot_x;
  mouse_pot_x = PEEK(0xD620);
  if (delta >= 0x01 && delta <= 0x3f)
    mouse_x += delta;
  delta = -delta;
  if (delta >= 0x01 && delta <= 0x3f)
    mouse_x -= delta;

  delta = PEEK(0xD621) - mouse_pot_y;
  mouse_pot_y = PEEK(0xD621);
  if (delta >= 0x01 && delta <= 0x3f)
    mouse_y -= delta;
  delta = -delta;
  if (delta >= 0x01 && delta <= 0x3f)
    mouse_y += delta;

  mouse_clip_position();
  mouse_update_pointer();

  if (!(PEEK(0xDC01) & 0x10))
    mouse_click_flag = 1;

  if (mx)
    *mx = mouse_x;
  if (my)
    *my = mouse_y;
}

void mouse_warp_to(unsigned short x, unsigned short y)
{
  mouse_x = x;
  mouse_y = y;
  mouse_clip_position();
  mouse_update_pointer();

  // Mark POT position as read
  mouse_pot_x = PEEK(0xD620);
  mouse_pot_y = PEEK(0xD621);
}
