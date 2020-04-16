#!/bin/sh
if [ ! -d target ];then
    mkdir target
fi

g++ -Wall -g main.cc -lpthread -o target/main
