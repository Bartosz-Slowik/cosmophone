/* Place app descriptor at start of .rodata in DROM (single DROM segment for bootloader) */
SECTIONS
{
  .rodata : ALIGN(4)
  {
    KEEP(*(.rodata.esp_app_desc))
  } > drom_seg
}
INSERT BEFORE .rodata
