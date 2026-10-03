INSERT INTO "tags" ("user_id", "name")
VALUES ($1::uuid, $2::varchar)
ON CONFLICT ("user_id", "name") DO NOTHING
RETURNING "id", "user_id", "name";