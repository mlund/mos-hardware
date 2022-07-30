void toggle_rom_write_protect();
void closeall(void);
void close(unsigned char fd);

// Returns file descriptor
unsigned char open(char* filename);

// Read upto one sector of data into the supplied buffer.
// Returns the number of bytes actually read.
unsigned short read512(unsigned char* buffer);

// Change working directory
// (only accepts one directory segment at a time
unsigned char chdir(char* filename);

// Change working directory to the root directory
unsigned char chdirroot(void);
