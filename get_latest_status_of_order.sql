SELECT  o.OrderId,
        s.OrderStatus,
        s.CreateDate
FROM dbo.Orders AS o
OUTER APPLY (
    SELECT TOP (1) os.OrderStatus, os.CreateDate
    FROM dbo.OrderStatus AS os
    WHERE os.OrderId = o.OrderId
    ORDER BY os.CreateDate DESC
) AS s
WHERE o.SomeFilter = 1;
--If you truly need “latest for every OrderId”
--You can use a “top-1-per-group” pattern (e.g., TOP (1) WITH TIES … ORDER BY ROW_NUMBER() OVER (PARTITION BY … ORDER BY …)), but it still has to touch essentially all rows to find the latest per group.
