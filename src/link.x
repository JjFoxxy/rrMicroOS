MEMORY
{
  RAM : ORIGIN = 0x40000000, LENGTH = 2M
}

/* The entry point is the reset handler */
ENTRY(_start);

SECTIONS
{
  .vector_table ORIGIN(RAM) :
  {
    KEEP(*(.vector_table.reset_vector));
  } > RAM
  .text :
  {
    *(.text .text.*);
  } > RAM
}