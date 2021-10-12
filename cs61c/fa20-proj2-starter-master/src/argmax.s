.globl argmax

.text
# =================================================================
# FUNCTION: Given a int vector, return the index of the largest
#	element. If there are multiple, return the one
#	with the smallest index.
# Arguments:
# 	a0 (int*) is the pointer to the start of the vector
#	a1 (int)  is the # of elements in the vector
# Returns:
#	a0 (int)  is the first index of the largest element
# Exceptions:
# - If the length of the vector is less than 1,
#   this function terminates the program with error code 77.
# =================================================================
argmax:

    # Prologue

    li t0, 1
    bge a1, t0, correct
    li a1, 77
    j exit2

correct:
    mv t0, zero
    mv t2, zero     # max index
    lw t3, 0(a0)    # max value

loop_start:
    lw t1, 0(a0)
    ble t1, t3, loop_continue
    mv t3, t1
    mv t2, t0

loop_continue:
    addi t0, t0, 1
    addi a0, a0, 4
    beq t0, a1, loop_end
    j loop_start

loop_end:
    # Epilogue
    mv a0, t2

    ret
