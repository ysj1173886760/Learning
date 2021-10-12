.globl matmul

.text
# =======================================================
# FUNCTION: Matrix Multiplication of 2 integer matrices
# 	d = matmul(m0, m1)
# Arguments:
# 	a0 (int*)  is the pointer to the start of m0 
#	a1 (int)   is the # of rows (height) of m0
#	a2 (int)   is the # of columns (width) of m0
#	a3 (int*)  is the pointer to the start of m1
# 	a4 (int)   is the # of rows (height) of m1
#	a5 (int)   is the # of columns (width) of m1
#	a6 (int*)  is the pointer to the the start of d
# Returns:
#	None (void), sets d = matmul(m0, m1)
# Exceptions:
#   Make sure to check in top to bottom order!
#   - If the dimensions of m0 do not make sense,
#     this function terminates the program with exit code 72.
#   - If the dimensions of m1 do not make sense,
#     this function terminates the program with exit code 73.
#   - If the dimensions of m0 and m1 don't match,
#     this function terminates the program with exit code 74.
# =======================================================
matmul:

    # Error checks
    li t0, 1
    blt a1, t0, error1
    blt a2, t0, error1
    blt a4, t0, error2
    blt a5, t0, error2
    bne a2, a4, error3

    # Prologue

#    for (i = 0; i < row_a; i++) {
#        for (j = 0; j < col_b; j++) {
#            c[i][j] = dot(a, b, col_a, 1, row_b)
#        }
#    }

    addi sp, sp, -40
    sw s0, 0(sp)
    sw s1, 4(sp)
    sw s2, 8(sp)
    sw s3, 12(sp)
    sw s4, 16(sp)
    sw s5, 20(sp)
    sw s6, 24(sp)
    sw ra, 36(sp)
    mv s0, a0
    mv s1, a1
    mv s2, a2
    mv s3, a3
    mv s4, a4
    mv s5, a5
    mv s6, a6

    li, t0, 0   # i = 0
outer_loop_start:
    li, t1, 0   # j = 0

inner_loop_start:
    # a
    mv a0, s0
    # b + j
    mv a1, s3
    li t2, 4
    mul t2, t2, t1
    add a1, a1, t2

    mv a2, s2
    li a3, 1
    mv a4, s5
    sw t0, 28(sp)
    sw t1, 32(sp)

    jal dot
    sw a0, 0(s6)
    lw t0, 28(sp)
    lw t1, 32(sp)

inner_loop_continue:
    # j++
    addi t1, t1, 1
    addi s6, s6, 4
    beq t1, s5, inner_loop_end
    j inner_loop_start

inner_loop_end:
    addi t0, t0, 1
    beq t0, s1, outer_loop_end
    li t2, 4
    mul t2, t2, s2
    add s0, s0, t2
    j outer_loop_start

outer_loop_end:

    # Epilogue
    
    lw s0, 0(sp)
    lw s1, 4(sp)
    lw s2, 8(sp)
    lw s3, 12(sp)
    lw s4, 16(sp)
    lw s5, 20(sp)
    lw s6, 24(sp)
    lw ra, 36(sp)
    addi sp, sp, 40
    
    ret

error1:
    li a1, 72
    j exit2
error2:
    li a1, 73
    j exit2
error3:
    li a1, 74
    j exit2