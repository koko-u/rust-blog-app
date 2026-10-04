INSERT INTO "post_tags" ("user_id", "post_id", "tag_id")
VALUES ($1::uuid, $2::uuid, $3::uuid)
ON CONFLICT ("post_id", "tag_id") DO NOTHING;