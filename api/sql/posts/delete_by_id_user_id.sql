DELETE
FROM "posts"
WHERE "id" = $1::uuid
  AND "user_id" = $2::uuid