#!/bin/sh
col=$(head -n 1 file.txt | wc | awk '{print $2}')
for i in `seq 1 $col`; do
  awk -v i=$i '{print $i}' file.txt | xargs echo
done
