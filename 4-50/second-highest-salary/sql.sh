#!/bin/sh
SELECT CASE
WHEN min(salary) = max(salary) THEN null ELSE min(salary) END
as SecondHighestSalary from
(select distinct salary from Employee order by Salary desc limit 2) as tmp
