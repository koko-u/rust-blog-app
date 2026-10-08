-- $1: post_id
-- $2: user_id
SELECT "id", "post_id", "user_id", "content"
FROM "comments"
WHERE "post_id" = $1::uuid
  AND "user_id" = $2::uuid
