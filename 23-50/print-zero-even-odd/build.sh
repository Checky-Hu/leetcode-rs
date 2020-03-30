#!/bin/sh
if [ ! -d target ];then
    mkdir target
fi

g++ -Wall -g main.c -lpthread -o target/main
