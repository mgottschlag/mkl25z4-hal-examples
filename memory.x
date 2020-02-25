/* Parts taken from https://github.com/therealprof/mkw41z-hal/blob/master/memory.x */

MEMORY
{
  FLASH : ORIGIN = 0x00000000, LENGTH = 128K
  RAM : ORIGIN = 0x1ffff000, LENGTH = 16K
  /* This chip has funny security bits at 0x400 ... */
  FCF : ORIGIN = 0x00000400, LENGTH = 16
}

/* Skip the whole sector with the security bits and let program code start
 * after to prevent nasty mishaps */
_stext = 0x00000800;
