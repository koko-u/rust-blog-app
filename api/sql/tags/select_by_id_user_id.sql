SELECT "id", "user_id", "name"
FROM "tags"
WHERE "id" = $1::uuid
  AND "user_id" = $2::uuid