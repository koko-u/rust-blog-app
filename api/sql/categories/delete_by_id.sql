DELETE
FROM "categories"
WHERE "id" = $1::uuid
RETURNING "id", "user_id","name", "slug"