#!/bin/sh
CREATE FUNCTION getNthHighestSalary(N INT) RETURNS INT
BEGIN
  RETURN (
      SELECT DISTINCT Salary
      FROM (SELECT Salary, DENSE_RANK() OVER(ORDER BY Salary DESC) AS ranking FROM Employee) t
      WHERE ranking = N
  );
END
