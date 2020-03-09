#!/bin/bash

function run_it {

    valgrind target/debug/concurrency $1 $2 $3 $4 2>&1 |\
            grep -A 2 "HEAP SUMMARY" |\
            sed -e 's/==[0-9][0-9][0-9]*==//g'
    echo "===========================================================";
}

cargo -q build;

run_it 1 2 1 n;
run_it 2 3 1 n;
run_it 4 5 1 n;
run_it 8 9 1 n;


