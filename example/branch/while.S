  .data
  .text
  .globl main
main:
  addi sp, sp, -32
  sw ra, 28(sp)
  addi t0, sp, 0
  addi t1, sp, 4
  sw t0, 0(t1)
  lw t0, 4(sp)
  sw zero, 0(t0)
  j LBB9_1
LBB9_1:
  lw t0, 4(sp)
  lw t0, 0(t0)
  sw t0, 8(sp)
  lw t0, 8(sp)
  li t1, 10
  slt t0, t0, t1
  sw t0, 12(sp)
  lw t0, 12(sp)
  bnez t0, LBB9_2
  j LBB9_3
LBB9_2:
  lw t0, 4(sp)
  lw t0, 0(t0)
  sw t0, 16(sp)
  lw t0, 16(sp)
  li t1, 1
  add t0, t0, t1
  sw t0, 20(sp)
  lw t0, 20(sp)
  lw t1, 4(sp)
  sw t0, 0(t1)
  j LBB9_1
LBB9_3:
  lw t0, 4(sp)
  lw t0, 0(t0)
  sw t0, 24(sp)
  lw t0, 24(sp)
  mv a0, t0
  lw ra, 28(sp)
  addi sp, sp, 32
  ret

