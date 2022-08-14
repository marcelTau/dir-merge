#!/bin/bash

function test() {
    mkdir test1
    mkdir test2

    echo "equal" > test1/t1_equal
    echo "equal" > test2/t2_equal

    echo "not_equal1" > test1/t1_not_equal
    echo "not_equal2" > test2/t2_not_equal

    echo "TEST: $1"
    cargo run -- -A test1 -B test2 --action $1 --confirmation true

    tree test*

    rm -rf test1
    rm -rf test2
}

test diff
test equal
test merge_into_a
test merge_into_b
