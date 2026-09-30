INSERT INTO "categories" ("user_id", "name", "slug")
VALUES ($1::uuid, $2::varchar, $3::varchar)
ON CONFLICT ("slug") DO NOTHING
RETURNING "id", "user_id", "name", "slug";