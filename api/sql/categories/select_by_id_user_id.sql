SELECT "id", "user_id", "name", "slug"
FROM "categories"
WHERE "id" = $1::uuid
  AND "user_id" = $2::uuid;