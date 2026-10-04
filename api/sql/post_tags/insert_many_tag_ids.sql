WITH "tag_ids" AS (SELECT "tag_id"
                   FROM unnest($3::uuid[]) AS a("tag_id"))
INSERT
INTO "post_tags" ("user_id", "post_id", "tag_id")
SELECT $1::uuid, $2::uuid, "tag_id"
FROM "tag_ids"
ON CONFLICT ("post_id", "tag_id") DO NOTHING;