#!/bin/sh
WITH Salary AS (
    SELECT MAX(Salary) Sal, D.Id, D.Name
    from Employee E
    JOIN Department D ON E.DepartmentID = D.ID
    GROUP BY D.ID, D.Name)

SELECT S.Name Department, E.Name Employee, S.Sal Salary
FROM Employee E
JOIN Salary S ON E.DepartmentID = S.ID AND E.Salary = S.Sal
