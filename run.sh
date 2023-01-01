# !/bin/bash

day=$1

if [ ! -d "$day" ]
then
    echo "Usage: run.sh <dayxx>"
    echo "Please enter a valid day input"
    echo "Use the folder name of the solutions"
    exit 1
fi

cd $day

cargo run --bin "$@"
