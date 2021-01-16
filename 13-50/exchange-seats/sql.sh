#!/bin/sh
SELECT IF(id%2=1, IF(id=(SELECT MAX(id) FROM seat), id, id+1), id-1) id, student
FROM seat
ORDER BY id
