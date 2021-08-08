#!/bin/bash

echo "$* display";
for i in "$*"; do
    echo $i
done

echo "$@ display";
for i in "$@"; do
    echo $i
done
