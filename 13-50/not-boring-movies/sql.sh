#!/bin/bash
SELECT id, movie, description, rating
FROM cinema
WHERE description != "boring" AND id % 2 != 0
ORDER BY rating DESC
