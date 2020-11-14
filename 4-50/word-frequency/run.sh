#!/bin/sh
tr -s ' ' '\n' < words.txt | sort | uniq -c | sort -rn | awk '{print $2,$1}'
