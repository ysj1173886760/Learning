#!/usr/bin/env python

import sys
import argparse
import os

def ascii_to_binary(args):
    # Open input and output files
    input = open(args.input_file, 'r')
    output = open(args.output_file, 'wb')

    # Read in lines of input
    input_rows = input.readlines()

    # Read in strings representing integer dimensions, save as bytes to output
    dims = [int(s) for s in input_rows[0].split()]
    for d in dims:
        output.write(d.to_bytes(4, "little", signed=True))

    # Read in each row of the matrix
    for row in input_rows[1:]:
        # Read in float elements of each row, output as bytes to output
        int_row = [int(s) for s in row.split()]
        for num in int_row:
            output.write(num.to_bytes(4, "little", signed=True))

    input.close()
    output.close()

def binary_to_ascii(args):
    # Open input and output files
    input = open(args.input_file, 'rb')
    output = open(args.output_file, 'w')

    # Read in bytes representing integer dimensions, save as strngs to output
    rows = int.from_bytes(input.read(4), "little", signed=True)
    columns = int.from_bytes(input.read(4), "little", signed=True)
    output.write("{} {}{}".format(rows, columns, os.linesep))

    # Read in one row's worth of elements at a time
    for i in range(rows):
        curr_row = []
        # For each row, read in bytes representing floats, then save the row as a string to output
        for j in range(columns):
            val = int.from_bytes(input.read(4), "little", signed=True)
            curr_row.append(str(val))
        output.write(" ".join(curr_row) + os.linesep)

    input.close()
    output.close()

def main():
    parser = argparse.ArgumentParser(description="Converts between ascii and binary files representing integer matrices")
    parser.add_argument("input_file", help="file to read from")
    parser.add_argument("output_file", help="file to write to")
    group = parser.add_mutually_exclusive_group()
    group.add_argument("--to-binary", action="store_true", default=False, help="convert from ascii to binary")
    group.add_argument("--to-ascii", action="store_true", default=False, help="convert from binary to ascii")

    args = parser.parse_args()
    if args.to_binary:
        ascii_to_binary(args)
    elif args.to_ascii:
        binary_to_ascii(args)
    else:
        parser.error("Either --to-binary or --to-ascii must be specified")

if __name__ == "__main__":
    main()
