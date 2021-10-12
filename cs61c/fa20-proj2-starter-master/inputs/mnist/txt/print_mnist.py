import sys

## This program takes one command line argument, a number, e.g. "5"
## It will look at "mnist_input<number>.txt", e.g. "mnist_input5.txt"

## Open the .txt version of the input file and add all lines to an array
file = open("inputs/mnist_input" + sys.argv[1] + ".txt", "r")
lines = []
for line in file:
    lines.append(line)

index = 0
toprint = ""

## Print a * if line is nonzero, and a space if it is 0, with a newline every 28 lines
while index < 784:
    if "0" in lines[index + 1]:
        toprint = toprint + " "
    else:
        toprint = toprint + "*"
    index = index + 1
    if index % 28 == 0:
        toprint = toprint + "\n"

## Check what this input file is classified as
classified_as = open("./labels/label" + sys.argv[1] + ".txt", "r")
number_classified = str(classified_as.read())

## Print the image as ASCII art
print("\n\nThis is the MNIST input image: \n\n" + toprint +
        "\n\nThe classifier matches it to the number " + number_classified)