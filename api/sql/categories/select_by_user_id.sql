SELECT "id", "user_id", "name", "slug"
FROM "categories"
WHERE "user_id" = $1::uuid
ORDER BY "id";