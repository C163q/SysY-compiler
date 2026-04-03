  .data
  .globl n
n:
  .zero 4

  .text
  .globl swap
swap:
  addi sp, sp, -96
  sw ra, 92(sp)
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
  sw a2, 0(t0)
  addi t0, sp, 24
  addi t1, sp, 28
  sw t0, 0(t1)
  lw t0, 12(sp)
  lw t0, 0(t0)
  sw t0, 32(sp)
  lw t0, 4(sp)
  lw t0, 0(t0)
  sw t0, 36(sp)
  addi t0, sp, 36
  lw t0, 0(t0)
  lw t1, 32(sp)
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  sw t0, 40(sp)
  lw t0, 40(sp)
  lw t0, 0(t0)
  sw t0, 44(sp)
  lw t0, 44(sp)
  lw t1, 28(sp)
  sw t0, 0(t1)
  lw t0, 20(sp)
  lw t0, 0(t0)
  sw t0, 48(sp)
  lw t0, 4(sp)
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
  lw t0, 12(sp)
  lw t0, 0(t0)
  sw t0, 64(sp)
  lw t0, 4(sp)
  lw t0, 0(t0)
  sw t0, 68(sp)
  addi t0, sp, 68
  lw t0, 0(t0)
  lw t1, 64(sp)
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  sw t0, 72(sp)
  lw t0, 60(sp)
  lw t1, 72(sp)
  sw t0, 0(t1)
  lw t0, 28(sp)
  lw t0, 0(t0)
  sw t0, 76(sp)
  lw t0, 20(sp)
  lw t0, 0(t0)
  sw t0, 80(sp)
  lw t0, 4(sp)
  lw t0, 0(t0)
  sw t0, 84(sp)
  addi t0, sp, 84
  lw t0, 0(t0)
  lw t1, 80(sp)
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  sw t0, 88(sp)
  lw t0, 76(sp)
  lw t1, 88(sp)
  sw t0, 0(t1)
  mv a0, zero
  lw ra, 92(sp)
  addi sp, sp, 96
  ret

  .globl heap_ajust
heap_ajust:
  addi sp, sp, -224
  sw ra, 220(sp)
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
  sw a2, 0(t0)
  addi t0, sp, 24
  addi t1, sp, 28
  sw t0, 0(t1)
  lw t0, 12(sp)
  lw t0, 0(t0)
  sw t0, 32(sp)
  lw t0, 32(sp)
  lw t1, 28(sp)
  sw t0, 0(t1)
  addi t0, sp, 36
  addi t1, sp, 40
  sw t0, 0(t1)
  lw t0, 28(sp)
  lw t0, 0(t0)
  sw t0, 44(sp)
  lw t0, 44(sp)
  li t1, 2
  mul t0, t0, t1
  sw t0, 48(sp)
  lw t0, 48(sp)
  li t1, 1
  add t0, t0, t1
  sw t0, 52(sp)
  lw t0, 52(sp)
  lw t1, 40(sp)
  sw t0, 0(t1)
  j LBB10_1
LBB10_1:
  lw t0, 40(sp)
  lw t0, 0(t0)
  sw t0, 56(sp)
  lw t0, 20(sp)
  lw t0, 0(t0)
  sw t0, 60(sp)
  lw t0, 60(sp)
  li t1, 1
  add t0, t0, t1
  sw t0, 64(sp)
  lw t0, 56(sp)
  lw t1, 64(sp)
  slt t0, t0, t1
  sw t0, 68(sp)
  lw t0, 68(sp)
  bnez t0, LBB10_2
  j LBB10_11
LBB10_2:
  addi t0, sp, 72
  addi t1, sp, 76
  sw t0, 0(t1)
  lw t0, 76(sp)
  sw zero, 0(t0)
  lw t0, 40(sp)
  lw t0, 0(t0)
  sw t0, 80(sp)
  lw t0, 20(sp)
  lw t0, 0(t0)
  sw t0, 84(sp)
  lw t0, 80(sp)
  lw t1, 84(sp)
  slt t0, t0, t1
  sw t0, 88(sp)
  lw t0, 88(sp)
  snez t1, t0
  sw t1, 92(sp)
  lw t0, 92(sp)
  bnez t0, LBB10_3
  j LBB10_4
LBB10_3:
  lw t0, 40(sp)
  lw t0, 0(t0)
  sw t0, 96(sp)
  lw t0, 4(sp)
  lw t0, 0(t0)
  sw t0, 100(sp)
  addi t0, sp, 100
  lw t0, 0(t0)
  lw t1, 96(sp)
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  sw t0, 104(sp)
  lw t0, 104(sp)
  lw t0, 0(t0)
  sw t0, 108(sp)
  lw t0, 40(sp)
  lw t0, 0(t0)
  sw t0, 112(sp)
  lw t0, 112(sp)
  li t1, 1
  add t0, t0, t1
  sw t0, 116(sp)
  lw t0, 4(sp)
  lw t0, 0(t0)
  sw t0, 120(sp)
  addi t0, sp, 120
  lw t0, 0(t0)
  lw t1, 116(sp)
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  sw t0, 124(sp)
  lw t0, 124(sp)
  lw t0, 0(t0)
  sw t0, 128(sp)
  lw t0, 108(sp)
  lw t1, 128(sp)
  slt t0, t0, t1
  sw t0, 132(sp)
  lw t0, 132(sp)
  snez t1, t0
  sw t1, 136(sp)
  lw t0, 136(sp)
  lw t1, 76(sp)
  sw t0, 0(t1)
  j LBB10_4
LBB10_4:
  lw t0, 76(sp)
  lw t0, 0(t0)
  sw t0, 140(sp)
  lw t0, 140(sp)
  bnez t0, LBB10_5
  j LBB10_6
LBB10_5:
  lw t0, 40(sp)
  lw t0, 0(t0)
  sw t0, 144(sp)
  lw t0, 144(sp)
  li t1, 1
  add t0, t0, t1
  sw t0, 148(sp)
  lw t0, 148(sp)
  lw t1, 40(sp)
  sw t0, 0(t1)
  j LBB10_6
LBB10_6:
  lw t0, 28(sp)
  lw t0, 0(t0)
  sw t0, 152(sp)
  lw t0, 4(sp)
  lw t0, 0(t0)
  sw t0, 156(sp)
  addi t0, sp, 156
  lw t0, 0(t0)
  lw t1, 152(sp)
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  sw t0, 160(sp)
  lw t0, 160(sp)
  lw t0, 0(t0)
  sw t0, 164(sp)
  lw t0, 40(sp)
  lw t0, 0(t0)
  sw t0, 168(sp)
  lw t0, 4(sp)
  lw t0, 0(t0)
  sw t0, 172(sp)
  addi t0, sp, 172
  lw t0, 0(t0)
  lw t1, 168(sp)
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  sw t0, 176(sp)
  lw t0, 176(sp)
  lw t0, 0(t0)
  sw t0, 180(sp)
  lw t0, 164(sp)
  lw t1, 180(sp)
  sgt t0, t0, t1
  sw t0, 184(sp)
  lw t0, 184(sp)
  bnez t0, LBB10_7
  j LBB10_9
LBB10_7:
  mv a0, zero
  lw ra, 220(sp)
  addi sp, sp, 224
  ret
LBB10_8:
  j LBB10_10
LBB10_9:
  lw t0, 4(sp)
  lw t0, 0(t0)
  sw t0, 188(sp)
  lw t0, 28(sp)
  lw t0, 0(t0)
  sw t0, 192(sp)
  lw t0, 40(sp)
  lw t0, 0(t0)
  sw t0, 196(sp)
  lw t0, 188(sp)
  mv a0, t0
  lw t0, 192(sp)
  mv a1, t0
  lw t0, 196(sp)
  mv a2, t0
  call swap
  sw a0, 200(sp)
  lw t0, 200(sp)
  lw t1, 28(sp)
  sw t0, 0(t1)
  lw t0, 40(sp)
  lw t0, 0(t0)
  sw t0, 204(sp)
  lw t0, 204(sp)
  lw t1, 28(sp)
  sw t0, 0(t1)
  lw t0, 28(sp)
  lw t0, 0(t0)
  sw t0, 208(sp)
  lw t0, 208(sp)
  li t1, 2
  mul t0, t0, t1
  sw t0, 212(sp)
  lw t0, 212(sp)
  li t1, 1
  add t0, t0, t1
  sw t0, 216(sp)
  lw t0, 216(sp)
  lw t1, 40(sp)
  sw t0, 0(t1)
  j LBB10_10
LBB10_10:
  j LBB10_1
LBB10_11:
  mv a0, zero
  lw ra, 220(sp)
  addi sp, sp, 224
  ret

  .globl heap_sort
heap_sort:
  addi sp, sp, -176
  sw ra, 160(sp)
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
  addi t0, sp, 24
  addi t1, sp, 28
  sw t0, 0(t1)
  lw t0, 12(sp)
  lw t0, 0(t0)
  sw t0, 32(sp)
  lw t0, 32(sp)
  li t1, 2
  div t0, t0, t1
  sw t0, 36(sp)
  lw t0, 36(sp)
  li t1, 1
  sub t0, t0, t1
  sw t0, 40(sp)
  lw t0, 40(sp)
  lw t1, 20(sp)
  sw t0, 0(t1)
  j LBB11_1
LBB11_1:
  lw t0, 20(sp)
  lw t0, 0(t0)
  sw t0, 44(sp)
  li t0, 1
  sub t1, zero, t0
  sw t1, 48(sp)
  lw t0, 44(sp)
  lw t1, 48(sp)
  sgt t0, t0, t1
  sw t0, 52(sp)
  lw t0, 52(sp)
  bnez t0, LBB11_2
  j LBB11_3
LBB11_2:
  lw t0, 12(sp)
  lw t0, 0(t0)
  sw t0, 56(sp)
  lw t0, 56(sp)
  li t1, 1
  sub t0, t0, t1
  sw t0, 60(sp)
  lw t0, 60(sp)
  lw t1, 28(sp)
  sw t0, 0(t1)
  lw t0, 4(sp)
  lw t0, 0(t0)
  sw t0, 64(sp)
  lw t0, 20(sp)
  lw t0, 0(t0)
  sw t0, 68(sp)
  lw t0, 28(sp)
  lw t0, 0(t0)
  sw t0, 72(sp)
  lw t0, 64(sp)
  mv a0, t0
  lw t0, 68(sp)
  mv a1, t0
  lw t0, 72(sp)
  mv a2, t0
  call heap_ajust
  sw a0, 76(sp)
  lw t0, 76(sp)
  lw t1, 28(sp)
  sw t0, 0(t1)
  lw t0, 20(sp)
  lw t0, 0(t0)
  sw t0, 80(sp)
  lw t0, 80(sp)
  li t1, 1
  sub t0, t0, t1
  sw t0, 84(sp)
  lw t0, 84(sp)
  lw t1, 20(sp)
  sw t0, 0(t1)
  j LBB11_1
LBB11_3:
  lw t0, 12(sp)
  lw t0, 0(t0)
  sw t0, 88(sp)
  lw t0, 88(sp)
  li t1, 1
  sub t0, t0, t1
  sw t0, 92(sp)
  lw t0, 92(sp)
  lw t1, 20(sp)
  sw t0, 0(t1)
  j LBB11_4
LBB11_4:
  lw t0, 20(sp)
  lw t0, 0(t0)
  sw t0, 96(sp)
  lw t0, 96(sp)
  sgt t0, t0, zero
  sw t0, 100(sp)
  lw t0, 100(sp)
  bnez t0, LBB11_5
  j LBB11_6
LBB11_5:
  addi t0, sp, 104
  addi t1, sp, 108
  sw t0, 0(t1)
  lw t0, 108(sp)
  sw zero, 0(t0)
  lw t0, 4(sp)
  lw t0, 0(t0)
  sw t0, 112(sp)
  lw t0, 108(sp)
  lw t0, 0(t0)
  sw t0, 116(sp)
  lw t0, 20(sp)
  lw t0, 0(t0)
  sw t0, 120(sp)
  lw t0, 112(sp)
  mv a0, t0
  lw t0, 116(sp)
  mv a1, t0
  lw t0, 120(sp)
  mv a2, t0
  call swap
  sw a0, 124(sp)
  lw t0, 124(sp)
  lw t1, 28(sp)
  sw t0, 0(t1)
  lw t0, 20(sp)
  lw t0, 0(t0)
  sw t0, 128(sp)
  lw t0, 128(sp)
  li t1, 1
  sub t0, t0, t1
  sw t0, 132(sp)
  lw t0, 132(sp)
  lw t1, 28(sp)
  sw t0, 0(t1)
  lw t0, 4(sp)
  lw t0, 0(t0)
  sw t0, 136(sp)
  lw t0, 108(sp)
  lw t0, 0(t0)
  sw t0, 140(sp)
  lw t0, 28(sp)
  lw t0, 0(t0)
  sw t0, 144(sp)
  lw t0, 136(sp)
  mv a0, t0
  lw t0, 140(sp)
  mv a1, t0
  lw t0, 144(sp)
  mv a2, t0
  call heap_ajust
  sw a0, 148(sp)
  lw t0, 148(sp)
  lw t1, 28(sp)
  sw t0, 0(t1)
  lw t0, 20(sp)
  lw t0, 0(t0)
  sw t0, 152(sp)
  lw t0, 152(sp)
  li t1, 1
  sub t0, t0, t1
  sw t0, 156(sp)
  lw t0, 156(sp)
  lw t1, 20(sp)
  sw t0, 0(t1)
  j LBB11_4
LBB11_6:
  mv a0, zero
  lw ra, 160(sp)
  addi sp, sp, 176
  ret

  .globl main
main:
  addi sp, sp, -160
  sw ra, 152(sp)
  li t0, 10
  la t1, n
  sw t0, 0(t1)
  addi t0, sp, 0
  addi t1, sp, 40
  sw t0, 0(t1)
  addi t0, sp, 40
  lw t0, 0(t0)
  li t1, 4
  mul t1, zero, t1
  add t0, t0, t1
  sw t0, 44(sp)
  li t0, 4
  lw t1, 44(sp)
  sw t0, 0(t1)
  addi t0, sp, 40
  lw t0, 0(t0)
  li t1, 1
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  sw t0, 48(sp)
  li t0, 3
  lw t1, 48(sp)
  sw t0, 0(t1)
  addi t0, sp, 40
  lw t0, 0(t0)
  li t1, 2
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  sw t0, 52(sp)
  li t0, 9
  lw t1, 52(sp)
  sw t0, 0(t1)
  addi t0, sp, 40
  lw t0, 0(t0)
  li t1, 3
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  sw t0, 56(sp)
  li t0, 2
  lw t1, 56(sp)
  sw t0, 0(t1)
  addi t0, sp, 40
  lw t0, 0(t0)
  li t1, 4
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  sw t0, 60(sp)
  lw t0, 60(sp)
  sw zero, 0(t0)
  addi t0, sp, 40
  lw t0, 0(t0)
  li t1, 5
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  sw t0, 64(sp)
  li t0, 1
  lw t1, 64(sp)
  sw t0, 0(t1)
  addi t0, sp, 40
  lw t0, 0(t0)
  li t1, 6
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  sw t0, 68(sp)
  li t0, 6
  lw t1, 68(sp)
  sw t0, 0(t1)
  addi t0, sp, 40
  lw t0, 0(t0)
  li t1, 7
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  sw t0, 72(sp)
  li t0, 5
  lw t1, 72(sp)
  sw t0, 0(t1)
  addi t0, sp, 40
  lw t0, 0(t0)
  li t1, 8
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  sw t0, 76(sp)
  li t0, 7
  lw t1, 76(sp)
  sw t0, 0(t1)
  addi t0, sp, 40
  lw t0, 0(t0)
  li t1, 9
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  sw t0, 80(sp)
  li t0, 8
  lw t1, 80(sp)
  sw t0, 0(t1)
  addi t0, sp, 84
  addi t1, sp, 88
  sw t0, 0(t1)
  lw t0, 88(sp)
  sw zero, 0(t0)
  addi t0, sp, 40
  lw t0, 0(t0)
  li t1, 4
  mul t1, zero, t1
  add t0, t0, t1
  sw t0, 92(sp)
  la t0, n
  lw t0, 0(t0)
  sw t0, 96(sp)
  lw t0, 92(sp)
  mv a0, t0
  lw t0, 96(sp)
  mv a1, t0
  call heap_sort
  sw a0, 100(sp)
  lw t0, 100(sp)
  lw t1, 88(sp)
  sw t0, 0(t1)
  j LBB12_1
LBB12_1:
  lw t0, 88(sp)
  lw t0, 0(t0)
  sw t0, 104(sp)
  la t0, n
  lw t0, 0(t0)
  sw t0, 108(sp)
  lw t0, 104(sp)
  lw t1, 108(sp)
  slt t0, t0, t1
  sw t0, 112(sp)
  lw t0, 112(sp)
  bnez t0, LBB12_2
  j LBB12_3
LBB12_2:
  addi t0, sp, 116
  addi t1, sp, 120
  sw t0, 0(t1)
  lw t0, 88(sp)
  lw t0, 0(t0)
  sw t0, 124(sp)
  addi t0, sp, 40
  lw t0, 0(t0)
  lw t1, 124(sp)
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  sw t0, 128(sp)
  lw t0, 128(sp)
  lw t0, 0(t0)
  sw t0, 132(sp)
  lw t0, 132(sp)
  lw t1, 120(sp)
  sw t0, 0(t1)
  lw t0, 120(sp)
  lw t0, 0(t0)
  sw t0, 136(sp)
  lw t0, 136(sp)
  mv a0, t0
  call putint
  li t0, 10
  lw t1, 120(sp)
  sw t0, 0(t1)
  lw t0, 120(sp)
  lw t0, 0(t0)
  sw t0, 140(sp)
  lw t0, 140(sp)
  mv a0, t0
  call putch
  lw t0, 88(sp)
  lw t0, 0(t0)
  sw t0, 144(sp)
  lw t0, 144(sp)
  li t1, 1
  add t0, t0, t1
  sw t0, 148(sp)
  lw t0, 148(sp)
  lw t1, 88(sp)
  sw t0, 0(t1)
  j LBB12_1
LBB12_3:
  mv a0, zero
  lw ra, 152(sp)
  addi sp, sp, 160
  ret

