#!/bin/bash

my_array=(1 2 3 4)

echo "* display"
for i in ${my_array[*]}; do
    echo $i
done

echo "@ display"
for i in ${my_array[@]}; do
    echo $i
done

echo "length of array is ${#my_array[@]}"

highD_array=($my_array 5 6 7)
for i in ${highD_array[*]}; do
    echo $i
done

