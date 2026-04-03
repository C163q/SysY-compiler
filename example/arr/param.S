  .data
  .globl init
init:
  .word 0

  .text
  .globl init1d
init1d:
  addi sp, sp, -80
  sw ra, 68(sp)
  addi t0, sp, 0
  addi t1, sp, 4
  sw t0, 0(t1)
  lw t0, 4(sp)
  sw a0, 0(t0)
  addi t0, sp, 8
  addi t1, sp, 12
  sw t0, 0(t1)
  lw t0, 12(sp)
  sw a1, 0(t0)
  addi t0, sp, 16
  addi t1, sp, 20
  sw t0, 0(t1)
  lw t0, 20(sp)
  sw zero, 0(t0)
  j LBB9_1
LBB9_1:
  lw t0, 20(sp)
  lw t0, 0(t0)
  sw t0, 24(sp)
  lw t0, 4(sp)
  lw t0, 0(t0)
  sw t0, 28(sp)
  lw t0, 24(sp)
  lw t1, 28(sp)
  slt t0, t0, t1
  sw t0, 32(sp)
  lw t0, 32(sp)
  bnez t0, LBB9_2
  j LBB9_3
LBB9_2:
  la t0, init
  lw t0, 0(t0)
  sw t0, 36(sp)
  lw t0, 20(sp)
  lw t0, 0(t0)
  sw t0, 40(sp)
  lw t0, 12(sp)
  lw t0, 0(t0)
  sw t0, 44(sp)
  addi t0, sp, 44
  lw t0, 0(t0)
  lw t1, 40(sp)
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  sw t0, 48(sp)
  lw t0, 36(sp)
  lw t1, 48(sp)
  sw t0, 0(t1)
  la t0, init
  lw t0, 0(t0)
  sw t0, 52(sp)
  lw t0, 52(sp)
  li t1, 1
  add t0, t0, t1
  sw t0, 56(sp)
  lw t0, 56(sp)
  la t1, init
  sw t0, 0(t1)
  lw t0, 20(sp)
  lw t0, 0(t0)
  sw t0, 60(sp)
  lw t0, 60(sp)
  li t1, 1
  add t0, t0, t1
  sw t0, 64(sp)
  lw t0, 64(sp)
  lw t1, 20(sp)
  sw t0, 0(t1)
  j LBB9_1
LBB9_3:
  lw ra, 68(sp)
  addi sp, sp, 80
  ret

  .globl init2d
init2d:
  addi sp, sp, -64
  sw ra, 60(sp)
  addi t0, sp, 0
  addi t1, sp, 4
  sw t0, 0(t1)
  lw t0, 4(sp)
  sw a0, 0(t0)
  addi t0, sp, 8
  addi t1, sp, 12
  sw t0, 0(t1)
  lw t0, 12(sp)
  sw a1, 0(t0)
  addi t0, sp, 16
  addi t1, sp, 20
  sw t0, 0(t1)
  lw t0, 20(sp)
  sw zero, 0(t0)
  j LBB10_1
LBB10_1:
  lw t0, 20(sp)
  lw t0, 0(t0)
  sw t0, 24(sp)
  lw t0, 4(sp)
  lw t0, 0(t0)
  sw t0, 28(sp)
  lw t0, 24(sp)
  lw t1, 28(sp)
  slt t0, t0, t1
  sw t0, 32(sp)
  lw t0, 32(sp)
  bnez t0, LBB10_2
  j LBB10_3
LBB10_2:
  lw t0, 20(sp)
  lw t0, 0(t0)
  sw t0, 36(sp)
  lw t0, 12(sp)
  lw t0, 0(t0)
  sw t0, 40(sp)
  addi t0, sp, 40
  lw t0, 0(t0)
  lw t1, 36(sp)
  li t2, 40
  mul t2, t1, t2
  add t0, t0, t2
  sw t0, 44(sp)
  addi t0, sp, 44
  lw t0, 0(t0)
  li t1, 4
  mul t1, zero, t1
  add t0, t0, t1
  sw t0, 48(sp)
  li t0, 10
  mv a0, t0
  lw t0, 48(sp)
  mv a1, t0
  call init1d
  lw t0, 20(sp)
  lw t0, 0(t0)
  sw t0, 52(sp)
  lw t0, 52(sp)
  li t1, 1
  add t0, t0, t1
  sw t0, 56(sp)
  lw t0, 56(sp)
  lw t1, 20(sp)
  sw t0, 0(t1)
  j LBB10_1
LBB10_3:
  lw ra, 60(sp)
  addi sp, sp, 64
  ret

  .globl init3d
init3d:
  addi sp, sp, -64
  sw ra, 60(sp)
  addi t0, sp, 0
  addi t1, sp, 4
  sw t0, 0(t1)
  lw t0, 4(sp)
  sw a0, 0(t0)
  addi t0, sp, 8
  addi t1, sp, 12
  sw t0, 0(t1)
  lw t0, 12(sp)
  sw a1, 0(t0)
  addi t0, sp, 16
  addi t1, sp, 20
  sw t0, 0(t1)
  lw t0, 20(sp)
  sw zero, 0(t0)
  j LBB11_1
LBB11_1:
  lw t0, 20(sp)
  lw t0, 0(t0)
  sw t0, 24(sp)
  lw t0, 4(sp)
  lw t0, 0(t0)
  sw t0, 28(sp)
  lw t0, 24(sp)
  lw t1, 28(sp)
  slt t0, t0, t1
  sw t0, 32(sp)
  lw t0, 32(sp)
  bnez t0, LBB11_2
  j LBB11_3
LBB11_2:
  lw t0, 20(sp)
  lw t0, 0(t0)
  sw t0, 36(sp)
  lw t0, 12(sp)
  lw t0, 0(t0)
  sw t0, 40(sp)
  addi t0, sp, 40
  lw t0, 0(t0)
  lw t1, 36(sp)
  li t2, 400
  mul t2, t1, t2
  add t0, t0, t2
  sw t0, 44(sp)
  addi t0, sp, 44
  lw t0, 0(t0)
  li t1, 40
  mul t1, zero, t1
  add t0, t0, t1
  sw t0, 48(sp)
  li t0, 10
  mv a0, t0
  lw t0, 48(sp)
  mv a1, t0
  call init2d
  lw t0, 20(sp)
  lw t0, 0(t0)
  sw t0, 52(sp)
  lw t0, 52(sp)
  li t1, 1
  add t0, t0, t1
  sw t0, 56(sp)
  lw t0, 56(sp)
  lw t1, 20(sp)
  sw t0, 0(t1)
  j LBB11_1
LBB11_3:
  lw ra, 60(sp)
  addi sp, sp, 64
  ret

  .globl sum1d
sum1d:
  addi sp, sp, -96
  sw ra, 80(sp)
  addi t0, sp, 0
  addi t1, sp, 4
  sw t0, 0(t1)
  lw t0, 4(sp)
  sw a0, 0(t0)
  addi t0, sp, 8
  addi t1, sp, 12
  sw t0, 0(t1)
  lw t0, 12(sp)
  sw a1, 0(t0)
  addi t0, sp, 16
  addi t1, sp, 20
  sw t0, 0(t1)
  lw t0, 20(sp)
  sw zero, 0(t0)
  addi t0, sp, 24
  addi t1, sp, 28
  sw t0, 0(t1)
  lw t0, 28(sp)
  sw zero, 0(t0)
  j LBB12_1
LBB12_1:
  lw t0, 20(sp)
  lw t0, 0(t0)
  sw t0, 32(sp)
  lw t0, 4(sp)
  lw t0, 0(t0)
  sw t0, 36(sp)
  lw t0, 32(sp)
  lw t1, 36(sp)
  slt t0, t0, t1
  sw t0, 40(sp)
  lw t0, 40(sp)
  bnez t0, LBB12_2
  j LBB12_3
LBB12_2:
  lw t0, 28(sp)
  lw t0, 0(t0)
  sw t0, 44(sp)
  lw t0, 20(sp)
  lw t0, 0(t0)
  sw t0, 48(sp)
  lw t0, 12(sp)
  lw t0, 0(t0)
  sw t0, 52(sp)
  addi t0, sp, 52
  lw t0, 0(t0)
  lw t1, 48(sp)
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  sw t0, 56(sp)
  lw t0, 56(sp)
  lw t0, 0(t0)
  sw t0, 60(sp)
  lw t0, 44(sp)
  lw t1, 60(sp)
  add t0, t0, t1
  sw t0, 64(sp)
  lw t0, 64(sp)
  lw t1, 28(sp)
  sw t0, 0(t1)
  lw t0, 20(sp)
  lw t0, 0(t0)
  sw t0, 68(sp)
  lw t0, 68(sp)
  li t1, 1
  add t0, t0, t1
  sw t0, 72(sp)
  lw t0, 72(sp)
  lw t1, 20(sp)
  sw t0, 0(t1)
  j LBB12_1
LBB12_3:
  lw t0, 28(sp)
  lw t0, 0(t0)
  sw t0, 76(sp)
  lw t0, 76(sp)
  mv a0, t0
  lw ra, 80(sp)
  addi sp, sp, 96
  ret

  .globl sum2d
sum2d:
  addi sp, sp, -96
  sw ra, 84(sp)
  addi t0, sp, 0
  addi t1, sp, 4
  sw t0, 0(t1)
  lw t0, 4(sp)
  sw a0, 0(t0)
  addi t0, sp, 8
  addi t1, sp, 12
  sw t0, 0(t1)
  lw t0, 12(sp)
  sw a1, 0(t0)
  addi t0, sp, 16
  addi t1, sp, 20
  sw t0, 0(t1)
  lw t0, 20(sp)
  sw zero, 0(t0)
  addi t0, sp, 24
  addi t1, sp, 28
  sw t0, 0(t1)
  lw t0, 28(sp)
  sw zero, 0(t0)
  j LBB13_1
LBB13_1:
  lw t0, 20(sp)
  lw t0, 0(t0)
  sw t0, 32(sp)
  lw t0, 4(sp)
  lw t0, 0(t0)
  sw t0, 36(sp)
  lw t0, 32(sp)
  lw t1, 36(sp)
  slt t0, t0, t1
  sw t0, 40(sp)
  lw t0, 40(sp)
  bnez t0, LBB13_2
  j LBB13_3
LBB13_2:
  lw t0, 28(sp)
  lw t0, 0(t0)
  sw t0, 44(sp)
  lw t0, 20(sp)
  lw t0, 0(t0)
  sw t0, 48(sp)
  lw t0, 12(sp)
  lw t0, 0(t0)
  sw t0, 52(sp)
  addi t0, sp, 52
  lw t0, 0(t0)
  lw t1, 48(sp)
  li t2, 40
  mul t2, t1, t2
  add t0, t0, t2
  sw t0, 56(sp)
  addi t0, sp, 56
  lw t0, 0(t0)
  li t1, 4
  mul t1, zero, t1
  add t0, t0, t1
  sw t0, 60(sp)
  li t0, 10
  mv a0, t0
  lw t0, 60(sp)
  mv a1, t0
  call sum1d
  sw a0, 64(sp)
  lw t0, 44(sp)
  lw t1, 64(sp)
  add t0, t0, t1
  sw t0, 68(sp)
  lw t0, 68(sp)
  lw t1, 28(sp)
  sw t0, 0(t1)
  lw t0, 20(sp)
  lw t0, 0(t0)
  sw t0, 72(sp)
  lw t0, 72(sp)
  li t1, 1
  add t0, t0, t1
  sw t0, 76(sp)
  lw t0, 76(sp)
  lw t1, 20(sp)
  sw t0, 0(t1)
  j LBB13_1
LBB13_3:
  lw t0, 28(sp)
  lw t0, 0(t0)
  sw t0, 80(sp)
  lw t0, 80(sp)
  mv a0, t0
  lw ra, 84(sp)
  addi sp, sp, 96
  ret

  .globl sum3d
sum3d:
  addi sp, sp, -96
  sw ra, 84(sp)
  addi t0, sp, 0
  addi t1, sp, 4
  sw t0, 0(t1)
  lw t0, 4(sp)
  sw a0, 0(t0)
  addi t0, sp, 8
  addi t1, sp, 12
  sw t0, 0(t1)
  lw t0, 12(sp)
  sw a1, 0(t0)
  addi t0, sp, 16
  addi t1, sp, 20
  sw t0, 0(t1)
  lw t0, 20(sp)
  sw zero, 0(t0)
  addi t0, sp, 24
  addi t1, sp, 28
  sw t0, 0(t1)
  lw t0, 28(sp)
  sw zero, 0(t0)
  j LBB14_1
LBB14_1:
  lw t0, 20(sp)
  lw t0, 0(t0)
  sw t0, 32(sp)
  lw t0, 4(sp)
  lw t0, 0(t0)
  sw t0, 36(sp)
  lw t0, 32(sp)
  lw t1, 36(sp)
  slt t0, t0, t1
  sw t0, 40(sp)
  lw t0, 40(sp)
  bnez t0, LBB14_2
  j LBB14_3
LBB14_2:
  lw t0, 28(sp)
  lw t0, 0(t0)
  sw t0, 44(sp)
  lw t0, 20(sp)
  lw t0, 0(t0)
  sw t0, 48(sp)
  lw t0, 12(sp)
  lw t0, 0(t0)
  sw t0, 52(sp)
  addi t0, sp, 52
  lw t0, 0(t0)
  lw t1, 48(sp)
  li t2, 400
  mul t2, t1, t2
  add t0, t0, t2
  sw t0, 56(sp)
  addi t0, sp, 56
  lw t0, 0(t0)
  li t1, 40
  mul t1, zero, t1
  add t0, t0, t1
  sw t0, 60(sp)
  li t0, 10
  mv a0, t0
  lw t0, 60(sp)
  mv a1, t0
  call sum2d
  sw a0, 64(sp)
  lw t0, 44(sp)
  lw t1, 64(sp)
  add t0, t0, t1
  sw t0, 68(sp)
  lw t0, 68(sp)
  lw t1, 28(sp)
  sw t0, 0(t1)
  lw t0, 20(sp)
  lw t0, 0(t0)
  sw t0, 72(sp)
  lw t0, 72(sp)
  li t1, 1
  add t0, t0, t1
  sw t0, 76(sp)
  lw t0, 76(sp)
  lw t1, 20(sp)
  sw t0, 0(t1)
  j LBB14_1
LBB14_3:
  lw t0, 28(sp)
  lw t0, 0(t0)
  sw t0, 80(sp)
  lw t0, 80(sp)
  mv a0, t0
  lw ra, 84(sp)
  addi sp, sp, 96
  ret

  .globl main
main:
  li t1, -4080
  add sp, sp, t1
  li t1, 4076
  add t1, sp, t1
  sw ra, 0(t1)
  addi t0, sp, 0
  li t1, 4000
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4000
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 400
  mul t1, zero, t1
  add t0, t0, t1
  li t1, 4004
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 10
  mv a0, t0
  li t0, 4004
  add t0, sp, t0
  lw t0, 0(t0)
  mv a1, t0
  call init3d
  li t0, 4008
  add t0, sp, t0
  li t1, 4012
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4000
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 400
  mul t1, zero, t1
  add t0, t0, t1
  li t1, 4016
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 10
  mv a0, t0
  li t0, 4016
  add t0, sp, t0
  lw t0, 0(t0)
  mv a1, t0
  call sum3d
  li t0, 4020
  add t0, sp, t0
  sw a0, 0(t0)
  li t0, 4020
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 4012
  add t1, sp, t1
  lw t1, 0(t1)
  sw t0, 0(t1)
  li t0, 4012
  add t0, sp, t0
  lw t0, 0(t0)
  lw t0, 0(t0)
  li t1, 4024
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4000
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 1
  li t2, 400
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 4028
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4028
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 40
  mul t1, zero, t1
  add t0, t0, t1
  li t1, 4032
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 10
  mv a0, t0
  li t0, 4032
  add t0, sp, t0
  lw t0, 0(t0)
  mv a1, t0
  call sum2d
  li t0, 4036
  add t0, sp, t0
  sw a0, 0(t0)
  li t0, 4024
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 4036
  add t1, sp, t1
  lw t1, 0(t1)
  add t0, t0, t1
  li t2, 4040
  add t2, sp, t2
  sw t0, 0(t2)
  li t0, 4040
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 4012
  add t1, sp, t1
  lw t1, 0(t1)
  sw t0, 0(t1)
  li t0, 4012
  add t0, sp, t0
  lw t0, 0(t0)
  lw t0, 0(t0)
  li t1, 4044
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4000
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 2
  li t2, 400
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 4048
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4048
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 3
  li t2, 40
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 4052
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4052
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 4
  mul t1, zero, t1
  add t0, t0, t1
  li t1, 4056
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 10
  mv a0, t0
  li t0, 4056
  add t0, sp, t0
  lw t0, 0(t0)
  mv a1, t0
  call sum1d
  li t0, 4060
  add t0, sp, t0
  sw a0, 0(t0)
  li t0, 4044
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 4060
  add t1, sp, t1
  lw t1, 0(t1)
  add t0, t0, t1
  li t2, 4064
  add t2, sp, t2
  sw t0, 0(t2)
  li t0, 4064
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 4012
  add t1, sp, t1
  lw t1, 0(t1)
  sw t0, 0(t1)
  li t0, 4012
  add t0, sp, t0
  lw t0, 0(t0)
  lw t0, 0(t0)
  li t1, 4068
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4068
  add t0, sp, t0
  lw t0, 0(t0)
  mv a0, t0
  call putint
  li t0, 10
  mv a0, t0
  call putch
  li t0, 4012
  add t0, sp, t0
  lw t0, 0(t0)
  lw t0, 0(t0)
  li t1, 4072
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4072
  add t0, sp, t0
  lw t0, 0(t0)
  mv a0, t0
  li t1, 4076
  add t1, sp, t1
  lw ra, 0(t1)
  li t1, 4080
  add sp, sp, t1
  ret

