-- $1 id: CommentId
-- $2 post_id: PostId
-- $3 user_id: UserId
-- $4 content: String

UPDATE "comments"
SET "content" = $4::varchar
WHERE "id" = $1::uuid
  AND "post_id" = $2::uuid
  AND "user_id" = $3::uuid
RETURNING "id", "post_id", "user_id", "content"