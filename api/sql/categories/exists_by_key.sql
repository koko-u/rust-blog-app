-- $1 id: models::CategoryId
-- $2 user_id: shared::models::UserId
SELECT EXISTS (SELECT 1
               FROM "categories"
               WHERE "id" = $1::uuid
                 AND "user_id" = $2::uuid) AS "exists!"