-- we query the  orders table
-- then use a outer apply (Left Join Lateral) to get the latest status of each order
-- but including also orders that did not get any status at all
-- use CROSS APPLY if you want to remove orders that did not get any status at all
-- Final clarification for my future self:
-- We use the table where you get the orders or you can also use the same status order
-- then you use a second table that will be sorted by status or date descendeing or ascending
-- depending on the problem to resolve. The second table is inside an OUTER APPLY or CROSS APPLY
-- We finally use the top 1,3,5 or any number to obtain the latest status of each order
-- An equivalent could be the top most expensive products per category.
-- 
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
