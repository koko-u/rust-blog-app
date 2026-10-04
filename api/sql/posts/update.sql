UPDATE "posts" AS "P"
SET "category_id" = $3::uuid,
    "title"       = $4::varchar,
    "slug"        = $5::varchar,
    "content"     = $6::varchar
WHERE "id" = $1::uuid
  AND "user_id" = $2::uuid
