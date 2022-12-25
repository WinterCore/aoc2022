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

# Days that require dependencies
if [ $day == "day09" ] || [ $day == "day10" ]
then
    cargo run --bin $day
else
    rustc main.rs -o compiled && ./compiled
fi
