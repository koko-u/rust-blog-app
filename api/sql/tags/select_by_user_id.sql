SELECT "id", "user_id", "name"
FROM "tags"
WHERE "user_id" = $1::uuid
