import sys

## Takes in one argument, a string, and reads in <arg>.bmp to write out <arg>.bin
## For example, `python bmp_to_bin.py five` reads in five.bmp and outputs five.bin

## Read in .bmp file and skip past BMP header
infile = open(sys.argv[1] + ".bmp", "rb")
infile.seek(54)

## Write appropriate header (784 1) to binary file
index = 0
outfile = open(sys.argv[1] + ".bin", "wb+")
header_arr = [16, 3, 0, 0, 1, 0, 0, 0]
byte_header = bytearray(header_arr)
outfile.write(byte_header)

## Read in BMP file, differentiating between white pixels (r = g = b = 0xff) and non-white pixels
ff_byte = '0xFF'
ff_ba = bytearray([int(ff_byte, 16)])
toprint = ""
line = []
all_lines = []
while index < 784:
    r = infile.read(1)
    b = infile.read(1)
    g = infile.read(1)
    if (r == ff_ba and b == ff_ba and g == ff_ba):
        line.append(" ")
    else: 
        line.append("*")
    index = index + 1
    if (index % 28 == 0):
        all_lines.append(line)
        line = []

## Flip the image to the correct orientation
final_lines = []
final_arr = []
for i in range(28):
    final_lines.append([])
    final_arr.append([])
    for j in range(28):
        final_arr[i].append([])

bytes128 = bytearray([128, 0, 0, 0])
bytes0 = bytearray([0, 0, 0, 0])
for i in range(28):
    index = 27
    line_str = ""
    while (index >= 0): 
        line_str = all_lines[i][index] + line_str
        index = index - 1
        final_arr[28 - i - 1][index] = all_lines[i][index]
    final_lines[28 - i - 1] = line_str

## Print the image read in as ASCII art
for line in final_lines:
    print(line)


## Write the binary image to the .bin file
for i in range(28):
    for j in range(28):
       if final_arr[i][j] == "*":
           outfile.write(bytes128)
       else:
           outfile.write(bytes0)    

## Close input and output files
infile.close()
outfile.close()    
