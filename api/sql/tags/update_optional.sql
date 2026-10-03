-- $1: models::TagId
-- $2 pub user_id: shared::models::UserId,
-- $3 pub name: String,
UPDATE "tags"
SET "name" = $3::varchar
WHERE "id" = $1::uuid
  AND "user_id" = $2::uuid
RETURNING "id", "user_id", "name";