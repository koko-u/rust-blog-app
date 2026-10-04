-- $1 user_id: shared::models::UserId
-- $2 category_id: c_models::CategoryId
-- $3 title: String
-- $4 slug: String
-- $5 content: Option<String>


INSERT
INTO "posts" AS "P" ("user_id", "category_id", "title", "slug", "content")
VALUES ($1::uuid/*user_id*/,
        $2::uuid/*category_id*/,
        $3::varchar/*title*/,
        $4::varchar/*slug*/,
        $5::varchar/*content*/)
ON CONFLICT ("slug") DO NOTHING
RETURNING "P"."id",
    "P"."user_id",
    "P"."category_id",
        (SELECT "user_id" FROM "categories" WHERE "id" = $2::uuid) AS "category_user_id!",
        (SELECT "name" FROM "categories" WHERE "id" = $2::uuid) AS "category_name!",
        (SELECT "slug" FROM "categories" WHERE "id" = $2::uuid) AS "category_slug!",
    "P"."title",
    "P"."slug",
    "P"."content",
    CAST(NULL AS uuid) AS "tag_id",
    CAST(NULL AS uuid) AS "tag_user_id",
    CAST(NULL AS varchar) AS "tag_name";



