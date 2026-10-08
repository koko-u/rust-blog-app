-- $1 id: models::TagId
-- $2 comment_id
-- $3 user_id: shared::models::UserId
SELECT EXISTS (SELECT 1
               FROM "comments"
               WHERE "id" = $1::uuid
                 AND "post_id" = $2::uuid
                 AND "user_id" = $3::uuid) AS "exists!"