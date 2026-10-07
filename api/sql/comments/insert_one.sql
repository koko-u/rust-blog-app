-- $1: post_id
-- $2: user_id
-- $3: content
INSERT INTO "comments" ("post_id", "user_id", "content")
VALUES ($1::uuid, $2::uuid, $3::varchar)
RETURNING "id","post_id","user_id","content"