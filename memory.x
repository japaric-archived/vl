MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 128K
  RAM : ORIGIN = 0x20000000, LENGTH = 8K
}

SECTIONS
{
  .text ORIGIN(FLASH) :
  {
    /* Vector table */
    LONG(ORIGIN(RAM) + LENGTH(RAM));
    LONG(_start + 1);
    KEEP(*(.rodata._EXCEPTIONS));
    __exceptions = .;
    KEEP(*(.rodata._INTERRUPTS));
    __interrupts = .;

    /* Entry point */
    _start = .;
    KEEP(*(.text._start));

    *(.text.*);
    *(.rodata.*);
  } > FLASH

  .bss : ALIGN(4)
  {
    _sbss = .;
    *(.bss.*);
    _ebss = ALIGN(4);
  } > RAM

  .data : ALIGN(4)
  {
    _sdata = .;
    *(.data.*);
    _edata = ALIGN(4);
  } > RAM AT > FLASH

  _sidata = LOADADDR(.data);

  /DISCARD/ :
  {
    *(.ARM.exidx.*)
    *(.ARM.extab.*)
  }
}

ASSERT(__exceptions - ORIGIN(FLASH) == 0x40,
       "you must define the _EXCEPTIONS symbol");

ASSERT(__interrupts - ORIGIN(FLASH) == 0x134,
       "you must define the _INTERRUPTS symbol");
