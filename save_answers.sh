#!/bin/bash

if [ "$#" -ne 4 ]; then
  echo "Usage: $0 <year> <day> <p1_answer> <p2_answer>"
  exit 1
fi

year=$1
day=$2
p1_answer=$3
p2_answer=$4

echo -n "$p1_answer" > "answer/${year}/day${day}p1.txt"
echo -n "$p2_answer" > "answer/${year}/day${day}p2.txt"