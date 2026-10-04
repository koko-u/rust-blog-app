-- $1 id: models::PostId
-- $2 user_id: shared::models::UserId
SELECT EXISTS (SELECT 1
               FROM "posts"
               WHERE "id" = $1::uuid
                 AND "user_id" = $2::uuid) AS "exists!"