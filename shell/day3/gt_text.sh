#!/bin/bash

a=10
b=20

val=`expr $a > $b`
echo "a > b == $val"

if [ $b -gt $a ]
then
    echo "b is greater than a"
fi
