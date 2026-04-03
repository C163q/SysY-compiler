  .data
  .globl ga
ga:
  .zero 40

  .globl gb
gb:
  .word 5
  .word 6
  .word 7
  .word 8
  .zero 4
  .zero 4

  .text
  .globl main
main:
  addi sp, sp, -304
  sw ra, 288(sp)
  addi t0, sp, 0
  addi t1, sp, 40
  sw t0, 0(t1)
  addi t0, sp, 44
  addi t1, sp, 68
  sw t0, 0(t1)
  addi t0, sp, 68
  lw t0, 0(t0)
  li t1, 12
  mul t1, zero, t1
  add t0, t0, t1
  sw t0, 72(sp)
  addi t0, sp, 72
  lw t0, 0(t0)
  li t1, 4
  mul t1, zero, t1
  add t0, t0, t1
  sw t0, 76(sp)
  li t0, 1
  lw t1, 76(sp)
  sw t0, 0(t1)
  addi t0, sp, 68
  lw t0, 0(t0)
  li t1, 12
  mul t1, zero, t1
  add t0, t0, t1
  sw t0, 80(sp)
  addi t0, sp, 80
  lw t0, 0(t0)
  li t1, 1
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  sw t0, 84(sp)
  li t0, 2
  lw t1, 84(sp)
  sw t0, 0(t1)
  addi t0, sp, 68
  lw t0, 0(t0)
  li t1, 12
  mul t1, zero, t1
  add t0, t0, t1
  sw t0, 88(sp)
  addi t0, sp, 88
  lw t0, 0(t0)
  li t1, 2
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  sw t0, 92(sp)
  li t0, 3
  lw t1, 92(sp)
  sw t0, 0(t1)
  addi t0, sp, 68
  lw t0, 0(t0)
  li t1, 1
  li t2, 12
  mul t2, t1, t2
  add t0, t0, t2
  sw t0, 96(sp)
  addi t0, sp, 96
  lw t0, 0(t0)
  li t1, 4
  mul t1, zero, t1
  add t0, t0, t1
  sw t0, 100(sp)
  li t0, 4
  lw t1, 100(sp)
  sw t0, 0(t1)
  addi t0, sp, 68
  lw t0, 0(t0)
  li t1, 1
  li t2, 12
  mul t2, t1, t2
  add t0, t0, t2
  sw t0, 104(sp)
  addi t0, sp, 104
  lw t0, 0(t0)
  li t1, 1
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  sw t0, 108(sp)
  lw t0, 108(sp)
  sw zero, 0(t0)
  addi t0, sp, 68
  lw t0, 0(t0)
  li t1, 1
  li t2, 12
  mul t2, t1, t2
  add t0, t0, t2
  sw t0, 112(sp)
  addi t0, sp, 112
  lw t0, 0(t0)
  li t1, 2
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  sw t0, 116(sp)
  lw t0, 116(sp)
  sw zero, 0(t0)
  addi t0, sp, 120
  addi t1, sp, 124
  sw t0, 0(t1)
  lw t0, 124(sp)
  sw zero, 0(t0)
  addi t0, sp, 128
  addi t1, sp, 132
  sw t0, 0(t1)
  lw t0, 132(sp)
  sw zero, 0(t0)
  j LBB9_1
LBB9_1:
  lw t0, 132(sp)
  lw t0, 0(t0)
  sw t0, 136(sp)
  lw t0, 136(sp)
  li t1, 2
  slt t0, t0, t1
  sw t0, 140(sp)
  lw t0, 140(sp)
  bnez t0, LBB9_2
  j LBB9_6
LBB9_2:
  addi t0, sp, 144
  addi t1, sp, 148
  sw t0, 0(t1)
  lw t0, 148(sp)
  sw zero, 0(t0)
  j LBB9_3
LBB9_3:
  lw t0, 148(sp)
  lw t0, 0(t0)
  sw t0, 152(sp)
  lw t0, 152(sp)
  li t1, 3
  slt t0, t0, t1
  sw t0, 156(sp)
  lw t0, 156(sp)
  bnez t0, LBB9_4
  j LBB9_5
LBB9_4:
  lw t0, 124(sp)
  lw t0, 0(t0)
  sw t0, 160(sp)
  lw t0, 132(sp)
  lw t0, 0(t0)
  sw t0, 164(sp)
  addi t0, sp, 68
  lw t0, 0(t0)
  lw t1, 164(sp)
  li t2, 12
  mul t2, t1, t2
  add t0, t0, t2
  sw t0, 168(sp)
  lw t0, 148(sp)
  lw t0, 0(t0)
  sw t0, 172(sp)
  addi t0, sp, 168
  lw t0, 0(t0)
  lw t1, 172(sp)
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  sw t0, 176(sp)
  lw t0, 176(sp)
  lw t0, 0(t0)
  sw t0, 180(sp)
  lw t0, 160(sp)
  lw t1, 180(sp)
  add t0, t0, t1
  sw t0, 184(sp)
  lw t0, 132(sp)
  lw t0, 0(t0)
  sw t0, 188(sp)
  la t0, gb
  lw t1, 188(sp)
  li t2, 12
  mul t2, t1, t2
  add t0, t0, t2
  sw t0, 192(sp)
  lw t0, 148(sp)
  lw t0, 0(t0)
  sw t0, 196(sp)
  addi t0, sp, 192
  lw t0, 0(t0)
  lw t1, 196(sp)
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  sw t0, 200(sp)
  lw t0, 200(sp)
  lw t0, 0(t0)
  sw t0, 204(sp)
  lw t0, 184(sp)
  lw t1, 204(sp)
  add t0, t0, t1
  sw t0, 208(sp)
  lw t0, 208(sp)
  lw t1, 124(sp)
  sw t0, 0(t1)
  lw t0, 148(sp)
  lw t0, 0(t0)
  sw t0, 212(sp)
  lw t0, 212(sp)
  li t1, 1
  add t0, t0, t1
  sw t0, 216(sp)
  lw t0, 216(sp)
  lw t1, 148(sp)
  sw t0, 0(t1)
  j LBB9_3
LBB9_5:
  lw t0, 132(sp)
  lw t0, 0(t0)
  sw t0, 220(sp)
  lw t0, 220(sp)
  li t1, 1
  add t0, t0, t1
  sw t0, 224(sp)
  lw t0, 224(sp)
  lw t1, 132(sp)
  sw t0, 0(t1)
  j LBB9_1
LBB9_6:
  lw t0, 132(sp)
  sw zero, 0(t0)
  j LBB9_7
LBB9_7:
  lw t0, 132(sp)
  lw t0, 0(t0)
  sw t0, 228(sp)
  lw t0, 228(sp)
  li t1, 10
  slt t0, t0, t1
  sw t0, 232(sp)
  lw t0, 232(sp)
  bnez t0, LBB9_8
  j LBB9_9
LBB9_8:
  lw t0, 124(sp)
  lw t0, 0(t0)
  sw t0, 236(sp)
  lw t0, 132(sp)
  lw t0, 0(t0)
  sw t0, 240(sp)
  lw t0, 236(sp)
  lw t1, 240(sp)
  add t0, t0, t1
  sw t0, 244(sp)
  lw t0, 132(sp)
  lw t0, 0(t0)
  sw t0, 248(sp)
  addi t0, sp, 40
  lw t0, 0(t0)
  lw t1, 248(sp)
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  sw t0, 252(sp)
  lw t0, 244(sp)
  lw t1, 252(sp)
  sw t0, 0(t1)
  lw t0, 124(sp)
  lw t0, 0(t0)
  sw t0, 256(sp)
  lw t0, 132(sp)
  lw t0, 0(t0)
  sw t0, 260(sp)
  lw t0, 256(sp)
  lw t1, 260(sp)
  add t0, t0, t1
  sw t0, 264(sp)
  lw t0, 132(sp)
  lw t0, 0(t0)
  sw t0, 268(sp)
  la t0, ga
  lw t1, 268(sp)
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  sw t0, 272(sp)
  lw t0, 264(sp)
  lw t1, 272(sp)
  sw t0, 0(t1)
  lw t0, 132(sp)
  lw t0, 0(t0)
  sw t0, 276(sp)
  lw t0, 276(sp)
  li t1, 1
  add t0, t0, t1
  sw t0, 280(sp)
  lw t0, 280(sp)
  lw t1, 132(sp)
  sw t0, 0(t1)
  j LBB9_7
LBB9_9:
  lw t0, 124(sp)
  lw t0, 0(t0)
  sw t0, 284(sp)
  lw t0, 284(sp)
  mv a0, t0
  lw ra, 288(sp)
  addi sp, sp, 304
  ret

