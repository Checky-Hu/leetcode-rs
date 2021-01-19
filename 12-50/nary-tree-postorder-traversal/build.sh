#!/bin/sh
if [ ! -d target ];then
    mkdir target
fi

gcc -Wall -g main.c -o target/main
