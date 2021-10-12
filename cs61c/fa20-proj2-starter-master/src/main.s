.import read_matrix.s
.import write_matrix.s
.import matmul.s
.import dot.s
.import relu.s
.import argmax.s
.import utils.s
.import classify.s

.globl main
# This is a dummy main function which imports and calls the classify function.
# While it just exits right after, it could always call classify again.
main:
    # initialize register a2 to zero
    mv a2, zero

    # call classify function
    jal classify

    # exit program normally
    jal exit
