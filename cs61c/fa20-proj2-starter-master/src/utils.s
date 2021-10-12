##############################################################
# Do not modify! (But feel free to use the functions provided)
##############################################################

#define c_print_int 1
#define c_print_str 4
#define c_atoi 5
#define c_sbrk 9
#define c_exit 10
#define c_print_char 11
#define c_openFile 13
#define c_readFile 14
#define c_writeFile 15
#define c_closeFile 16
#define c_exit2 17
#define c_fflush 18
#define c_feof 19
#define c_ferror 20
#define c_printHex 34

# ecall wrappers
.globl print_int, print_str, atoi, sbrk, exit, print_char, fopen, fread, fwrite, fclose, exit2, fflush, ferror, print_hex

# helper functions
.globl file_error, print_int_array, malloc, free, print_num_alloc_blocks, num_alloc_blocks

# unittest helper functions
.globl compare_int_array

.data
error_string: .string "This library file should not be directly called!"

.text
# Exits if you run this file
main:
    la a1 error_string
    jal print_str
    li a1 1
    jal exit2
# End main


#================================================================
# void print_int(int a1)
# Prints the integer in a1.
# args:
#   a1 = integer to print
# return:
#   void
#================================================================
print_int:
    li a0 c_print_int
    ecall
    ret


#================================================================
# void print_str(char *a1)
# Prints the null-terminated string at address a1.
# args:
#   a1 = address of the string you want printed.
# return:
#   void
#================================================================
print_str:
    li a0 c_print_str
    ecall
    ret


#================================================================
# int atoi(char* a1)
# Returns the integer version of the string at address a1.
# args:
#   a1 = address of the string you want to turn into an integer.
# return:
#   a0 = Integer representation of string 
#================================================================
atoi:
    li a0 c_atoi
    ecall
    ret


#================================================================
# void *sbrk(int a1)
# Allocates a1 bytes onto the heap.
# args:
#   a1 = Number of bytes you want to allocate.
# return:
#   a0 = Pointer to the start of the allocated memory
#================================================================
sbrk:
    li a0 c_sbrk
    ecall
    ret


#================================================================
# void noreturn exit()
# Exits the program with a zero exit code.
# args:
#   None
# return:
#   No Return
#================================================================
exit:
    li a0 c_exit
    ecall


#================================================================
# void print_char(char a1)
# Prints the ASCII character in a1 to the console.
# args:
#   a1 = character to print
# return:
#   void
#================================================================
print_char:
    li a0 c_print_char
    ecall
    ret


#================================================================
# int fopen(char *a1, int a2)
# Opens file with name a1 with permissions a2.
# args:
#   a1 = filepath
#   a2 = permissions (0, 1, 2, 3, 4, 5 = r, w, a, r+, w+, a+)
# return:
#   a0 = file descriptor
#================================================================
fopen:
    li a0 c_openFile
    ecall
    #FOPEN_RETURN_HOOK
    ret


#================================================================
# int fread(int a1, void *a2, size_t a3)
# Reads a3 bytes of the file into the buffer a2.
# args:
#   a1 = file descriptor
#   a2 = pointer to the buffer you want to write the read bytes to.
#   a3 = Number of bytes to be read.
# return:
#   a0 = Number of bytes actually read.
#================================================================
fread:
    li a0 c_readFile
    ecall
    #FREAD_RETURN_HOOK
    ret


#================================================================
# int fwrite(int a1, void *a2, size_t a3, size_t a4)
# Writes a3 * a4 bytes from the buffer in a2 to the file descriptor a1.
# args:
#   a1 = file descriptor
#   a2 = Buffer to read from
#   a3 = Number of items to read from the buffer.
#   a4 = Size of each item in the buffer.
# return:
#   a0 = Number of elements writen. If this is less than a3,
#    it is either an error or EOF. You will also need to still flush the fd.
#================================================================
fwrite:
    li a0 c_writeFile
    ecall
    #FWRITE_RETURN_HOOK
    ret


#================================================================
# int fclose(int a1)
# Closes the file descriptor a1.
# args:
#   a1 = file descriptor
# return:
#   a0 = 0 on success, and EOF (-1) otherwise.
#================================================================
fclose:
    li a0 c_closeFile
    ecall
    #FCLOSE_RETURN_HOOK
    ret


#================================================================
# void noreturn exit2(int a1)
# Exits the program with error code a1.
# args:
#   a1 = Exit code.
# return:
#   This program does not return.
#================================================================
exit2:
    li a0 c_exit2
    ecall
    ret


#================================================================
# int fflush(int a1)
# Flushes the data to the filesystem.
# args:
#   a1 = file descriptor
# return:
#   a0 = 0 on success, and EOF (-1) otherwise.
#================================================================
fflush:
    li a0 c_fflush
    ecall
    ret


#================================================================
# int ferror(int a1)
# Returns a nonzero value if the file stream has errors, otherwise it returns 0.
# args:
#   a1 = file descriptor
# return:
#   a0 = Nonzero falue if the end of file is reached. 0 Otherwise.
#================================================================
ferror:
    li a0 c_ferror
    ecall
    ret


#================================================================
# void print_hex(int a1)
#
# args:
#   a1 = The word which will be printed as a hex value.
# return:
#   void
#================================================================
print_hex:
    li a0 c_printHex
    ecall
    ret




#================================================================
# void* malloc(int a0)
# Allocates heap memory and return a pointer to it
# args:
#   a0 is the # of bytes to allocate heap memory for
# return:
#   a0 is the pointer to the allocated heap memory
#================================================================
malloc:
    # Call to sbrk
    mv a1 a0
    li a0 0x3CC
    addi a6 x0 1
    ecall
    #MALLOC_RETURN_HOOK
    ret


#================================================================
# void free(int a0)
# Frees heap memory referenced by pointer
# args:
#   a0 is the pointer to heap memory to free
# return:
#   void
#================================================================
free:
    mv a1 a0
    li a0 0x3CC
    addi a6 x0 4
    ecall
    ret

#================================================================
# void num_alloc_blocks(int a0)
# Returns the number of currently allocated blocks
# args:
#   void
# return:
#   a0 is the # of allocated blocks
#================================================================
num_alloc_blocks:
    li a0, 0x3CC
    li a6, 5
    ecall
    ret

print_num_alloc_blocks:
    addi sp, sp -4
    sw ra 0(sp)

    jal num_alloc_blocks
    mv a1 a0
    jal print_int

    li a1 '\n'
    jal print_char

    lw ra 0(sp)
    addi sp, sp 4
    ret

#================================================================
# void print_int_array(int* a0, int a1, int a2)
# Prints an integer array, with spaces between the elements
# args:
#   a0 is the pointer to the start of the array
#   a1 is the # of rows in the array
#   a2 is the # of columns in the array
# return:
#   void
#================================================================
print_int_array:
    # Prologue
    addi sp sp -24
    sw s0 0(sp)
    sw s1 4(sp)
    sw s2 8(sp)
    sw s3 12(sp)
    sw s4 16(sp)
    sw ra 20(sp)

    # Save arguments
    mv s0 a0
    mv s1 a1
    mv s2 a2

    # Set outer loop index
    li s3 0

outer_loop_start:
    # Check outer loop condition
    beq s3 s1 outer_loop_end

    # Set inner loop index
    li s4 0

inner_loop_start:
    # Check inner loop condition
    beq s4 s2 inner_loop_end

    # t0 = row index * len(row) + column index
    mul t0 s2 s3 
    add t0 t0 s4 
    slli t0 t0 2

    # Load matrix element
    add t0 t0 s0
    lw t1 0(t0)

    # Print matrix element
    mv a1 t1
    jal print_int

    # Print whitespace
    li a1 ' '
    jal print_char
    

    addi s4 s4 1
    j inner_loop_start

inner_loop_end:
    # Print newline
    li a1 '\n'
    jal print_char

    addi s3 s3 1
    j outer_loop_start

outer_loop_end:
    # Epilogue
    lw s0 0(sp)
    lw s1 4(sp)
    lw s2 8(sp)
    lw s3 12(sp)
    lw s4 16(sp)
    lw ra 20(sp)
    addi sp sp 24

    ret

#================================================================
# void compare_int_array(int a0, int* a0, int* a1, int a2)
# Prints an integer array, with spaces between the elements
# args:
#   a0 is the base exit code that will be used if an unequal element is found
#   a1 is the pointer to the expected data
#   a2 is the pointer to the actual data
#   a3 is the number of elements in each array
#   a4 is the error message
# return:
#   void
#================================================================
compare_int_array:
    # Prologue
    addi sp sp -24
    sw s0 0(sp)
    sw s1 4(sp)
    sw s2 8(sp)
    sw s3 12(sp)
    sw s4 16(sp)
    sw ra 20(sp)

    # save pointer to original array in s1
    mv s1, a2

    # t0: current element
    mv t0 zero

loop_start:
    # we are done once t0 >= a3
    bge t0, a3, end

    # t1 := *a1
    lw t1, 0(a1)
    # t2 := *a2
    lw t2, 0(a2)

    # if the values are different -> fail
    bne t1, t2, fail

    # go to next value
    addi t0, t0, 1
    addi a1, a1, 4
    addi a2, a2, 4
    j loop_start

fail:
    # exit code: a0
    mv s0, a0
    # remember length
    mv s2, a3

    # print user supplied error message
    mv a1, a4
    jal print_str

    # print actual data
    mv a0, s1
    li a1, 1
    mv a2, s2
    jal print_int_array

    # exit with user defined error code
    mv a1, s0
    jal exit2

end:
    # Epilogue
    lw s0 0(sp)
    lw s1 4(sp)
    lw s2 8(sp)
    lw s3 12(sp)
    lw s4 16(sp)
    lw ra 20(sp)
    addi sp sp 24

    ret
