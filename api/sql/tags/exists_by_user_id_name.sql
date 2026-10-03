-- $1 user_id: shared::models::UserId
-- $2 name: string
SELECT EXISTS (SELECT 1
               FROM "tags"
               WHERE "user_id" = $1::uuid
                 AND "name" = $2::varchar) AS "exists!"