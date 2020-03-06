#!/bin/bash

cp src/book.rs.global.static.using.book.references src/book.rs;
cp src/main.rs.global.static.using.book.references src/main.rs;

cargo build;

/usr/bin/time -v target/debug/concurrency 2>&1 | grep -E "(Maximum)|(Page size)"



cp src/book.rs.using.book.copies src/book.rs;
cp src/main.rs.using.book.copies src/main.rs;

cargo build;

/usr/bin/time -v target/debug/concurrency 2>&1 | grep -E "(Maximum)|(Page size)"



