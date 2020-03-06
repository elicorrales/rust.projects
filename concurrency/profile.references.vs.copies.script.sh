#!/bin/bash

num_books=$1;
num_loops=$2;
delay_ms=$3;
is_random=$4;

if [ $# -ne 4 ];
then
        echo "books  loops  delay  random";
        exit 1;
fi;

function run_it {

    cargo -q build;
    #/usr/bin/time -v target/debug/concurrency n 2>&1 | grep -E "Maximum"
    valgrind target/debug/concurrency $num_books $num_loops $delay_ms $is_random 2>&1 | grep -A 1 "HEAP SUMMARY" 
    echo "===========================================================";
}

echo "==== doing  references ======================"
cp src/book.rs.global.static.using.book.references src/book.rs;
cp src/main.rs.global.static.using.book.references src/main.rs;
run_it;


echo "==== doing  my copying ======================"
cp src/book.rs.using.book.copies src/book.rs;
cp src/main.rs.using.book.copies src/main.rs;
run_it;


echo "==== doing  derived copy ======================"
cp src/book.rs.using.derived.copy src/book.rs;
cp src/main.rs.using.derived.copy src/main.rs;
run_it;


