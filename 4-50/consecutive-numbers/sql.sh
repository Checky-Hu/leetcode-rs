#!/bin/sh
SELECT distinct ConsecutiveNums from (
SELECT
CASE
WHEN Num = lag(Num) OVER (ORDER BY Id) and Num = lead(Num) OVER (ORDER BY Id)
THEN Num
END AS ConsecutiveNums
from Logs
) a
WHERE a.ConsecutiveNums IS NOT null
