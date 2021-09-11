import os
import time
import subprocess

def main():
    if not os.path.exists("ans"):
        print("cannot find ans folder")
        return

    files = os.listdir("ans")
    for file in files:
        # for every ans, get the corresponding result from user's result
        if not os.path.exists(file):
            print("cannot find corresponding sql script for " + file)
            continue
        
        print("*******************************")
        print("testing {}".format(file))
        start_time = time.perf_counter()
        command = "diff <(echo \".read ans/{}\" | sqlite3 musicbrainz-cmudb2020.db) <(echo \".read {}\" | sqlite3 musicbrainz-cmudb2020.db)".format(file, file)

        try:
            p = subprocess.Popen(command, shell=True, stdout=subprocess.STDOUT, executable="/bin/bash").wait()

        except Exception as e:
            print("Call of {} failed {}".format(command, e))
            return
        
        print(f"spent {time.perf_counter() - start_time:0.4f} seconds")

if __name__ == "__main__":
    main()
