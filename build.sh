#!/bin/bash

function test() {
    mkdir test1
    mkdir test2

    echo "equal" > test1/t1_equal
    echo "equal" > test2/t2_equal

    echo "not_equal1" > test1/t1_not_equal
    echo "not_equal2" > test2/t2_not_equal

    echo "TEST: $1"

    tree_check=0
    for var in "$@"; do
        if [[ $var == "test" ]]; then
            tree_check=1
        fi
    done

    if [[ tree_check -eq 1 ]]; then 
        tree test* >> /tmp/test_before
    fi



    tree test*
    cargo run -- -A test1 -B test2 --action $1 --confirmation true $2 $3 $4 $5
    tree test*

    if [[ tree_check -eq 1 ]]; then 
        tree test* >> /tmp/test_after

        if [[ `diff /tmp/test_before /tmp/test_after` -ne 0 ]]; then 
            echo "TEST FAILED, THE TEST_ONLY MODE DOES NOT WORK!!!!!!"
            exit 1
        fi
    fi

    rm -rf test1
    rm -rf test2
    rm -rf test3
}

test diff
test equal
test merge_into_a
test merge_into_b
test merge --merge test3
test merge --merge test3 --test true
test merge_into_a --test true
test merge_into_b --test true
