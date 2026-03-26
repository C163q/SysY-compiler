  .data
  .text
  .globl main
main:
  addi sp, sp, -352
  sw ra, 340(sp)
  li t0, 1
  sw t0, 0(sp)
  sw zero, 4(sp)
  lw t0, 0(sp)
  sw t0, 8(sp)
  lw t0, 8(sp)
  li t1, 2
  add t0, t0, t1
  sw t0, 12(sp)
  lw t0, 12(sp)
  sw t0, 0(sp)
  lw t0, 0(sp)
  sw t0, 20(sp)
  lw t0, 20(sp)
  li t1, 3
  add t0, t0, t1
  sw t0, 24(sp)
  lw t0, 24(sp)
  sw t0, 16(sp)
  lw t0, 16(sp)
  sw t0, 28(sp)
  lw t0, 28(sp)
  li t1, 4
  add t0, t0, t1
  sw t0, 32(sp)
  lw t0, 32(sp)
  sw t0, 16(sp)
  lw t0, 4(sp)
  sw t0, 36(sp)
  lw t0, 0(sp)
  sw t0, 40(sp)
  lw t0, 36(sp)
  lw t1, 40(sp)
  add t0, t0, t1
  sw t0, 44(sp)
  lw t0, 16(sp)
  sw t0, 48(sp)
  lw t0, 44(sp)
  lw t1, 48(sp)
  add t0, t0, t1
  sw t0, 52(sp)
  lw t0, 52(sp)
  sw t0, 4(sp)
  lw t0, 16(sp)
  sw t0, 56(sp)
  lw t0, 56(sp)
  li t1, 5
  add t0, t0, t1
  sw t0, 60(sp)
  lw t0, 60(sp)
  sw t0, 16(sp)
  lw t0, 16(sp)
  sw t0, 68(sp)
  lw t0, 68(sp)
  li t1, 6
  add t0, t0, t1
  sw t0, 72(sp)
  lw t0, 72(sp)
  sw t0, 64(sp)
  lw t0, 0(sp)
  sw t0, 76(sp)
  lw t0, 64(sp)
  sw t0, 80(sp)
  lw t0, 76(sp)
  lw t1, 80(sp)
  add t0, t0, t1
  sw t0, 84(sp)
  lw t0, 84(sp)
  sw t0, 0(sp)
  lw t0, 4(sp)
  sw t0, 88(sp)
  lw t0, 0(sp)
  sw t0, 92(sp)
  lw t0, 88(sp)
  lw t1, 92(sp)
  add t0, t0, t1
  sw t0, 96(sp)
  lw t0, 16(sp)
  sw t0, 100(sp)
  lw t0, 96(sp)
  lw t1, 100(sp)
  add t0, t0, t1
  sw t0, 104(sp)
  lw t0, 64(sp)
  sw t0, 108(sp)
  lw t0, 104(sp)
  lw t1, 108(sp)
  add t0, t0, t1
  sw t0, 112(sp)
  lw t0, 112(sp)
  sw t0, 4(sp)
  lw t0, 16(sp)
  sw t0, 116(sp)
  lw t0, 0(sp)
  sw t0, 120(sp)
  lw t0, 116(sp)
  lw t1, 120(sp)
  add t0, t0, t1
  sw t0, 124(sp)
  lw t0, 124(sp)
  sw t0, 16(sp)
  lw t0, 64(sp)
  sw t0, 132(sp)
  lw t0, 132(sp)
  li t1, 7
  add t0, t0, t1
  sw t0, 136(sp)
  lw t0, 136(sp)
  sw t0, 128(sp)
  lw t0, 128(sp)
  sw t0, 140(sp)
  lw t0, 140(sp)
  li t1, 8
  add t0, t0, t1
  sw t0, 144(sp)
  lw t0, 144(sp)
  sw t0, 128(sp)
  lw t0, 4(sp)
  sw t0, 148(sp)
  lw t0, 128(sp)
  sw t0, 152(sp)
  lw t0, 148(sp)
  lw t1, 152(sp)
  add t0, t0, t1
  sw t0, 156(sp)
  lw t0, 16(sp)
  sw t0, 160(sp)
  lw t0, 156(sp)
  lw t1, 160(sp)
  add t0, t0, t1
  sw t0, 164(sp)
  lw t0, 64(sp)
  sw t0, 168(sp)
  lw t0, 164(sp)
  lw t1, 168(sp)
  add t0, t0, t1
  sw t0, 172(sp)
  lw t0, 172(sp)
  sw t0, 4(sp)
  lw t0, 16(sp)
  sw t0, 176(sp)
  lw t0, 128(sp)
  sw t0, 180(sp)
  lw t0, 176(sp)
  lw t1, 180(sp)
  add t0, t0, t1
  sw t0, 184(sp)
  lw t0, 184(sp)
  sw t0, 16(sp)
  lw t0, 64(sp)
  sw t0, 192(sp)
  lw t0, 192(sp)
  li t1, 9
  add t0, t0, t1
  sw t0, 196(sp)
  lw t0, 196(sp)
  sw t0, 188(sp)
  lw t0, 128(sp)
  sw t0, 200(sp)
  lw t0, 200(sp)
  li t1, 10
  add t0, t0, t1
  sw t0, 204(sp)
  lw t0, 204(sp)
  sw t0, 128(sp)
  lw t0, 188(sp)
  sw t0, 208(sp)
  lw t0, 208(sp)
  li t1, 12
  add t0, t0, t1
  sw t0, 212(sp)
  lw t0, 212(sp)
  sw t0, 188(sp)
  lw t0, 4(sp)
  sw t0, 216(sp)
  lw t0, 216(sp)
  li t1, 11
  add t0, t0, t1
  sw t0, 220(sp)
  lw t0, 188(sp)
  sw t0, 224(sp)
  lw t0, 220(sp)
  lw t1, 224(sp)
  add t0, t0, t1
  sw t0, 228(sp)
  lw t0, 64(sp)
  sw t0, 232(sp)
  lw t0, 228(sp)
  lw t1, 232(sp)
  add t0, t0, t1
  sw t0, 236(sp)
  lw t0, 236(sp)
  sw t0, 4(sp)
  lw t0, 64(sp)
  sw t0, 240(sp)
  lw t0, 188(sp)
  sw t0, 244(sp)
  lw t0, 240(sp)
  lw t1, 244(sp)
  add t0, t0, t1
  sw t0, 248(sp)
  lw t0, 248(sp)
  sw t0, 64(sp)
  lw t0, 188(sp)
  sw t0, 256(sp)
  lw t0, 256(sp)
  li t1, 13
  add t0, t0, t1
  sw t0, 260(sp)
  lw t0, 260(sp)
  sw t0, 252(sp)
  lw t0, 252(sp)
  sw t0, 264(sp)
  lw t0, 264(sp)
  li t1, 11
  add t0, t0, t1
  sw t0, 268(sp)
  lw t0, 268(sp)
  sw t0, 252(sp)
  lw t0, 4(sp)
  sw t0, 272(sp)
  lw t0, 272(sp)
  li t1, 11
  add t0, t0, t1
  sw t0, 276(sp)
  lw t0, 188(sp)
  sw t0, 280(sp)
  lw t0, 276(sp)
  lw t1, 280(sp)
  add t0, t0, t1
  sw t0, 284(sp)
  lw t0, 252(sp)
  sw t0, 288(sp)
  lw t0, 284(sp)
  lw t1, 288(sp)
  add t0, t0, t1
  sw t0, 292(sp)
  lw t0, 292(sp)
  sw t0, 4(sp)
  lw t0, 4(sp)
  sw t0, 296(sp)
  lw t0, 64(sp)
  sw t0, 300(sp)
  lw t0, 296(sp)
  lw t1, 300(sp)
  sub t0, t0, t1
  sw t0, 304(sp)
  lw t0, 304(sp)
  sw t0, 4(sp)
  lw t0, 4(sp)
  sw t0, 308(sp)
  lw t0, 16(sp)
  sw t0, 312(sp)
  lw t0, 308(sp)
  lw t1, 312(sp)
  sub t0, t0, t1
  sw t0, 316(sp)
  lw t0, 316(sp)
  sw t0, 4(sp)
  lw t0, 4(sp)
  sw t0, 320(sp)
  lw t0, 0(sp)
  sw t0, 324(sp)
  lw t0, 320(sp)
  lw t1, 324(sp)
  sub t0, t0, t1
  sw t0, 328(sp)
  lw t0, 328(sp)
  sw t0, 4(sp)
  lw t0, 4(sp)
  sw t0, 332(sp)
  lw t0, 332(sp)
  li t1, 77
  rem t0, t0, t1
  sw t0, 336(sp)
  lw t0, 336(sp)
  mv a0, t0
  lw ra, 340(sp)
  addi sp, sp, 352
  ret

