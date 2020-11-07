#!/bin/sh
SELECT high.id as Id
FROM Weather high
INNER JOIN Weather low
ON high.Temperature > low.Temperature AND DATE(high.recordDate) = DATE(low.recordDate + INTERVAL 1 DAY);
