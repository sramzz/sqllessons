SELECT
    u.DisplayName,
    u.Reputation,
    p.PostTypeId,
    p.Score
FROM dbo.Users AS u
JOIN
(
    SELECT
        p.*,
        n = 
            ROW_NUMBER() OVER
            (
                PARTITION BY
                    p.OwnerUserId
                ORDER BY
                    p.Score DESC
            )
    FROM dbo.Posts AS p
) AS p
    ON  p.OwnerUserId = u.Id
    AND p.n = 1
WHERE u.Reputation > 50000
ORDER BY 
    u.Reputation DESC,
    p.Score DESC;