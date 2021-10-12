.globl abs

.text
# =================================================================
# FUNCTION: Given an int return its absolute value.
# Arguments:
# 	a0 (int) is input integer
# Returns:
#	a0 (int) the absolute value of the input
# =================================================================
abs:
    # Prologue

    # if positive, return directly
    bge a0, zero, res

    sub a0, zero, a0

    # Epilogue
res:
    ret
