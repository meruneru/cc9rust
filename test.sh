#!/bin/bash

assert(){
    expected="$1"
    input="$2"

    ./target/debug/cc9rust "$input" > tmp.s
    cc -o tmp tmp.s
    ./tmp
    actual="$?"

    if [ "$actual" = "$expected" ]; then
        echo "$input => $actual"
    else
        echo "$input => $expected expected, but got $actual"
        exit 1
    fi
}

# assert <expected> <input>
assert 0 0
assert 123 123
assert 25 '10+40-25'

echo OK
