SELECT "id", "user_id", "name", "slug"
FROM "categories"
WHERE "id" = $1::uuid;