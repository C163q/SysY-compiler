  .data
  .globl a
a:
  .word 10

  .text
  .globl inc
inc:
  addi sp, sp, -16
  sw ra, 12(sp)
  la t0, a
  lw t0, 0(t0)
  sw t0, 0(sp)
  lw t0, 0(sp)
  li t1, 1
  add t0, t0, t1
  sw t0, 4(sp)
  lw t0, 4(sp)
  la t1, a
  sw t0, 0(t1)
  la t0, a
  lw t0, 0(t0)
  sw t0, 8(sp)
  lw t0, 8(sp)
  mv a0, t0
  lw ra, 12(sp)
  addi sp, sp, 16
  ret

  .globl print_a
print_a:
  addi sp, sp, -16
  sw ra, 4(sp)
  la t0, a
  lw t0, 0(t0)
  sw t0, 0(sp)
  lw t0, 0(sp)
  mv a0, t0
  call putint
  li t0, 10
  mv a0, t0
  call putch
  lw ra, 4(sp)
  addi sp, sp, 16
  ret

  .globl main
main:
  addi sp, sp, -48
  sw ra, 40(sp)
  sw zero, 0(sp)
  j LBB11_1
LBB11_1:
  lw t0, 0(sp)
  sw t0, 4(sp)
  lw t0, 4(sp)
  li t1, 10
  slt t0, t0, t1
  sw t0, 8(sp)
  lw t0, 8(sp)
  bnez t0, LBB11_2
  j LBB11_3
LBB11_2:
  call inc
  sw a0, 12(sp)
  li t0, 1
  sw t0, 16(sp)
  lw t0, 16(sp)
  sw t0, 20(sp)
  lw t0, 20(sp)
  li t1, 2
  add t0, t0, t1
  sw t0, 24(sp)
  lw t0, 24(sp)
  sw t0, 16(sp)
  lw t0, 16(sp)
  sw t0, 28(sp)
  lw t0, 28(sp)
  mv a0, t0
  call putint
  li t0, 10
  mv a0, t0
  call putch
  call print_a
  lw t0, 0(sp)
  sw t0, 32(sp)
  lw t0, 32(sp)
  li t1, 1
  add t0, t0, t1
  sw t0, 36(sp)
  lw t0, 36(sp)
  sw t0, 0(sp)
  j LBB11_1
LBB11_3:
  mv a0, zero
  lw ra, 40(sp)
  addi sp, sp, 48
  ret

