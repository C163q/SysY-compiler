  .data
  .text
  .globl main
main:
  addi sp, sp, -16
  sw ra, 8(sp)
  li t0, 4
  add t1, zero, t0
  sw t1, 0(sp)
  lw t0, 0(sp)
  li t1, 1
  sub t0, t0, t1
  sw t0, 4(sp)
  lw t0, 4(sp)
  mv a0, t0
  lw ra, 8(sp)
  addi sp, sp, 16
  ret

