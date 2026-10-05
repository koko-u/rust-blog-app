DELETE
FROM "tags"
WHERE "id" = $1::uuid
  AND "user_id" = $2::uuid
RETURNING "id", "user_id", "name";