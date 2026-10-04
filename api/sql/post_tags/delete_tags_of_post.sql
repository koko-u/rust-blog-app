DELETE
FROM "post_tags"
WHERE "user_id" = $1::uuid
  AND "post_id" = $2::uuid