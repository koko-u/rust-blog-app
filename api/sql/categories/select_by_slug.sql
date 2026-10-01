SELECT "id", "user_id", "name", "slug"
FROM "categories"
WHERE "slug" = $1::varchar;