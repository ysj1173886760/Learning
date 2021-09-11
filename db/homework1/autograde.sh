#! /bin/bash
files=$(ls *.sql)
declare -A scores
scores=(
	["q1_sample.sql"]=0
	["q2_long_name.sql"]=5
	["q3_old_music_nations.sql"]=5
	["q4_dubbed_smash.sql"]=10
	["q5_vinyl_lover.sql"]=10
	["q6_old_is_not_gold.sql"]=10
	["q7_release_percentage.sql"]=15
	["q8_collaborate_artist.sql"]=15
	["q9_dre_and_eminem.sql"]=15
	["q10_around_the_world.sql"]=15
)
res=0
for i in $files
do
	if ! [ -s $i ]
	then
		echo "skipping $i"
		continue
	fi

	echo "************************"
	echo "testing $i"
	SECONDS=0
	diff <(echo ".read ans/$i" | sqlite3 musicbrainz-cmudb2020.db) <(echo ".read $i" | sqlite3 musicbrainz-cmudb2020.db)
	return_val=$?

	echo "elapsed time $SECONDS"
	if [ $return_val -eq 0 ] 
	then
		echo "$i passed!"
		(( res = $res + ${scores[$i]} ))
	else
		echo "$i failed"
	fi
done

echo "final score $res"