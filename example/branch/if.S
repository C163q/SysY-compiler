  .data
  .text
  .globl main
main:
  addi sp, sp, -320
  sw ra, 308(sp)
  li t0, 1
  sw t0, 4(sp)
  li t0, 1
  seqz t1, t0
  sw t1, 8(sp)
  lw t0, 8(sp)
  bnez t0, LBB9_1
  j LBB9_4
LBB9_1:
  sw zero, 12(sp)
  li t0, 2
  snez t1, t0
  sw t1, 16(sp)
  lw t0, 16(sp)
  bnez t0, LBB9_2
  j LBB9_3
LBB9_2:
  li t0, 3
  snez t1, t0
  sw t1, 20(sp)
  lw t0, 20(sp)
  sw t0, 12(sp)
  j LBB9_3
LBB9_3:
  lw t0, 12(sp)
  sw t0, 24(sp)
  lw t0, 24(sp)
  snez t1, t0
  sw t1, 28(sp)
  lw t0, 28(sp)
  sw t0, 4(sp)
  j LBB9_4
LBB9_4:
  lw t0, 4(sp)
  sw t0, 32(sp)
  lw t0, 32(sp)
  sw t0, 0(sp)
  li t0, 1
  sw t0, 40(sp)
  sw zero, 44(sp)
  snez t0, zero
  sw t0, 48(sp)
  lw t0, 48(sp)
  bnez t0, LBB9_5
  j LBB9_6
LBB9_5:
  li t0, 1
  snez t1, t0
  sw t1, 52(sp)
  lw t0, 52(sp)
  sw t0, 44(sp)
  j LBB9_6
LBB9_6:
  lw t0, 44(sp)
  sw t0, 56(sp)
  lw t0, 56(sp)
  seqz t1, t0
  sw t1, 60(sp)
  lw t0, 60(sp)
  bnez t0, LBB9_7
  j LBB9_8
LBB9_7:
  snez t0, zero
  sw t0, 64(sp)
  lw t0, 64(sp)
  sw t0, 40(sp)
  j LBB9_8
LBB9_8:
  lw t0, 40(sp)
  sw t0, 68(sp)
  lw t0, 68(sp)
  sw t0, 36(sp)
  li t0, 1
  sw t0, 76(sp)
  sw zero, 80(sp)
  li t0, 1
  snez t1, t0
  sw t1, 84(sp)
  lw t0, 84(sp)
  bnez t0, LBB9_9
  j LBB9_10
LBB9_9:
  snez t0, zero
  sw t0, 88(sp)
  lw t0, 88(sp)
  sw t0, 80(sp)
  j LBB9_10
LBB9_10:
  lw t0, 80(sp)
  sw t0, 92(sp)
  lw t0, 92(sp)
  seqz t1, t0
  sw t1, 96(sp)
  lw t0, 96(sp)
  bnez t0, LBB9_11
  j LBB9_12
LBB9_11:
  li t0, 1
  snez t1, t0
  sw t1, 100(sp)
  lw t0, 100(sp)
  sw t0, 76(sp)
  j LBB9_12
LBB9_12:
  lw t0, 76(sp)
  sw t0, 104(sp)
  lw t0, 104(sp)
  li t1, 4
  mul t0, t0, t1
  sw t0, 108(sp)
  lw t0, 108(sp)
  sw t0, 72(sp)
  li t0, 5
  sw t0, 112(sp)
  li t0, 1
  sw t0, 116(sp)
  lw t0, 0(sp)
  sw t0, 120(sp)
  lw t0, 120(sp)
  li t1, 1
  xor t0, t0, t1
  seqz t2, t0
  sw t2, 124(sp)
  lw t0, 124(sp)
  seqz t1, t0
  sw t1, 128(sp)
  lw t0, 128(sp)
  bnez t0, LBB9_13
  j LBB9_14
LBB9_13:
  lw t0, 0(sp)
  sw t0, 132(sp)
  lw t0, 132(sp)
  li t1, 2
  xor t0, t0, t1
  seqz t2, t0
  sw t2, 136(sp)
  lw t0, 136(sp)
  snez t1, t0
  sw t1, 140(sp)
  lw t0, 140(sp)
  sw t0, 116(sp)
  j LBB9_14
LBB9_14:
  lw t0, 116(sp)
  sw t0, 144(sp)
  lw t0, 144(sp)
  bnez t0, LBB9_15
  j LBB9_16
LBB9_15:
  j LBB9_16
LBB9_16:
  li t0, 1
  sw t0, 148(sp)
  lw t0, 36(sp)
  sw t0, 152(sp)
  lw t0, 152(sp)
  seqz t1, t0
  sw t1, 156(sp)
  lw t0, 156(sp)
  seqz t1, t0
  sw t1, 160(sp)
  lw t0, 160(sp)
  bnez t0, LBB9_17
  j LBB9_18
LBB9_17:
  lw t0, 36(sp)
  sw t0, 164(sp)
  lw t0, 164(sp)
  li t1, 1
  xor t0, t0, t1
  seqz t2, t0
  sw t2, 168(sp)
  lw t0, 168(sp)
  snez t1, t0
  sw t1, 172(sp)
  lw t0, 172(sp)
  sw t0, 148(sp)
  j LBB9_18
LBB9_18:
  lw t0, 148(sp)
  sw t0, 176(sp)
  lw t0, 176(sp)
  bnez t0, LBB9_19
  j LBB9_20
LBB9_19:
  lw t0, 112(sp)
  sw t0, 180(sp)
  lw t0, 180(sp)
  li t1, 1
  add t0, t0, t1
  sw t0, 184(sp)
  lw t0, 184(sp)
  sw t0, 112(sp)
  j LBB9_21
LBB9_20:
  j LBB9_21
LBB9_21:
  li t0, 1
  sw t0, 188(sp)
  sw zero, 192(sp)
  lw t0, 0(sp)
  sw t0, 196(sp)
  lw t0, 196(sp)
  snez t1, t0
  sw t1, 200(sp)
  lw t0, 200(sp)
  bnez t0, LBB9_22
  j LBB9_23
LBB9_22:
  lw t0, 36(sp)
  sw t0, 204(sp)
  lw t0, 204(sp)
  snez t1, t0
  sw t1, 208(sp)
  lw t0, 208(sp)
  sw t0, 192(sp)
  j LBB9_23
LBB9_23:
  lw t0, 192(sp)
  sw t0, 212(sp)
  lw t0, 212(sp)
  seqz t1, t0
  sw t1, 216(sp)
  lw t0, 216(sp)
  bnez t0, LBB9_24
  j LBB9_27
LBB9_24:
  sw zero, 220(sp)
  lw t0, 72(sp)
  sw t0, 224(sp)
  lw t0, 224(sp)
  snez t1, t0
  sw t1, 228(sp)
  lw t0, 228(sp)
  bnez t0, LBB9_25
  j LBB9_26
LBB9_25:
  lw t0, 112(sp)
  sw t0, 232(sp)
  lw t0, 232(sp)
  snez t1, t0
  sw t1, 236(sp)
  lw t0, 236(sp)
  sw t0, 220(sp)
  j LBB9_26
LBB9_26:
  lw t0, 220(sp)
  sw t0, 240(sp)
  lw t0, 240(sp)
  snez t1, t0
  sw t1, 244(sp)
  lw t0, 244(sp)
  sw t0, 188(sp)
  j LBB9_27
LBB9_27:
  lw t0, 188(sp)
  sw t0, 248(sp)
  lw t0, 248(sp)
  bnez t0, LBB9_28
  j LBB9_29
LBB9_28:
  lw t0, 112(sp)
  sw t0, 252(sp)
  lw t0, 252(sp)
  li t1, 1
  add t0, t0, t1
  sw t0, 256(sp)
  lw t0, 256(sp)
  sw t0, 112(sp)
  j LBB9_29
LBB9_29:
  li t0, 1
  sw t0, 260(sp)
  lw t0, 112(sp)
  sw t0, 264(sp)
  lw t0, 264(sp)
  seqz t1, t0
  sw t1, 268(sp)
  lw t0, 268(sp)
  bnez t0, LBB9_30
  j LBB9_31
LBB9_30:
  lw t0, 72(sp)
  sw t0, 272(sp)
  lw t0, 272(sp)
  seqz t1, t0
  sw t1, 276(sp)
  lw t0, 276(sp)
  snez t1, t0
  sw t1, 280(sp)
  lw t0, 280(sp)
  sw t0, 260(sp)
  j LBB9_31
LBB9_31:
  lw t0, 260(sp)
  sw t0, 284(sp)
  lw t0, 284(sp)
  mv a0, t0
  lw ra, 308(sp)
  addi sp, sp, 320
  ret
LBB9_32:
  li t0, 1
  sw t0, 288(sp)
  lw t0, 112(sp)
  sw t0, 292(sp)
  lw t0, 292(sp)
  seqz t1, t0
  sw t1, 296(sp)
  lw t0, 296(sp)
  bnez t0, LBB9_33
  j LBB9_34
LBB9_33:
  li t0, 1
  snez t1, t0
  sw t1, 300(sp)
  lw t0, 300(sp)
  sw t0, 288(sp)
  j LBB9_34
LBB9_34:
  lw t0, 288(sp)
  sw t0, 304(sp)
  lw t0, 304(sp)
  mv a0, t0
  lw ra, 308(sp)
  addi sp, sp, 320
  ret

