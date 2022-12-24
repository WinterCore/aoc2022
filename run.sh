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

if [ $day == "day09" ]
then
    cargo run --bin day09
else
    rustc main.rs -o compiled && ./compiled
fi
