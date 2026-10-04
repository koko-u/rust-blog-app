SELECT "id", "user_id", "name", "slug"
FROM "categories"
WHERE "user_id" = $1::uuid
  AND "name" = $2::varchar;