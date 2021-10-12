.globl dot

.text
# =======================================================
# FUNCTION: Dot product of 2 int vectors
# Arguments:
#   a0 (int*) is the pointer to the start of v0
#   a1 (int*) is the pointer to the start of v1
#   a2 (int)  is the length of the vectors
#   a3 (int)  is the stride of v0
#   a4 (int)  is the stride of v1
# Returns:
#   a0 (int)  is the dot product of v0 and v1
# Exceptions:
# - If the length of the vector is less than 1,
#   this function terminates the program with error code 75.
# - If the stride of either vector is less than 1,
#   this function terminates the program with error code 76.
# =======================================================
dot:
    # Prologue
    li t0, 1
    blt a2, t0, error1
    blt a3, t0, error2
    blt a4, t0, error2
    j correct

error1:
    li a1, 75
    j exit2
error2:
    li a1, 76
    j exit2

correct:
    li t3, 0 # res
    li t4, 0 # i
    li t5, 0 # j

loop_start:
    lw t1, 0(a0)
    lw t2, 0(a1)
    mul t1, t1, t2
    add t3, t3, t1

loop_continue:
    li t1, 4
    mul t1, a3, t1
    add a0, a0, t1

    li t1, 4
    mul t1, a4, t1
    add a1, a1, t1

    add t4, t4, a3
    add t5, t5, a4
    bge t4, a2, loop_end
    # really?
    # bge t5, a2, loop_end

    j loop_start

loop_end:
    mv a0, t3
    # Epilogue
    
    ret
