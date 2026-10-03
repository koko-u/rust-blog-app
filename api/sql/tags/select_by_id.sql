SELECT "id", "user_id", "name"
FROM "tags"
WHERE "id" = $1::uuid;