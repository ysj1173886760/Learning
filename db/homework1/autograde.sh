#! /bin/bash
files=$(ls *.sql)
for i in $files
do
	echo "************************"
	echo "testing $i"
	SECONDS=0
	diff <(echo ".read ans/$i" | sqlite3 musicbrainz-cmudb2020.db) <(echo ".read $i" | sqlite3 musicbrainz-cmudb2020.db)
	echo "elapsed time $SECONDS"

	if [ $! -eq 0 ] 
	then
		echo "$i passed!"
	else
		echo "$i failed"
	fi
done