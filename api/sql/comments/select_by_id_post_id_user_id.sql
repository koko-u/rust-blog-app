-- $1: comment_id
-- $2: post_id
-- $3: user_id
SELECT "id", "post_id", "user_id", "content"
FROM "comments"
WHERE "id" = $1::uuid
  AND "post_id" = $2::uuid
  AND "user_id" = $3::uuid
