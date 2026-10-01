-- $1 id: models::CategoryId,
-- $2 pub user_id: shared::models::UserId,
-- $3 pub name: String,
-- $4 pub slug: String,
UPDATE "categories"
SET "name" = $3::varchar,
    "slug" = $4::varchar
WHERE "id" = $1::uuid
  AND "user_id" = $2::uuid
RETURNING "id", "user_id", "name", "slug"