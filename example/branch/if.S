  .data
  .text
  .globl main
main:
  addi sp, sp, -384
  sw ra, 376(sp)
  addi t0, sp, 0
  addi t1, sp, 4
  sw t0, 0(t1)
  addi t0, sp, 8
  addi t1, sp, 12
  sw t0, 0(t1)
  li t0, 1
  lw t1, 12(sp)
  sw t0, 0(t1)
  li t0, 1
  seqz t1, t0
  sw t1, 16(sp)
  lw t0, 16(sp)
  bnez t0, LBB9_1
  j LBB9_4
LBB9_1:
  addi t0, sp, 20
  addi t1, sp, 24
  sw t0, 0(t1)
  lw t0, 24(sp)
  sw zero, 0(t0)
  li t0, 2
  snez t1, t0
  sw t1, 28(sp)
  lw t0, 28(sp)
  bnez t0, LBB9_2
  j LBB9_3
LBB9_2:
  li t0, 3
  snez t1, t0
  sw t1, 32(sp)
  lw t0, 32(sp)
  lw t1, 24(sp)
  sw t0, 0(t1)
  j LBB9_3
LBB9_3:
  lw t0, 24(sp)
  lw t0, 0(t0)
  sw t0, 36(sp)
  lw t0, 36(sp)
  snez t1, t0
  sw t1, 40(sp)
  lw t0, 40(sp)
  lw t1, 12(sp)
  sw t0, 0(t1)
  j LBB9_4
LBB9_4:
  lw t0, 12(sp)
  lw t0, 0(t0)
  sw t0, 44(sp)
  lw t0, 44(sp)
  lw t1, 4(sp)
  sw t0, 0(t1)
  addi t0, sp, 48
  addi t1, sp, 52
  sw t0, 0(t1)
  addi t0, sp, 56
  addi t1, sp, 60
  sw t0, 0(t1)
  li t0, 1
  lw t1, 60(sp)
  sw t0, 0(t1)
  addi t0, sp, 64
  addi t1, sp, 68
  sw t0, 0(t1)
  lw t0, 68(sp)
  sw zero, 0(t0)
  snez t0, zero
  sw t0, 72(sp)
  lw t0, 72(sp)
  bnez t0, LBB9_5
  j LBB9_6
LBB9_5:
  li t0, 1
  snez t1, t0
  sw t1, 76(sp)
  lw t0, 76(sp)
  lw t1, 68(sp)
  sw t0, 0(t1)
  j LBB9_6
LBB9_6:
  lw t0, 68(sp)
  lw t0, 0(t0)
  sw t0, 80(sp)
  lw t0, 80(sp)
  seqz t1, t0
  sw t1, 84(sp)
  lw t0, 84(sp)
  bnez t0, LBB9_7
  j LBB9_8
LBB9_7:
  snez t0, zero
  sw t0, 88(sp)
  lw t0, 88(sp)
  lw t1, 60(sp)
  sw t0, 0(t1)
  j LBB9_8
LBB9_8:
  lw t0, 60(sp)
  lw t0, 0(t0)
  sw t0, 92(sp)
  lw t0, 92(sp)
  lw t1, 52(sp)
  sw t0, 0(t1)
  addi t0, sp, 96
  addi t1, sp, 100
  sw t0, 0(t1)
  addi t0, sp, 104
  addi t1, sp, 108
  sw t0, 0(t1)
  li t0, 1
  lw t1, 108(sp)
  sw t0, 0(t1)
  addi t0, sp, 112
  addi t1, sp, 116
  sw t0, 0(t1)
  lw t0, 116(sp)
  sw zero, 0(t0)
  li t0, 1
  snez t1, t0
  sw t1, 120(sp)
  lw t0, 120(sp)
  bnez t0, LBB9_9
  j LBB9_10
LBB9_9:
  snez t0, zero
  sw t0, 124(sp)
  lw t0, 124(sp)
  lw t1, 116(sp)
  sw t0, 0(t1)
  j LBB9_10
LBB9_10:
  lw t0, 116(sp)
  lw t0, 0(t0)
  sw t0, 128(sp)
  lw t0, 128(sp)
  seqz t1, t0
  sw t1, 132(sp)
  lw t0, 132(sp)
  bnez t0, LBB9_11
  j LBB9_12
LBB9_11:
  li t0, 1
  snez t1, t0
  sw t1, 136(sp)
  lw t0, 136(sp)
  lw t1, 108(sp)
  sw t0, 0(t1)
  j LBB9_12
LBB9_12:
  lw t0, 108(sp)
  lw t0, 0(t0)
  sw t0, 140(sp)
  lw t0, 140(sp)
  li t1, 4
  mul t0, t0, t1
  sw t0, 144(sp)
  lw t0, 144(sp)
  lw t1, 100(sp)
  sw t0, 0(t1)
  addi t0, sp, 148
  addi t1, sp, 152
  sw t0, 0(t1)
  li t0, 5
  lw t1, 152(sp)
  sw t0, 0(t1)
  addi t0, sp, 156
  addi t1, sp, 160
  sw t0, 0(t1)
  li t0, 1
  lw t1, 160(sp)
  sw t0, 0(t1)
  lw t0, 4(sp)
  lw t0, 0(t0)
  sw t0, 164(sp)
  lw t0, 164(sp)
  li t1, 1
  xor t0, t0, t1
  seqz t2, t0
  sw t2, 168(sp)
  lw t0, 168(sp)
  seqz t1, t0
  sw t1, 172(sp)
  lw t0, 172(sp)
  bnez t0, LBB9_13
  j LBB9_14
LBB9_13:
  lw t0, 4(sp)
  lw t0, 0(t0)
  sw t0, 176(sp)
  lw t0, 176(sp)
  li t1, 2
  xor t0, t0, t1
  seqz t2, t0
  sw t2, 180(sp)
  lw t0, 180(sp)
  snez t1, t0
  sw t1, 184(sp)
  lw t0, 184(sp)
  lw t1, 160(sp)
  sw t0, 0(t1)
  j LBB9_14
LBB9_14:
  lw t0, 160(sp)
  lw t0, 0(t0)
  sw t0, 188(sp)
  lw t0, 188(sp)
  bnez t0, LBB9_15
  j LBB9_16
LBB9_15:
  j LBB9_16
LBB9_16:
  addi t0, sp, 192
  addi t1, sp, 196
  sw t0, 0(t1)
  li t0, 1
  lw t1, 196(sp)
  sw t0, 0(t1)
  lw t0, 52(sp)
  lw t0, 0(t0)
  sw t0, 200(sp)
  lw t0, 200(sp)
  seqz t1, t0
  sw t1, 204(sp)
  lw t0, 204(sp)
  seqz t1, t0
  sw t1, 208(sp)
  lw t0, 208(sp)
  bnez t0, LBB9_17
  j LBB9_18
LBB9_17:
  lw t0, 52(sp)
  lw t0, 0(t0)
  sw t0, 212(sp)
  lw t0, 212(sp)
  li t1, 1
  xor t0, t0, t1
  seqz t2, t0
  sw t2, 216(sp)
  lw t0, 216(sp)
  snez t1, t0
  sw t1, 220(sp)
  lw t0, 220(sp)
  lw t1, 196(sp)
  sw t0, 0(t1)
  j LBB9_18
LBB9_18:
  lw t0, 196(sp)
  lw t0, 0(t0)
  sw t0, 224(sp)
  lw t0, 224(sp)
  bnez t0, LBB9_19
  j LBB9_20
LBB9_19:
  lw t0, 152(sp)
  lw t0, 0(t0)
  sw t0, 228(sp)
  lw t0, 228(sp)
  li t1, 1
  add t0, t0, t1
  sw t0, 232(sp)
  lw t0, 232(sp)
  lw t1, 152(sp)
  sw t0, 0(t1)
  j LBB9_21
LBB9_20:
  j LBB9_21
LBB9_21:
  addi t0, sp, 236
  addi t1, sp, 240
  sw t0, 0(t1)
  li t0, 1
  lw t1, 240(sp)
  sw t0, 0(t1)
  addi t0, sp, 244
  addi t1, sp, 248
  sw t0, 0(t1)
  lw t0, 248(sp)
  sw zero, 0(t0)
  lw t0, 4(sp)
  lw t0, 0(t0)
  sw t0, 252(sp)
  lw t0, 252(sp)
  snez t1, t0
  sw t1, 256(sp)
  lw t0, 256(sp)
  bnez t0, LBB9_22
  j LBB9_23
LBB9_22:
  lw t0, 52(sp)
  lw t0, 0(t0)
  sw t0, 260(sp)
  lw t0, 260(sp)
  snez t1, t0
  sw t1, 264(sp)
  lw t0, 264(sp)
  lw t1, 248(sp)
  sw t0, 0(t1)
  j LBB9_23
LBB9_23:
  lw t0, 248(sp)
  lw t0, 0(t0)
  sw t0, 268(sp)
  lw t0, 268(sp)
  seqz t1, t0
  sw t1, 272(sp)
  lw t0, 272(sp)
  bnez t0, LBB9_24
  j LBB9_27
LBB9_24:
  addi t0, sp, 276
  addi t1, sp, 280
  sw t0, 0(t1)
  lw t0, 280(sp)
  sw zero, 0(t0)
  lw t0, 100(sp)
  lw t0, 0(t0)
  sw t0, 284(sp)
  lw t0, 284(sp)
  snez t1, t0
  sw t1, 288(sp)
  lw t0, 288(sp)
  bnez t0, LBB9_25
  j LBB9_26
LBB9_25:
  lw t0, 152(sp)
  lw t0, 0(t0)
  sw t0, 292(sp)
  lw t0, 292(sp)
  snez t1, t0
  sw t1, 296(sp)
  lw t0, 296(sp)
  lw t1, 280(sp)
  sw t0, 0(t1)
  j LBB9_26
LBB9_26:
  lw t0, 280(sp)
  lw t0, 0(t0)
  sw t0, 300(sp)
  lw t0, 300(sp)
  snez t1, t0
  sw t1, 304(sp)
  lw t0, 304(sp)
  lw t1, 240(sp)
  sw t0, 0(t1)
  j LBB9_27
LBB9_27:
  lw t0, 240(sp)
  lw t0, 0(t0)
  sw t0, 308(sp)
  lw t0, 308(sp)
  bnez t0, LBB9_28
  j LBB9_29
LBB9_28:
  lw t0, 152(sp)
  lw t0, 0(t0)
  sw t0, 312(sp)
  lw t0, 312(sp)
  li t1, 1
  add t0, t0, t1
  sw t0, 316(sp)
  lw t0, 316(sp)
  lw t1, 152(sp)
  sw t0, 0(t1)
  j LBB9_29
LBB9_29:
  addi t0, sp, 320
  addi t1, sp, 324
  sw t0, 0(t1)
  li t0, 1
  lw t1, 324(sp)
  sw t0, 0(t1)
  lw t0, 152(sp)
  lw t0, 0(t0)
  sw t0, 328(sp)
  lw t0, 328(sp)
  seqz t1, t0
  sw t1, 332(sp)
  lw t0, 332(sp)
  bnez t0, LBB9_30
  j LBB9_31
LBB9_30:
  lw t0, 100(sp)
  lw t0, 0(t0)
  sw t0, 336(sp)
  lw t0, 336(sp)
  seqz t1, t0
  sw t1, 340(sp)
  lw t0, 340(sp)
  snez t1, t0
  sw t1, 344(sp)
  lw t0, 344(sp)
  lw t1, 324(sp)
  sw t0, 0(t1)
  j LBB9_31
LBB9_31:
  lw t0, 324(sp)
  lw t0, 0(t0)
  sw t0, 348(sp)
  lw t0, 348(sp)
  mv a0, t0
  lw ra, 376(sp)
  addi sp, sp, 384
  ret
LBB9_32:
  addi t0, sp, 352
  addi t1, sp, 356
  sw t0, 0(t1)
  li t0, 1
  lw t1, 356(sp)
  sw t0, 0(t1)
  lw t0, 152(sp)
  lw t0, 0(t0)
  sw t0, 360(sp)
  lw t0, 360(sp)
  seqz t1, t0
  sw t1, 364(sp)
  lw t0, 364(sp)
  bnez t0, LBB9_33
  j LBB9_34
LBB9_33:
  li t0, 1
  snez t1, t0
  sw t1, 368(sp)
  lw t0, 368(sp)
  lw t1, 356(sp)
  sw t0, 0(t1)
  j LBB9_34
LBB9_34:
  lw t0, 356(sp)
  lw t0, 0(t0)
  sw t0, 372(sp)
  lw t0, 372(sp)
  mv a0, t0
  lw ra, 376(sp)
  addi sp, sp, 384
  ret

