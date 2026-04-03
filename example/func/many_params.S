  .data
  .text
  .globl sum
sum:
  addi sp, sp, -128
  sw ra, 124(sp)
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
  lw t0, 28(sp)
  sw a3, 0(t0)
  addi t0, sp, 32
  addi t1, sp, 36
  sw t0, 0(t1)
  lw t0, 36(sp)
  sw a4, 0(t0)
  addi t0, sp, 40
  addi t1, sp, 44
  sw t0, 0(t1)
  lw t0, 44(sp)
  sw a5, 0(t0)
  addi t0, sp, 48
  addi t1, sp, 52
  sw t0, 0(t1)
  lw t0, 52(sp)
  sw a6, 0(t0)
  addi t0, sp, 56
  addi t1, sp, 60
  sw t0, 0(t1)
  lw t0, 60(sp)
  sw a7, 0(t0)
  lw t0, 4(sp)
  lw t0, 0(t0)
  sw t0, 64(sp)
  lw t0, 12(sp)
  lw t0, 0(t0)
  sw t0, 68(sp)
  lw t0, 64(sp)
  lw t1, 68(sp)
  add t0, t0, t1
  sw t0, 72(sp)
  lw t0, 20(sp)
  lw t0, 0(t0)
  sw t0, 76(sp)
  lw t0, 72(sp)
  lw t1, 76(sp)
  add t0, t0, t1
  sw t0, 80(sp)
  lw t0, 28(sp)
  lw t0, 0(t0)
  sw t0, 84(sp)
  lw t0, 80(sp)
  lw t1, 84(sp)
  add t0, t0, t1
  sw t0, 88(sp)
  lw t0, 36(sp)
  lw t0, 0(t0)
  sw t0, 92(sp)
  lw t0, 88(sp)
  lw t1, 92(sp)
  add t0, t0, t1
  sw t0, 96(sp)
  lw t0, 44(sp)
  lw t0, 0(t0)
  sw t0, 100(sp)
  lw t0, 96(sp)
  lw t1, 100(sp)
  add t0, t0, t1
  sw t0, 104(sp)
  lw t0, 52(sp)
  lw t0, 0(t0)
  sw t0, 108(sp)
  lw t0, 104(sp)
  lw t1, 108(sp)
  add t0, t0, t1
  sw t0, 112(sp)
  lw t0, 60(sp)
  lw t0, 0(t0)
  sw t0, 116(sp)
  lw t0, 112(sp)
  lw t1, 116(sp)
  add t0, t0, t1
  sw t0, 120(sp)
  lw t0, 120(sp)
  mv a0, t0
  lw ra, 124(sp)
  addi sp, sp, 128
  ret

  .globl sum2
sum2:
  addi sp, sp, -256
  sw ra, 252(sp)
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
  lw t0, 28(sp)
  sw a3, 0(t0)
  addi t0, sp, 32
  addi t1, sp, 36
  sw t0, 0(t1)
  lw t0, 36(sp)
  sw a4, 0(t0)
  addi t0, sp, 40
  addi t1, sp, 44
  sw t0, 0(t1)
  lw t0, 44(sp)
  sw a5, 0(t0)
  addi t0, sp, 48
  addi t1, sp, 52
  sw t0, 0(t1)
  lw t0, 52(sp)
  sw a6, 0(t0)
  addi t0, sp, 56
  addi t1, sp, 60
  sw t0, 0(t1)
  lw t0, 60(sp)
  sw a7, 0(t0)
  addi t0, sp, 64
  addi t1, sp, 68
  sw t0, 0(t1)
  lw t0, 256(sp)
  lw t1, 68(sp)
  sw t0, 0(t1)
  addi t0, sp, 72
  addi t1, sp, 76
  sw t0, 0(t1)
  lw t0, 260(sp)
  lw t1, 76(sp)
  sw t0, 0(t1)
  addi t0, sp, 80
  addi t1, sp, 84
  sw t0, 0(t1)
  lw t0, 264(sp)
  lw t1, 84(sp)
  sw t0, 0(t1)
  addi t0, sp, 88
  addi t1, sp, 92
  sw t0, 0(t1)
  lw t0, 268(sp)
  lw t1, 92(sp)
  sw t0, 0(t1)
  addi t0, sp, 96
  addi t1, sp, 100
  sw t0, 0(t1)
  lw t0, 272(sp)
  lw t1, 100(sp)
  sw t0, 0(t1)
  addi t0, sp, 104
  addi t1, sp, 108
  sw t0, 0(t1)
  lw t0, 276(sp)
  lw t1, 108(sp)
  sw t0, 0(t1)
  addi t0, sp, 112
  addi t1, sp, 116
  sw t0, 0(t1)
  lw t0, 280(sp)
  lw t1, 116(sp)
  sw t0, 0(t1)
  addi t0, sp, 120
  addi t1, sp, 124
  sw t0, 0(t1)
  lw t0, 284(sp)
  lw t1, 124(sp)
  sw t0, 0(t1)
  lw t0, 4(sp)
  lw t0, 0(t0)
  sw t0, 128(sp)
  lw t0, 12(sp)
  lw t0, 0(t0)
  sw t0, 132(sp)
  lw t0, 128(sp)
  lw t1, 132(sp)
  add t0, t0, t1
  sw t0, 136(sp)
  lw t0, 20(sp)
  lw t0, 0(t0)
  sw t0, 140(sp)
  lw t0, 136(sp)
  lw t1, 140(sp)
  add t0, t0, t1
  sw t0, 144(sp)
  lw t0, 28(sp)
  lw t0, 0(t0)
  sw t0, 148(sp)
  lw t0, 144(sp)
  lw t1, 148(sp)
  add t0, t0, t1
  sw t0, 152(sp)
  lw t0, 36(sp)
  lw t0, 0(t0)
  sw t0, 156(sp)
  lw t0, 152(sp)
  lw t1, 156(sp)
  add t0, t0, t1
  sw t0, 160(sp)
  lw t0, 44(sp)
  lw t0, 0(t0)
  sw t0, 164(sp)
  lw t0, 160(sp)
  lw t1, 164(sp)
  add t0, t0, t1
  sw t0, 168(sp)
  lw t0, 52(sp)
  lw t0, 0(t0)
  sw t0, 172(sp)
  lw t0, 168(sp)
  lw t1, 172(sp)
  add t0, t0, t1
  sw t0, 176(sp)
  lw t0, 60(sp)
  lw t0, 0(t0)
  sw t0, 180(sp)
  lw t0, 176(sp)
  lw t1, 180(sp)
  add t0, t0, t1
  sw t0, 184(sp)
  lw t0, 68(sp)
  lw t0, 0(t0)
  sw t0, 188(sp)
  lw t0, 184(sp)
  lw t1, 188(sp)
  add t0, t0, t1
  sw t0, 192(sp)
  lw t0, 76(sp)
  lw t0, 0(t0)
  sw t0, 196(sp)
  lw t0, 192(sp)
  lw t1, 196(sp)
  add t0, t0, t1
  sw t0, 200(sp)
  lw t0, 84(sp)
  lw t0, 0(t0)
  sw t0, 204(sp)
  lw t0, 200(sp)
  lw t1, 204(sp)
  add t0, t0, t1
  sw t0, 208(sp)
  lw t0, 92(sp)
  lw t0, 0(t0)
  sw t0, 212(sp)
  lw t0, 208(sp)
  lw t1, 212(sp)
  add t0, t0, t1
  sw t0, 216(sp)
  lw t0, 100(sp)
  lw t0, 0(t0)
  sw t0, 220(sp)
  lw t0, 216(sp)
  lw t1, 220(sp)
  add t0, t0, t1
  sw t0, 224(sp)
  lw t0, 108(sp)
  lw t0, 0(t0)
  sw t0, 228(sp)
  lw t0, 224(sp)
  lw t1, 228(sp)
  add t0, t0, t1
  sw t0, 232(sp)
  lw t0, 116(sp)
  lw t0, 0(t0)
  sw t0, 236(sp)
  lw t0, 232(sp)
  lw t1, 236(sp)
  add t0, t0, t1
  sw t0, 240(sp)
  lw t0, 124(sp)
  lw t0, 0(t0)
  sw t0, 244(sp)
  lw t0, 240(sp)
  lw t1, 244(sp)
  add t0, t0, t1
  sw t0, 248(sp)
  lw t0, 248(sp)
  mv a0, t0
  lw ra, 252(sp)
  addi sp, sp, 256
  ret

  .globl main
main:
  addi sp, sp, -48
  sw ra, 36(sp)
  addi t0, sp, 0
  addi t1, sp, 4
  sw t0, 0(t1)
  li t0, 1
  mv a0, t0
  li t0, 2
  mv a1, t0
  li t0, 3
  mv a2, t0
  li t0, 4
  mv a3, t0
  li t0, 5
  mv a4, t0
  li t0, 6
  mv a5, t0
  li t0, 7
  mv a6, t0
  li t0, 8
  mv a7, t0
  call sum
  sw a0, 8(sp)
  lw t0, 8(sp)
  lw t1, 4(sp)
  sw t0, 0(t1)
  addi t0, sp, 12
  addi t1, sp, 16
  sw t0, 0(t1)
  addi sp, sp, -32
  li t0, 1
  mv a0, t0
  li t0, 2
  mv a1, t0
  li t0, 3
  mv a2, t0
  li t0, 4
  mv a3, t0
  li t0, 5
  mv a4, t0
  li t0, 6
  mv a5, t0
  li t0, 7
  mv a6, t0
  li t0, 8
  mv a7, t0
  li t0, 9
  sw t0, 0(sp)
  li t0, 10
  sw t0, 4(sp)
  li t0, 11
  sw t0, 8(sp)
  li t0, 12
  sw t0, 12(sp)
  li t0, 13
  sw t0, 16(sp)
  li t0, 14
  sw t0, 20(sp)
  li t0, 15
  sw t0, 24(sp)
  li t0, 16
  sw t0, 28(sp)
  call sum2
  addi sp, sp, 32
  sw a0, 20(sp)
  lw t0, 20(sp)
  lw t1, 16(sp)
  sw t0, 0(t1)
  lw t0, 4(sp)
  lw t0, 0(t0)
  sw t0, 24(sp)
  lw t0, 16(sp)
  lw t0, 0(t0)
  sw t0, 28(sp)
  lw t0, 24(sp)
  lw t1, 28(sp)
  add t0, t0, t1
  sw t0, 32(sp)
  lw t0, 32(sp)
  mv a0, t0
  lw ra, 36(sp)
  addi sp, sp, 48
  ret

