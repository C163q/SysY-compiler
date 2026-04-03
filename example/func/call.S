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
  addi sp, sp, -64
  sw ra, 48(sp)
  addi t0, sp, 0
  addi t1, sp, 4
  sw t0, 0(t1)
  lw t0, 4(sp)
  sw zero, 0(t0)
  j LBB11_1
LBB11_1:
  lw t0, 4(sp)
  lw t0, 0(t0)
  sw t0, 8(sp)
  lw t0, 8(sp)
  li t1, 10
  slt t0, t0, t1
  sw t0, 12(sp)
  lw t0, 12(sp)
  bnez t0, LBB11_2
  j LBB11_3
LBB11_2:
  call inc
  sw a0, 16(sp)
  addi t0, sp, 20
  addi t1, sp, 24
  sw t0, 0(t1)
  li t0, 1
  lw t1, 24(sp)
  sw t0, 0(t1)
  lw t0, 24(sp)
  lw t0, 0(t0)
  sw t0, 28(sp)
  lw t0, 28(sp)
  li t1, 2
  add t0, t0, t1
  sw t0, 32(sp)
  lw t0, 32(sp)
  lw t1, 24(sp)
  sw t0, 0(t1)
  lw t0, 24(sp)
  lw t0, 0(t0)
  sw t0, 36(sp)
  lw t0, 36(sp)
  mv a0, t0
  call putint
  li t0, 10
  mv a0, t0
  call putch
  call print_a
  lw t0, 4(sp)
  lw t0, 0(t0)
  sw t0, 40(sp)
  lw t0, 40(sp)
  li t1, 1
  add t0, t0, t1
  sw t0, 44(sp)
  lw t0, 44(sp)
  lw t1, 4(sp)
  sw t0, 0(t1)
  j LBB11_1
LBB11_3:
  mv a0, zero
  lw ra, 48(sp)
  addi sp, sp, 64
  ret

