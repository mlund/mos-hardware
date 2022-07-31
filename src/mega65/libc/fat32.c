#include "sdcard.h"
#include "hal.h"
#include "memory.h"

/*
  Create a file in the root directory of the new FAT32 filesystem
  with the indicated name and size.

  The file will be created contiguous on disk, and the first
  sector of the created file returned.

  The root directory is the start of cluster 2, and clusters are
  assumed to be 4KB in size, to keep things simple.

  Returns first sector of file if successful, or -1 on failure.
*/
long mega65_fat32_create_contiguous_file(char* name, long size, long root_dir_sector, long fat1_sector, long fat2_sector)
{
  unsigned char i;
  unsigned short offset;
  unsigned short clusters;
  unsigned long start_cluster = 0;
  unsigned long next_cluster;
  unsigned long contiguous_clusters = 0;
  unsigned int fat_offset = 0;
  int j;

  clusters = size / 4096;
  if (size & 4095)
    clusters++;

  for (fat_offset = 0; fat_offset <= (fat2_sector - fat1_sector); fat_offset++) {
    mega65_sdcard_readsector(fat1_sector + fat_offset);
    contiguous_clusters = 0;
    start_cluster = 0;

    // Skip any FAT sectors with allocated clusters
    for (j = 0; j < 512; j++)
      if (sector_buffer[j])
        break;
    if (j != 512) {
      continue;
    }

    for (offset = 0; offset < 512; offset += 4) {
      next_cluster = sector_buffer[offset];
      next_cluster |= ((long)sector_buffer[offset + 1] << 8L);
      next_cluster |= ((long)sector_buffer[offset + 2] << 16L);
      next_cluster |= ((long)sector_buffer[offset + 3] << 24L);
      if (!next_cluster) {
        if (!start_cluster) {
          start_cluster = (offset / 4) + fat_offset * (512 / 4);
        }
        contiguous_clusters++;
        if (contiguous_clusters == clusters) {
          // End of chain marker
          sector_buffer[offset + 0] = 0xff;
          sector_buffer[offset + 1] = 0xff;
          sector_buffer[offset + 2] = 0xff;
          sector_buffer[offset + 3] = 0x0f;
          break;
        }
        else {
          // Point to next cluster
          uint32_t the_cluster = (fat_offset * (512 / 4) + (offset / 4)) + 1;
          sector_buffer[offset + 0] = (the_cluster >> 0) & 0xff;
          sector_buffer[offset + 1] = (the_cluster >> 8) & 0xff;
          sector_buffer[offset + 2] = (the_cluster >> 16) & 0xff;
          sector_buffer[offset + 3] = (the_cluster >> 24) & 0xff;
        }
      }
      else {
        if (start_cluster) {
          // write_line("ERROR: Disk space is fragmented. File not created.",0);
          // 	    return 0;
          // Not enough contiguous space in this FAT sector, so try the next
          break;
        }
      }
    }

    if (start_cluster && (contiguous_clusters == clusters))
      break;
    else {
    }
  }
  if ((!start_cluster) || (contiguous_clusters != clusters)) {
    //    write_line("ERROR: Could not find enough free clusters in file system",0);
    return -1;
  }

  // Commit sector to disk (in both copies of FAT)
  mega65_sdcard_writesector(fat1_sector + fat_offset);
  mega65_sdcard_writesector(fat2_sector + fat_offset);

  mega65_sdcard_readsector(root_dir_sector);

  for (offset = 0; offset < 512; offset += 32) {
    if (sector_buffer[offset] > ' ')
      continue;
    else
      break;
  }
  if (offset == 512) {
    //    write_line("ERROR: First sector of root directory already full.",0);
    return -1;
  }

  // Build directory entry
  for (i = 0; i < 32; i++)
    sector_buffer[offset + i] = 0x00;
  for (i = 0; i < 12; i++)
    sector_buffer[offset + i] = name[i];
  sector_buffer[offset + 0x0b] = 0x20; // Archive bit set
  sector_buffer[offset + 0x1A] = start_cluster;
  sector_buffer[offset + 0x1B] = start_cluster >> 8;
  sector_buffer[offset + 0x14] = start_cluster >> 16;
  sector_buffer[offset + 0x15] = start_cluster >> 24;
  sector_buffer[offset + 0x1C] = (size >> 0) & 0xff;
  sector_buffer[offset + 0x1D] = (size >> 8L) & 0xff;
  sector_buffer[offset + 0x1E] = (size >> 16L) & 0xff;
  sector_buffer[offset + 0x1F] = (size >> 24l) & 0xff;

  mega65_sdcard_writesector(root_dir_sector);

  return root_dir_sector + (start_cluster - 2) * 8;
}
