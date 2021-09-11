#! /bin/bash
for i in $(ls *.sql | sort)
do
	echo "[\"$i\"]="
done
