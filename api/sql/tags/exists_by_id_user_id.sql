-- $1 id: models::TagId
-- $2 user_id: shared::models::UserId
SELECT EXISTS (SELECT 1
               FROM "tags"
               WHERE "id" = $1::uuid
                 AND "user_id" = $2::uuid) AS "exists!"