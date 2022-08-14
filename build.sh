#!/bin/bash

mkdir test1
mkdir test2

echo "equal" > test1/t1_equal
echo "equal" > test2/t2_equal

echo "not_equal1" > test1/t1_not_equal
echo "not_equal2" > test2/t2_not_equal

cargo run -- -A test1 -B test2 --action $1

tree test*

rm -rf test1
rm -rf test2
