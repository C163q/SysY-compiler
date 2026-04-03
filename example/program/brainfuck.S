  .data
  .globl program_length
program_length:
  .word 0

  .globl program
program:
  .zero 262144

  .globl tape
tape:
  .zero 262144

  .globl input
input:
  .zero 262144

  .globl input_length
input_length:
  .word 0

  .globl output
output:
  .zero 262144

  .globl output_length
output_length:
  .word 0

  .text
  .globl get_bf_char
get_bf_char:
  addi sp, sp, -256
  sw ra, 252(sp)
  addi t0, sp, 0
  addi t1, sp, 4
  sw t0, 0(t1)
  call getch
  sw a0, 8(sp)
  lw t0, 8(sp)
  lw t1, 4(sp)
  sw t0, 0(t1)
  j LBB9_1
LBB9_1:
  addi t0, sp, 12
  addi t1, sp, 16
  sw t0, 0(t1)
  lw t0, 16(sp)
  sw zero, 0(t0)
  addi t0, sp, 20
  addi t1, sp, 24
  sw t0, 0(t1)
  lw t0, 24(sp)
  sw zero, 0(t0)
  addi t0, sp, 28
  addi t1, sp, 32
  sw t0, 0(t1)
  lw t0, 32(sp)
  sw zero, 0(t0)
  addi t0, sp, 36
  addi t1, sp, 40
  sw t0, 0(t1)
  lw t0, 40(sp)
  sw zero, 0(t0)
  addi t0, sp, 44
  addi t1, sp, 48
  sw t0, 0(t1)
  lw t0, 48(sp)
  sw zero, 0(t0)
  addi t0, sp, 52
  addi t1, sp, 56
  sw t0, 0(t1)
  lw t0, 56(sp)
  sw zero, 0(t0)
  addi t0, sp, 60
  addi t1, sp, 64
  sw t0, 0(t1)
  lw t0, 64(sp)
  sw zero, 0(t0)
  addi t0, sp, 68
  addi t1, sp, 72
  sw t0, 0(t1)
  lw t0, 72(sp)
  sw zero, 0(t0)
  lw t0, 4(sp)
  lw t0, 0(t0)
  sw t0, 76(sp)
  lw t0, 76(sp)
  li t1, 62
  xor t0, t0, t1
  snez t2, t0
  sw t2, 80(sp)
  lw t0, 80(sp)
  snez t1, t0
  sw t1, 84(sp)
  lw t0, 84(sp)
  bnez t0, LBB9_2
  j LBB9_3
LBB9_2:
  lw t0, 4(sp)
  lw t0, 0(t0)
  sw t0, 88(sp)
  lw t0, 88(sp)
  li t1, 60
  xor t0, t0, t1
  snez t2, t0
  sw t2, 92(sp)
  lw t0, 92(sp)
  snez t1, t0
  sw t1, 96(sp)
  lw t0, 96(sp)
  lw t1, 72(sp)
  sw t0, 0(t1)
  j LBB9_3
LBB9_3:
  lw t0, 72(sp)
  lw t0, 0(t0)
  sw t0, 100(sp)
  lw t0, 100(sp)
  snez t1, t0
  sw t1, 104(sp)
  lw t0, 104(sp)
  bnez t0, LBB9_4
  j LBB9_5
LBB9_4:
  lw t0, 4(sp)
  lw t0, 0(t0)
  sw t0, 108(sp)
  lw t0, 108(sp)
  li t1, 43
  xor t0, t0, t1
  snez t2, t0
  sw t2, 112(sp)
  lw t0, 112(sp)
  snez t1, t0
  sw t1, 116(sp)
  lw t0, 116(sp)
  lw t1, 64(sp)
  sw t0, 0(t1)
  j LBB9_5
LBB9_5:
  lw t0, 64(sp)
  lw t0, 0(t0)
  sw t0, 120(sp)
  lw t0, 120(sp)
  snez t1, t0
  sw t1, 124(sp)
  lw t0, 124(sp)
  bnez t0, LBB9_6
  j LBB9_7
LBB9_6:
  lw t0, 4(sp)
  lw t0, 0(t0)
  sw t0, 128(sp)
  lw t0, 128(sp)
  li t1, 45
  xor t0, t0, t1
  snez t2, t0
  sw t2, 132(sp)
  lw t0, 132(sp)
  snez t1, t0
  sw t1, 136(sp)
  lw t0, 136(sp)
  lw t1, 56(sp)
  sw t0, 0(t1)
  j LBB9_7
LBB9_7:
  lw t0, 56(sp)
  lw t0, 0(t0)
  sw t0, 140(sp)
  lw t0, 140(sp)
  snez t1, t0
  sw t1, 144(sp)
  lw t0, 144(sp)
  bnez t0, LBB9_8
  j LBB9_9
LBB9_8:
  lw t0, 4(sp)
  lw t0, 0(t0)
  sw t0, 148(sp)
  lw t0, 148(sp)
  li t1, 91
  xor t0, t0, t1
  snez t2, t0
  sw t2, 152(sp)
  lw t0, 152(sp)
  snez t1, t0
  sw t1, 156(sp)
  lw t0, 156(sp)
  lw t1, 48(sp)
  sw t0, 0(t1)
  j LBB9_9
LBB9_9:
  lw t0, 48(sp)
  lw t0, 0(t0)
  sw t0, 160(sp)
  lw t0, 160(sp)
  snez t1, t0
  sw t1, 164(sp)
  lw t0, 164(sp)
  bnez t0, LBB9_10
  j LBB9_11
LBB9_10:
  lw t0, 4(sp)
  lw t0, 0(t0)
  sw t0, 168(sp)
  lw t0, 168(sp)
  li t1, 93
  xor t0, t0, t1
  snez t2, t0
  sw t2, 172(sp)
  lw t0, 172(sp)
  snez t1, t0
  sw t1, 176(sp)
  lw t0, 176(sp)
  lw t1, 40(sp)
  sw t0, 0(t1)
  j LBB9_11
LBB9_11:
  lw t0, 40(sp)
  lw t0, 0(t0)
  sw t0, 180(sp)
  lw t0, 180(sp)
  snez t1, t0
  sw t1, 184(sp)
  lw t0, 184(sp)
  bnez t0, LBB9_12
  j LBB9_13
LBB9_12:
  lw t0, 4(sp)
  lw t0, 0(t0)
  sw t0, 188(sp)
  lw t0, 188(sp)
  li t1, 46
  xor t0, t0, t1
  snez t2, t0
  sw t2, 192(sp)
  lw t0, 192(sp)
  snez t1, t0
  sw t1, 196(sp)
  lw t0, 196(sp)
  lw t1, 32(sp)
  sw t0, 0(t1)
  j LBB9_13
LBB9_13:
  lw t0, 32(sp)
  lw t0, 0(t0)
  sw t0, 200(sp)
  lw t0, 200(sp)
  snez t1, t0
  sw t1, 204(sp)
  lw t0, 204(sp)
  bnez t0, LBB9_14
  j LBB9_15
LBB9_14:
  lw t0, 4(sp)
  lw t0, 0(t0)
  sw t0, 208(sp)
  lw t0, 208(sp)
  li t1, 44
  xor t0, t0, t1
  snez t2, t0
  sw t2, 212(sp)
  lw t0, 212(sp)
  snez t1, t0
  sw t1, 216(sp)
  lw t0, 216(sp)
  lw t1, 24(sp)
  sw t0, 0(t1)
  j LBB9_15
LBB9_15:
  lw t0, 24(sp)
  lw t0, 0(t0)
  sw t0, 220(sp)
  lw t0, 220(sp)
  snez t1, t0
  sw t1, 224(sp)
  lw t0, 224(sp)
  bnez t0, LBB9_16
  j LBB9_17
LBB9_16:
  lw t0, 4(sp)
  lw t0, 0(t0)
  sw t0, 228(sp)
  lw t0, 228(sp)
  li t1, 35
  xor t0, t0, t1
  snez t2, t0
  sw t2, 232(sp)
  lw t0, 232(sp)
  snez t1, t0
  sw t1, 236(sp)
  lw t0, 236(sp)
  lw t1, 16(sp)
  sw t0, 0(t1)
  j LBB9_17
LBB9_17:
  lw t0, 16(sp)
  lw t0, 0(t0)
  sw t0, 240(sp)
  lw t0, 240(sp)
  bnez t0, LBB9_18
  j LBB9_19
LBB9_18:
  call getch
  sw a0, 244(sp)
  lw t0, 244(sp)
  lw t1, 4(sp)
  sw t0, 0(t1)
  j LBB9_1
LBB9_19:
  lw t0, 4(sp)
  lw t0, 0(t0)
  sw t0, 248(sp)
  lw t0, 248(sp)
  mv a0, t0
  lw ra, 252(sp)
  addi sp, sp, 256
  ret

  .globl read_program
read_program:
  addi sp, sp, -128
  sw ra, 112(sp)
  addi t0, sp, 0
  addi t1, sp, 4
  sw t0, 0(t1)
  call get_bf_char
  sw a0, 8(sp)
  lw t0, 8(sp)
  lw t1, 4(sp)
  sw t0, 0(t1)
  j LBB10_1
LBB10_1:
  lw t0, 4(sp)
  lw t0, 0(t0)
  sw t0, 12(sp)
  lw t0, 12(sp)
  li t1, 35
  xor t0, t0, t1
  snez t2, t0
  sw t2, 16(sp)
  lw t0, 16(sp)
  bnez t0, LBB10_2
  j LBB10_3
LBB10_2:
  lw t0, 4(sp)
  lw t0, 0(t0)
  sw t0, 20(sp)
  la t0, program_length
  lw t0, 0(t0)
  sw t0, 24(sp)
  la t0, program
  lw t1, 24(sp)
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  sw t0, 28(sp)
  lw t0, 20(sp)
  lw t1, 28(sp)
  sw t0, 0(t1)
  call get_bf_char
  sw a0, 32(sp)
  lw t0, 32(sp)
  lw t1, 4(sp)
  sw t0, 0(t1)
  la t0, program_length
  lw t0, 0(t0)
  sw t0, 36(sp)
  lw t0, 36(sp)
  li t1, 1
  add t0, t0, t1
  sw t0, 40(sp)
  lw t0, 40(sp)
  la t1, program_length
  sw t0, 0(t1)
  j LBB10_1
LBB10_3:
  addi t0, sp, 44
  addi t1, sp, 48
  sw t0, 0(t1)
  call getch
  sw a0, 52(sp)
  lw t0, 52(sp)
  lw t1, 48(sp)
  sw t0, 0(t1)
  lw t0, 48(sp)
  lw t0, 0(t0)
  sw t0, 56(sp)
  lw t0, 56(sp)
  li t1, 105
  xor t0, t0, t1
  snez t2, t0
  sw t2, 60(sp)
  lw t0, 60(sp)
  bnez t0, LBB10_4
  j LBB10_6
LBB10_4:
  lw ra, 112(sp)
  addi sp, sp, 128
  ret
LBB10_5:
  j LBB10_6
LBB10_6:
  call getint
  sw a0, 64(sp)
  lw t0, 64(sp)
  la t1, input_length
  sw t0, 0(t1)
  call getch
  sw a0, 68(sp)
  addi t0, sp, 72
  addi t1, sp, 76
  sw t0, 0(t1)
  lw t0, 76(sp)
  sw zero, 0(t0)
  j LBB10_7
LBB10_7:
  lw t0, 76(sp)
  lw t0, 0(t0)
  sw t0, 80(sp)
  la t0, input_length
  lw t0, 0(t0)
  sw t0, 84(sp)
  lw t0, 80(sp)
  lw t1, 84(sp)
  slt t0, t0, t1
  sw t0, 88(sp)
  lw t0, 88(sp)
  bnez t0, LBB10_8
  j LBB10_9
LBB10_8:
  call getch
  sw a0, 92(sp)
  lw t0, 76(sp)
  lw t0, 0(t0)
  sw t0, 96(sp)
  la t0, input
  lw t1, 96(sp)
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  sw t0, 100(sp)
  lw t0, 92(sp)
  lw t1, 100(sp)
  sw t0, 0(t1)
  lw t0, 76(sp)
  lw t0, 0(t0)
  sw t0, 104(sp)
  lw t0, 104(sp)
  li t1, 1
  add t0, t0, t1
  sw t0, 108(sp)
  lw t0, 108(sp)
  lw t1, 76(sp)
  sw t0, 0(t1)
  j LBB10_7
LBB10_9:
  lw ra, 112(sp)
  addi sp, sp, 128
  ret

  .globl run_program
run_program:
  li t1, -4560
  add sp, sp, t1
  li t1, 4548
  add t1, sp, t1
  sw ra, 0(t1)
  addi t0, sp, 0
  addi t1, sp, 4
  sw t0, 0(t1)
  lw t0, 4(sp)
  sw zero, 0(t0)
  addi t0, sp, 8
  addi t1, sp, 12
  sw t0, 0(t1)
  lw t0, 12(sp)
  sw zero, 0(t0)
  addi t0, sp, 16
  addi t1, sp, 20
  sw t0, 0(t1)
  lw t0, 20(sp)
  sw zero, 0(t0)
  addi t0, sp, 24
  li t1, 2072
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 4
  mul t1, zero, t1
  add t0, t0, t1
  li t1, 2076
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2076
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 1
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2080
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2080
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 2
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2084
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2084
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 3
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2088
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2088
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 4
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2092
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2092
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 5
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2096
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2096
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 6
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2100
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2100
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 7
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2104
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2104
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 8
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2108
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2108
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 9
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2112
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2112
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 10
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2116
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2116
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 11
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2120
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2120
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 12
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2124
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2124
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 13
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2128
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2128
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 14
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2132
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2132
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 15
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2136
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2136
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 16
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2140
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2140
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 17
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2144
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2144
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 18
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2148
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2148
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 19
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2152
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2152
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 20
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2156
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2156
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 21
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2160
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2160
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 22
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2164
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2164
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 23
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2168
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2168
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 24
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2172
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2172
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 25
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2176
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2176
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 26
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2180
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2180
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 27
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2184
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2184
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 28
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2188
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2188
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 29
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2192
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2192
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 30
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2196
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2196
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 31
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2200
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2200
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 32
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2204
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2204
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 33
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2208
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2208
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 34
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2212
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2212
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 35
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2216
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2216
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 36
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2220
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2220
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 37
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2224
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2224
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 38
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2228
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2228
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 39
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2232
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2232
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 40
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2236
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2236
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 41
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2240
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2240
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 42
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2244
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2244
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 43
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2248
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2248
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 44
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2252
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2252
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 45
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2256
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2256
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 46
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2260
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2260
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 47
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2264
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2264
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 48
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2268
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2268
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 49
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2272
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2272
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 50
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2276
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2276
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 51
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2280
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2280
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 52
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2284
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2284
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 53
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2288
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2288
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 54
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2292
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2292
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 55
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2296
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2296
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 56
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2300
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2300
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 57
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2304
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2304
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 58
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2308
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2308
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 59
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2312
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2312
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 60
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2316
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2316
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 61
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2320
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2320
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 62
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2324
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2324
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 63
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2328
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2328
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 64
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2332
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2332
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 65
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2336
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2336
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 66
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2340
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2340
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 67
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2344
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2344
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 68
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2348
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2348
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 69
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2352
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2352
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 70
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2356
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2356
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 71
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2360
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2360
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 72
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2364
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2364
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 73
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2368
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2368
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 74
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2372
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2372
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 75
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2376
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2376
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 76
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2380
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2380
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 77
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2384
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2384
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 78
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2388
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2388
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 79
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2392
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2392
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 80
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2396
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2396
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 81
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2400
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2400
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 82
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2404
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2404
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 83
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2408
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2408
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 84
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2412
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2412
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 85
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2416
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2416
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 86
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2420
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2420
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 87
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2424
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2424
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 88
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2428
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2428
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 89
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2432
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2432
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 90
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2436
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2436
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 91
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2440
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2440
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 92
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2444
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2444
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 93
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2448
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2448
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 94
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2452
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2452
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 95
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2456
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2456
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 96
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2460
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2460
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 97
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2464
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2464
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 98
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2468
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2468
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 99
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2472
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2472
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 100
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2476
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2476
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 101
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2480
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2480
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 102
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2484
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2484
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 103
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2488
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2488
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 104
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2492
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2492
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 105
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2496
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2496
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 106
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2500
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2500
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 107
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2504
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2504
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 108
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2508
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2508
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 109
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2512
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2512
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 110
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2516
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2516
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 111
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2520
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2520
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 112
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2524
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2524
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 113
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2528
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2528
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 114
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2532
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2532
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 115
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2536
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2536
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 116
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2540
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2540
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 117
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2544
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2544
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 118
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2548
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2548
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 119
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2552
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2552
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 120
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2556
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2556
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 121
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2560
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2560
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 122
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2564
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2564
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 123
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2568
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2568
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 124
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2572
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2572
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 125
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2576
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2576
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 126
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2580
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2580
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 127
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2584
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2584
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 128
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2588
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2588
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 129
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2592
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2592
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 130
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2596
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2596
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 131
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2600
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2600
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 132
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2604
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2604
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 133
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2608
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2608
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 134
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2612
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2612
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 135
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2616
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2616
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 136
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2620
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2620
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 137
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2624
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2624
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 138
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2628
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2628
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 139
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2632
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2632
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 140
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2636
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2636
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 141
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2640
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2640
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 142
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2644
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2644
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 143
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2648
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2648
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 144
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2652
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2652
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 145
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2656
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2656
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 146
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2660
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2660
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 147
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2664
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2664
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 148
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2668
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2668
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 149
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2672
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2672
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 150
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2676
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2676
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 151
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2680
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2680
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 152
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2684
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2684
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 153
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2688
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2688
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 154
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2692
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2692
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 155
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2696
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2696
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 156
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2700
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2700
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 157
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2704
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2704
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 158
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2708
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2708
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 159
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2712
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2712
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 160
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2716
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2716
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 161
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2720
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2720
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 162
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2724
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2724
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 163
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2728
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2728
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 164
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2732
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2732
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 165
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2736
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2736
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 166
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2740
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2740
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 167
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2744
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2744
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 168
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2748
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2748
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 169
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2752
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2752
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 170
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2756
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2756
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 171
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2760
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2760
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 172
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2764
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2764
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 173
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2768
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2768
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 174
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2772
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2772
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 175
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2776
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2776
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 176
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2780
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2780
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 177
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2784
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2784
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 178
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2788
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2788
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 179
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2792
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2792
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 180
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2796
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2796
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 181
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2800
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2800
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 182
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2804
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2804
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 183
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2808
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2808
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 184
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2812
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2812
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 185
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2816
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2816
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 186
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2820
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2820
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 187
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2824
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2824
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 188
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2828
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2828
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 189
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2832
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2832
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 190
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2836
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2836
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 191
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2840
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2840
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 192
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2844
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2844
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 193
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2848
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2848
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 194
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2852
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2852
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 195
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2856
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2856
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 196
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2860
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2860
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 197
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2864
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2864
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 198
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2868
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2868
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 199
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2872
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2872
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 200
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2876
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2876
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 201
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2880
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2880
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 202
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2884
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2884
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 203
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2888
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2888
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 204
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2892
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2892
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 205
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2896
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2896
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 206
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2900
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2900
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 207
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2904
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2904
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 208
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2908
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2908
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 209
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2912
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2912
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 210
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2916
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2916
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 211
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2920
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2920
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 212
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2924
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2924
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 213
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2928
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2928
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 214
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2932
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2932
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 215
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2936
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2936
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 216
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2940
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2940
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 217
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2944
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2944
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 218
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2948
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2948
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 219
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2952
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2952
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 220
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2956
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2956
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 221
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2960
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2960
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 222
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2964
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2964
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 223
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2968
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2968
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 224
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2972
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2972
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 225
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2976
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2976
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 226
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2980
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2980
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 227
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2984
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2984
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 228
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2988
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2988
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 229
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2992
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2992
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 230
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 2996
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2996
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 231
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3000
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3000
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 232
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3004
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3004
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 233
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3008
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3008
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 234
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3012
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3012
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 235
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3016
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3016
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 236
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3020
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3020
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 237
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3024
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3024
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 238
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3028
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3028
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 239
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3032
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3032
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 240
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3036
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3036
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 241
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3040
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3040
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 242
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3044
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3044
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 243
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3048
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3048
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 244
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3052
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3052
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 245
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3056
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3056
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 246
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3060
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3060
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 247
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3064
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3064
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 248
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3068
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3068
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 249
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3072
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3072
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 250
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3076
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3076
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 251
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3080
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3080
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 252
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3084
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3084
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 253
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3088
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3088
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 254
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3092
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3092
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 255
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3096
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3096
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 256
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3100
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3100
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 257
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3104
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3104
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 258
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3108
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3108
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 259
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3112
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3112
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 260
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3116
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3116
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 261
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3120
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3120
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 262
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3124
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3124
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 263
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3128
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3128
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 264
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3132
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3132
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 265
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3136
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3136
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 266
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3140
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3140
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 267
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3144
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3144
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 268
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3148
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3148
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 269
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3152
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3152
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 270
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3156
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3156
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 271
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3160
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3160
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 272
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3164
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3164
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 273
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3168
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3168
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 274
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3172
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3172
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 275
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3176
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3176
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 276
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3180
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3180
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 277
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3184
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3184
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 278
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3188
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3188
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 279
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3192
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3192
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 280
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3196
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3196
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 281
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3200
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3200
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 282
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3204
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3204
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 283
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3208
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3208
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 284
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3212
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3212
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 285
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3216
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3216
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 286
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3220
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3220
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 287
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3224
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3224
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 288
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3228
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3228
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 289
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3232
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3232
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 290
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3236
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3236
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 291
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3240
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3240
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 292
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3244
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3244
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 293
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3248
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3248
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 294
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3252
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3252
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 295
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3256
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3256
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 296
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3260
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3260
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 297
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3264
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3264
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 298
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3268
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3268
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 299
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3272
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3272
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 300
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3276
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3276
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 301
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3280
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3280
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 302
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3284
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3284
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 303
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3288
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3288
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 304
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3292
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3292
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 305
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3296
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3296
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 306
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3300
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3300
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 307
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3304
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3304
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 308
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3308
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3308
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 309
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3312
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3312
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 310
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3316
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3316
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 311
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3320
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3320
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 312
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3324
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3324
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 313
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3328
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3328
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 314
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3332
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3332
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 315
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3336
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3336
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 316
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3340
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3340
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 317
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3344
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3344
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 318
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3348
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3348
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 319
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3352
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3352
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 320
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3356
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3356
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 321
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3360
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3360
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 322
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3364
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3364
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 323
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3368
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3368
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 324
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3372
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3372
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 325
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3376
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3376
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 326
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3380
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3380
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 327
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3384
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3384
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 328
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3388
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3388
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 329
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3392
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3392
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 330
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3396
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3396
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 331
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3400
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3400
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 332
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3404
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3404
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 333
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3408
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3408
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 334
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3412
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3412
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 335
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3416
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3416
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 336
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3420
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3420
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 337
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3424
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3424
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 338
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3428
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3428
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 339
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3432
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3432
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 340
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3436
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3436
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 341
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3440
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3440
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 342
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3444
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3444
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 343
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3448
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3448
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 344
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3452
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3452
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 345
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3456
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3456
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 346
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3460
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3460
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 347
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3464
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3464
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 348
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3468
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3468
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 349
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3472
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3472
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 350
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3476
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3476
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 351
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3480
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3480
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 352
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3484
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3484
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 353
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3488
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3488
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 354
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3492
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3492
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 355
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3496
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3496
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 356
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3500
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3500
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 357
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3504
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3504
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 358
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3508
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3508
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 359
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3512
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3512
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 360
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3516
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3516
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 361
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3520
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3520
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 362
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3524
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3524
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 363
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3528
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3528
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 364
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3532
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3532
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 365
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3536
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3536
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 366
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3540
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3540
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 367
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3544
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3544
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 368
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3548
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3548
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 369
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3552
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3552
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 370
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3556
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3556
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 371
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3560
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3560
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 372
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3564
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3564
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 373
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3568
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3568
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 374
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3572
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3572
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 375
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3576
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3576
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 376
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3580
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3580
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 377
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3584
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3584
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 378
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3588
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3588
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 379
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3592
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3592
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 380
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3596
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3596
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 381
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3600
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3600
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 382
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3604
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3604
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 383
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3608
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3608
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 384
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3612
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3612
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 385
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3616
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3616
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 386
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3620
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3620
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 387
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3624
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3624
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 388
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3628
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3628
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 389
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3632
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3632
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 390
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3636
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3636
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 391
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3640
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3640
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 392
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3644
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3644
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 393
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3648
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3648
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 394
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3652
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3652
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 395
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3656
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3656
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 396
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3660
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3660
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 397
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3664
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3664
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 398
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3668
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3668
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 399
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3672
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3672
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 400
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3676
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3676
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 401
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3680
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3680
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 402
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3684
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3684
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 403
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3688
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3688
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 404
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3692
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3692
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 405
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3696
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3696
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 406
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3700
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3700
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 407
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3704
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3704
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 408
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3708
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3708
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 409
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3712
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3712
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 410
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3716
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3716
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 411
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3720
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3720
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 412
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3724
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3724
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 413
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3728
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3728
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 414
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3732
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3732
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 415
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3736
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3736
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 416
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3740
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3740
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 417
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3744
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3744
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 418
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3748
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3748
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 419
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3752
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3752
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 420
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3756
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3756
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 421
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3760
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3760
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 422
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3764
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3764
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 423
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3768
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3768
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 424
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3772
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3772
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 425
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3776
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3776
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 426
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3780
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3780
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 427
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3784
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3784
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 428
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3788
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3788
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 429
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3792
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3792
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 430
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3796
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3796
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 431
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3800
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3800
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 432
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3804
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3804
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 433
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3808
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3808
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 434
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3812
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3812
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 435
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3816
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3816
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 436
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3820
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3820
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 437
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3824
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3824
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 438
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3828
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3828
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 439
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3832
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3832
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 440
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3836
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3836
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 441
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3840
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3840
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 442
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3844
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3844
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 443
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3848
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3848
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 444
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3852
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3852
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 445
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3856
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3856
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 446
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3860
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3860
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 447
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3864
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3864
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 448
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3868
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3868
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 449
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3872
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3872
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 450
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3876
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3876
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 451
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3880
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3880
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 452
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3884
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3884
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 453
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3888
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3888
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 454
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3892
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3892
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 455
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3896
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3896
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 456
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3900
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3900
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 457
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3904
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3904
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 458
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3908
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3908
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 459
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3912
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3912
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 460
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3916
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3916
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 461
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3920
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3920
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 462
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3924
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3924
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 463
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3928
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3928
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 464
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3932
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3932
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 465
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3936
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3936
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 466
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3940
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3940
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 467
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3944
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3944
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 468
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3948
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3948
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 469
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3952
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3952
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 470
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3956
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3956
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 471
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3960
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3960
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 472
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3964
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3964
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 473
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3968
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3968
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 474
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3972
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3972
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 475
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3976
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3976
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 476
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3980
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3980
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 477
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3984
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3984
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 478
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3988
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3988
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 479
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3992
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3992
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 480
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 3996
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 3996
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 481
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 4000
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4000
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 482
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 4004
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4004
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 483
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 4008
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4008
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 484
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 4012
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4012
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 485
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 4016
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4016
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 486
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 4020
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4020
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 487
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 4024
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4024
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 488
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 4028
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4028
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 489
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 4032
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4032
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 490
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 4036
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4036
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 491
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 4040
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4040
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 492
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 4044
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4044
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 493
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 4048
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4048
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 494
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 4052
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4052
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 495
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 4056
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4056
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 496
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 4060
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4060
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 497
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 4064
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4064
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 498
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 4068
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4068
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 499
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 4072
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4072
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 500
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 4076
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4076
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 501
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 4080
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4080
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 502
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 4084
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4084
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 503
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 4088
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4088
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 504
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 4092
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4092
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 505
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 4096
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4096
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 506
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 4100
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4100
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 507
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 4104
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4104
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 508
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 4108
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4108
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 509
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 4112
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4112
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 510
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 4116
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4116
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 511
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 4120
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4120
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  li t0, 4124
  add t0, sp, t0
  li t1, 4128
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4128
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  la t0, output_length
  sw zero, 0(t0)
  j LBB11_1
LBB11_1:
  lw t0, 4(sp)
  lw t0, 0(t0)
  li t1, 4132
  add t1, sp, t1
  sw t0, 0(t1)
  la t0, program_length
  lw t0, 0(t0)
  li t1, 4136
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4132
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 4136
  add t1, sp, t1
  lw t1, 0(t1)
  slt t0, t0, t1
  li t2, 4140
  add t2, sp, t2
  sw t0, 0(t2)
  li t0, 4140
  add t0, sp, t0
  lw t0, 0(t0)
  bnez t0, LBB11_2
  j LBB11_42
LBB11_2:
  li t0, 4144
  add t0, sp, t0
  li t1, 4148
  add t1, sp, t1
  sw t0, 0(t1)
  lw t0, 4(sp)
  lw t0, 0(t0)
  li t1, 4152
  add t1, sp, t1
  sw t0, 0(t1)
  la t0, program
  li t1, 4152
  add t1, sp, t1
  lw t1, 0(t1)
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 4156
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4156
  add t0, sp, t0
  lw t0, 0(t0)
  lw t0, 0(t0)
  li t1, 4160
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4160
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 4148
  add t1, sp, t1
  lw t1, 0(t1)
  sw t0, 0(t1)
  li t0, 4148
  add t0, sp, t0
  lw t0, 0(t0)
  lw t0, 0(t0)
  li t1, 4164
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4164
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 62
  xor t0, t0, t1
  seqz t2, t0
  li a0, 4168
  add a0, sp, a0
  sw t2, 0(a0)
  li t0, 4168
  add t0, sp, t0
  lw t0, 0(t0)
  bnez t0, LBB11_3
  j LBB11_4
LBB11_3:
  lw t0, 12(sp)
  lw t0, 0(t0)
  li t1, 4172
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4172
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 1
  add t0, t0, t1
  li t2, 4176
  add t2, sp, t2
  sw t0, 0(t2)
  li t0, 4176
  add t0, sp, t0
  lw t0, 0(t0)
  lw t1, 12(sp)
  sw t0, 0(t1)
  j LBB11_41
LBB11_4:
  li t0, 4148
  add t0, sp, t0
  lw t0, 0(t0)
  lw t0, 0(t0)
  li t1, 4180
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4180
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 60
  xor t0, t0, t1
  seqz t2, t0
  li a0, 4184
  add a0, sp, a0
  sw t2, 0(a0)
  li t0, 4184
  add t0, sp, t0
  lw t0, 0(t0)
  bnez t0, LBB11_5
  j LBB11_6
LBB11_5:
  lw t0, 12(sp)
  lw t0, 0(t0)
  li t1, 4188
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4188
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 1
  sub t0, t0, t1
  li t2, 4192
  add t2, sp, t2
  sw t0, 0(t2)
  li t0, 4192
  add t0, sp, t0
  lw t0, 0(t0)
  lw t1, 12(sp)
  sw t0, 0(t1)
  j LBB11_40
LBB11_6:
  li t0, 4148
  add t0, sp, t0
  lw t0, 0(t0)
  lw t0, 0(t0)
  li t1, 4196
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4196
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 43
  xor t0, t0, t1
  seqz t2, t0
  li a0, 4200
  add a0, sp, a0
  sw t2, 0(a0)
  li t0, 4200
  add t0, sp, t0
  lw t0, 0(t0)
  bnez t0, LBB11_7
  j LBB11_8
LBB11_7:
  lw t0, 12(sp)
  lw t0, 0(t0)
  li t1, 4204
  add t1, sp, t1
  sw t0, 0(t1)
  la t0, tape
  li t1, 4204
  add t1, sp, t1
  lw t1, 0(t1)
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 4208
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4208
  add t0, sp, t0
  lw t0, 0(t0)
  lw t0, 0(t0)
  li t1, 4212
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4212
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 1
  add t0, t0, t1
  li t2, 4216
  add t2, sp, t2
  sw t0, 0(t2)
  lw t0, 12(sp)
  lw t0, 0(t0)
  li t1, 4220
  add t1, sp, t1
  sw t0, 0(t1)
  la t0, tape
  li t1, 4220
  add t1, sp, t1
  lw t1, 0(t1)
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 4224
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4216
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 4224
  add t1, sp, t1
  lw t1, 0(t1)
  sw t0, 0(t1)
  j LBB11_39
LBB11_8:
  li t0, 4148
  add t0, sp, t0
  lw t0, 0(t0)
  lw t0, 0(t0)
  li t1, 4228
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4228
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 45
  xor t0, t0, t1
  seqz t2, t0
  li a0, 4232
  add a0, sp, a0
  sw t2, 0(a0)
  li t0, 4232
  add t0, sp, t0
  lw t0, 0(t0)
  bnez t0, LBB11_9
  j LBB11_10
LBB11_9:
  lw t0, 12(sp)
  lw t0, 0(t0)
  li t1, 4236
  add t1, sp, t1
  sw t0, 0(t1)
  la t0, tape
  li t1, 4236
  add t1, sp, t1
  lw t1, 0(t1)
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 4240
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4240
  add t0, sp, t0
  lw t0, 0(t0)
  lw t0, 0(t0)
  li t1, 4244
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4244
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 1
  sub t0, t0, t1
  li t2, 4248
  add t2, sp, t2
  sw t0, 0(t2)
  lw t0, 12(sp)
  lw t0, 0(t0)
  li t1, 4252
  add t1, sp, t1
  sw t0, 0(t1)
  la t0, tape
  li t1, 4252
  add t1, sp, t1
  lw t1, 0(t1)
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 4256
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4248
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 4256
  add t1, sp, t1
  lw t1, 0(t1)
  sw t0, 0(t1)
  j LBB11_38
LBB11_10:
  li t0, 4148
  add t0, sp, t0
  lw t0, 0(t0)
  lw t0, 0(t0)
  li t1, 4260
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4260
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 91
  xor t0, t0, t1
  seqz t2, t0
  li a0, 4264
  add a0, sp, a0
  sw t2, 0(a0)
  li t0, 4264
  add t0, sp, t0
  lw t0, 0(t0)
  bnez t0, LBB11_11
  j LBB11_22
LBB11_11:
  li t0, 4268
  add t0, sp, t0
  li t1, 4272
  add t1, sp, t1
  sw t0, 0(t1)
  lw t0, 12(sp)
  lw t0, 0(t0)
  li t1, 4276
  add t1, sp, t1
  sw t0, 0(t1)
  la t0, tape
  li t1, 4276
  add t1, sp, t1
  lw t1, 0(t1)
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 4280
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4280
  add t0, sp, t0
  lw t0, 0(t0)
  lw t0, 0(t0)
  li t1, 4284
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4284
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 4272
  add t1, sp, t1
  lw t1, 0(t1)
  sw t0, 0(t1)
  li t0, 4272
  add t0, sp, t0
  lw t0, 0(t0)
  lw t0, 0(t0)
  li t1, 4288
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4288
  add t0, sp, t0
  lw t0, 0(t0)
  snez t1, t0
  li t2, 4292
  add t2, sp, t2
  sw t1, 0(t2)
  li t0, 4292
  add t0, sp, t0
  lw t0, 0(t0)
  bnez t0, LBB11_12
  j LBB11_13
LBB11_12:
  lw t0, 4(sp)
  lw t0, 0(t0)
  li t1, 4296
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4128
  add t0, sp, t0
  lw t0, 0(t0)
  lw t0, 0(t0)
  li t1, 4300
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 4300
  add t1, sp, t1
  lw t1, 0(t1)
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 4304
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4296
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 4304
  add t1, sp, t1
  lw t1, 0(t1)
  sw t0, 0(t1)
  li t0, 4128
  add t0, sp, t0
  lw t0, 0(t0)
  lw t0, 0(t0)
  li t1, 4308
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4308
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 1
  add t0, t0, t1
  li t2, 4312
  add t2, sp, t2
  sw t0, 0(t2)
  li t0, 4312
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 4128
  add t1, sp, t1
  lw t1, 0(t1)
  sw t0, 0(t1)
  j LBB11_21
LBB11_13:
  li t0, 4316
  add t0, sp, t0
  li t1, 4320
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 1
  li t1, 4320
  add t1, sp, t1
  lw t1, 0(t1)
  sw t0, 0(t1)
  j LBB11_14
LBB11_14:
  li t0, 4320
  add t0, sp, t0
  lw t0, 0(t0)
  lw t0, 0(t0)
  li t1, 4324
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4324
  add t0, sp, t0
  lw t0, 0(t0)
  sgt t0, t0, zero
  li t1, 4328
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4328
  add t0, sp, t0
  lw t0, 0(t0)
  bnez t0, LBB11_15
  j LBB11_20
LBB11_15:
  lw t0, 4(sp)
  lw t0, 0(t0)
  li t1, 4332
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4332
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 1
  add t0, t0, t1
  li t2, 4336
  add t2, sp, t2
  sw t0, 0(t2)
  li t0, 4336
  add t0, sp, t0
  lw t0, 0(t0)
  lw t1, 4(sp)
  sw t0, 0(t1)
  lw t0, 4(sp)
  lw t0, 0(t0)
  li t1, 4340
  add t1, sp, t1
  sw t0, 0(t1)
  la t0, program
  li t1, 4340
  add t1, sp, t1
  lw t1, 0(t1)
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 4344
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4344
  add t0, sp, t0
  lw t0, 0(t0)
  lw t0, 0(t0)
  li t1, 4348
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4348
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 93
  xor t0, t0, t1
  seqz t2, t0
  li a0, 4352
  add a0, sp, a0
  sw t2, 0(a0)
  li t0, 4352
  add t0, sp, t0
  lw t0, 0(t0)
  bnez t0, LBB11_16
  j LBB11_17
LBB11_16:
  li t0, 4320
  add t0, sp, t0
  lw t0, 0(t0)
  lw t0, 0(t0)
  li t1, 4356
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4356
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 1
  sub t0, t0, t1
  li t2, 4360
  add t2, sp, t2
  sw t0, 0(t2)
  li t0, 4360
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 4320
  add t1, sp, t1
  lw t1, 0(t1)
  sw t0, 0(t1)
  j LBB11_17
LBB11_17:
  lw t0, 4(sp)
  lw t0, 0(t0)
  li t1, 4364
  add t1, sp, t1
  sw t0, 0(t1)
  la t0, program
  li t1, 4364
  add t1, sp, t1
  lw t1, 0(t1)
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 4368
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4368
  add t0, sp, t0
  lw t0, 0(t0)
  lw t0, 0(t0)
  li t1, 4372
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4372
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 91
  xor t0, t0, t1
  seqz t2, t0
  li a0, 4376
  add a0, sp, a0
  sw t2, 0(a0)
  li t0, 4376
  add t0, sp, t0
  lw t0, 0(t0)
  bnez t0, LBB11_18
  j LBB11_19
LBB11_18:
  li t0, 4320
  add t0, sp, t0
  lw t0, 0(t0)
  lw t0, 0(t0)
  li t1, 4380
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4380
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 1
  add t0, t0, t1
  li t2, 4384
  add t2, sp, t2
  sw t0, 0(t2)
  li t0, 4384
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 4320
  add t1, sp, t1
  lw t1, 0(t1)
  sw t0, 0(t1)
  j LBB11_19
LBB11_19:
  j LBB11_14
LBB11_20:
  j LBB11_21
LBB11_21:
  j LBB11_37
LBB11_22:
  li t0, 4148
  add t0, sp, t0
  lw t0, 0(t0)
  lw t0, 0(t0)
  li t1, 4388
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4388
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 93
  xor t0, t0, t1
  seqz t2, t0
  li a0, 4392
  add a0, sp, a0
  sw t2, 0(a0)
  li t0, 4392
  add t0, sp, t0
  lw t0, 0(t0)
  bnez t0, LBB11_23
  j LBB11_27
LBB11_23:
  li t0, 4396
  add t0, sp, t0
  li t1, 4400
  add t1, sp, t1
  sw t0, 0(t1)
  lw t0, 12(sp)
  lw t0, 0(t0)
  li t1, 4404
  add t1, sp, t1
  sw t0, 0(t1)
  la t0, tape
  li t1, 4404
  add t1, sp, t1
  lw t1, 0(t1)
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 4408
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4408
  add t0, sp, t0
  lw t0, 0(t0)
  lw t0, 0(t0)
  li t1, 4412
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4412
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 4400
  add t1, sp, t1
  lw t1, 0(t1)
  sw t0, 0(t1)
  li t0, 4400
  add t0, sp, t0
  lw t0, 0(t0)
  lw t0, 0(t0)
  li t1, 4416
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4416
  add t0, sp, t0
  lw t0, 0(t0)
  seqz t1, t0
  li t2, 4420
  add t2, sp, t2
  sw t1, 0(t2)
  li t0, 4420
  add t0, sp, t0
  lw t0, 0(t0)
  bnez t0, LBB11_24
  j LBB11_25
LBB11_24:
  li t0, 4128
  add t0, sp, t0
  lw t0, 0(t0)
  lw t0, 0(t0)
  li t1, 4424
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4424
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 1
  sub t0, t0, t1
  li t2, 4428
  add t2, sp, t2
  sw t0, 0(t2)
  li t0, 4428
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 4128
  add t1, sp, t1
  lw t1, 0(t1)
  sw t0, 0(t1)
  j LBB11_26
LBB11_25:
  li t0, 4128
  add t0, sp, t0
  lw t0, 0(t0)
  lw t0, 0(t0)
  li t1, 4432
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4432
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 1
  sub t0, t0, t1
  li t2, 4436
  add t2, sp, t2
  sw t0, 0(t2)
  li t0, 2072
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 4436
  add t1, sp, t1
  lw t1, 0(t1)
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 4440
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4440
  add t0, sp, t0
  lw t0, 0(t0)
  lw t0, 0(t0)
  li t1, 4444
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4444
  add t0, sp, t0
  lw t0, 0(t0)
  lw t1, 4(sp)
  sw t0, 0(t1)
  j LBB11_26
LBB11_26:
  j LBB11_36
LBB11_27:
  li t0, 4148
  add t0, sp, t0
  lw t0, 0(t0)
  lw t0, 0(t0)
  li t1, 4448
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4448
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 46
  xor t0, t0, t1
  seqz t2, t0
  li a0, 4452
  add a0, sp, a0
  sw t2, 0(a0)
  li t0, 4452
  add t0, sp, t0
  lw t0, 0(t0)
  bnez t0, LBB11_28
  j LBB11_29
LBB11_28:
  lw t0, 12(sp)
  lw t0, 0(t0)
  li t1, 4456
  add t1, sp, t1
  sw t0, 0(t1)
  la t0, tape
  li t1, 4456
  add t1, sp, t1
  lw t1, 0(t1)
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 4460
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4460
  add t0, sp, t0
  lw t0, 0(t0)
  lw t0, 0(t0)
  li t1, 4464
  add t1, sp, t1
  sw t0, 0(t1)
  la t0, output_length
  lw t0, 0(t0)
  li t1, 4468
  add t1, sp, t1
  sw t0, 0(t1)
  la t0, output
  li t1, 4468
  add t1, sp, t1
  lw t1, 0(t1)
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 4472
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4464
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 4472
  add t1, sp, t1
  lw t1, 0(t1)
  sw t0, 0(t1)
  la t0, output_length
  lw t0, 0(t0)
  li t1, 4476
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4476
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 1
  add t0, t0, t1
  li t2, 4480
  add t2, sp, t2
  sw t0, 0(t2)
  li t0, 4480
  add t0, sp, t0
  lw t0, 0(t0)
  la t1, output_length
  sw t0, 0(t1)
  j LBB11_35
LBB11_29:
  li t0, 4148
  add t0, sp, t0
  lw t0, 0(t0)
  lw t0, 0(t0)
  li t1, 4484
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4484
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 44
  xor t0, t0, t1
  seqz t2, t0
  li a0, 4488
  add a0, sp, a0
  sw t2, 0(a0)
  li t0, 4488
  add t0, sp, t0
  lw t0, 0(t0)
  bnez t0, LBB11_30
  j LBB11_34
LBB11_30:
  lw t0, 20(sp)
  lw t0, 0(t0)
  li t1, 4492
  add t1, sp, t1
  sw t0, 0(t1)
  la t0, input_length
  lw t0, 0(t0)
  li t1, 4496
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4492
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 4496
  add t1, sp, t1
  lw t1, 0(t1)
  slt t2, t0, t1
  seqz t2, t2
  li a0, 4500
  add a0, sp, a0
  sw t2, 0(a0)
  li t0, 4500
  add t0, sp, t0
  lw t0, 0(t0)
  bnez t0, LBB11_31
  j LBB11_32
LBB11_31:
  lw t0, 12(sp)
  lw t0, 0(t0)
  li t1, 4504
  add t1, sp, t1
  sw t0, 0(t1)
  la t0, tape
  li t1, 4504
  add t1, sp, t1
  lw t1, 0(t1)
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 4508
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4508
  add t0, sp, t0
  lw t0, 0(t0)
  sw zero, 0(t0)
  j LBB11_33
LBB11_32:
  lw t0, 20(sp)
  lw t0, 0(t0)
  li t1, 4512
  add t1, sp, t1
  sw t0, 0(t1)
  la t0, input
  li t1, 4512
  add t1, sp, t1
  lw t1, 0(t1)
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 4516
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4516
  add t0, sp, t0
  lw t0, 0(t0)
  lw t0, 0(t0)
  li t1, 4520
  add t1, sp, t1
  sw t0, 0(t1)
  lw t0, 12(sp)
  lw t0, 0(t0)
  li t1, 4524
  add t1, sp, t1
  sw t0, 0(t1)
  la t0, tape
  li t1, 4524
  add t1, sp, t1
  lw t1, 0(t1)
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  li t1, 4528
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4520
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 4528
  add t1, sp, t1
  lw t1, 0(t1)
  sw t0, 0(t1)
  lw t0, 20(sp)
  lw t0, 0(t0)
  li t1, 4532
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4532
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 1
  add t0, t0, t1
  li t2, 4536
  add t2, sp, t2
  sw t0, 0(t2)
  li t0, 4536
  add t0, sp, t0
  lw t0, 0(t0)
  lw t1, 20(sp)
  sw t0, 0(t1)
  j LBB11_33
LBB11_33:
  j LBB11_34
LBB11_34:
  j LBB11_35
LBB11_35:
  j LBB11_36
LBB11_36:
  j LBB11_37
LBB11_37:
  j LBB11_38
LBB11_38:
  j LBB11_39
LBB11_39:
  j LBB11_40
LBB11_40:
  j LBB11_41
LBB11_41:
  lw t0, 4(sp)
  lw t0, 0(t0)
  li t1, 4540
  add t1, sp, t1
  sw t0, 0(t1)
  li t0, 4540
  add t0, sp, t0
  lw t0, 0(t0)
  li t1, 1
  add t0, t0, t1
  li t2, 4544
  add t2, sp, t2
  sw t0, 0(t2)
  li t0, 4544
  add t0, sp, t0
  lw t0, 0(t0)
  lw t1, 4(sp)
  sw t0, 0(t1)
  j LBB11_1
LBB11_42:
  li t1, 4548
  add t1, sp, t1
  lw ra, 0(t1)
  li t1, 4560
  add sp, sp, t1
  ret

  .globl output_
output_:
  addi sp, sp, -48
  sw ra, 40(sp)
  addi t0, sp, 0
  addi t1, sp, 4
  sw t0, 0(t1)
  lw t0, 4(sp)
  sw zero, 0(t0)
  j LBB12_1
LBB12_1:
  lw t0, 4(sp)
  lw t0, 0(t0)
  sw t0, 8(sp)
  la t0, output_length
  lw t0, 0(t0)
  sw t0, 12(sp)
  lw t0, 8(sp)
  lw t1, 12(sp)
  slt t0, t0, t1
  sw t0, 16(sp)
  lw t0, 16(sp)
  bnez t0, LBB12_2
  j LBB12_3
LBB12_2:
  lw t0, 4(sp)
  lw t0, 0(t0)
  sw t0, 20(sp)
  la t0, output
  lw t1, 20(sp)
  li t2, 4
  mul t2, t1, t2
  add t0, t0, t2
  sw t0, 24(sp)
  lw t0, 24(sp)
  lw t0, 0(t0)
  sw t0, 28(sp)
  lw t0, 28(sp)
  mv a0, t0
  call putch
  lw t0, 4(sp)
  lw t0, 0(t0)
  sw t0, 32(sp)
  lw t0, 32(sp)
  li t1, 1
  add t0, t0, t1
  sw t0, 36(sp)
  lw t0, 36(sp)
  lw t1, 4(sp)
  sw t0, 0(t1)
  j LBB12_1
LBB12_3:
  lw ra, 40(sp)
  addi sp, sp, 48
  ret

  .globl main
main:
  addi sp, sp, -16
  sw ra, 0(sp)
  call read_program
  call starttime
  call run_program
  call stoptime
  call output_
  mv a0, zero
  lw ra, 0(sp)
  addi sp, sp, 16
  ret

