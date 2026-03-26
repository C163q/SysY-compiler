  .data
  .text
  .globl main
main:
  addi sp, sp, -32
  sw ra, 24(sp)
  sw zero, 0(sp)
  j LBB9_1
LBB9_1:
  lw t0, 0(sp)
  sw t0, 4(sp)
  lw t0, 4(sp)
  li t1, 10
  slt t0, t0, t1
  sw t0, 8(sp)
  lw t0, 8(sp)
  bnez t0, LBB9_2
  j LBB9_3
LBB9_2:
  lw t0, 0(sp)
  sw t0, 12(sp)
  lw t0, 12(sp)
  li t1, 1
  add t0, t0, t1
  sw t0, 16(sp)
  lw t0, 16(sp)
  sw t0, 0(sp)
  j LBB9_1
LBB9_3:
  lw t0, 0(sp)
  sw t0, 20(sp)
  lw t0, 20(sp)
  mv a0, t0
  lw ra, 24(sp)
  addi sp, sp, 32
  ret

