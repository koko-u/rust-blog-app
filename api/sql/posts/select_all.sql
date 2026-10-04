SELECT "P"."id",
       "P"."user_id",
       "P"."category_id",
       "C"."user_id" AS "category_user_id",
       "C"."name"    AS "category_name",
       "C"."slug"    AS "category_slug",
       "P"."title",
       "P"."slug",
       "P"."content",
       "T"."id"      AS "tag_id",
       "T"."user_id" AS "tag_user_id",
       "T"."name"    AS "tag_name"
FROM "posts" AS "P"
         INNER JOIN
     "categories" AS "C"
     ON
         "P"."category_id" = "C"."id"
         LEFT OUTER JOIN
     "post_tags" AS "PT"
     ON
         "P"."id" = "PT"."post_id"
         LEFT OUTER JOIN
     "tags" AS "T"
     ON
         "PT"."tag_id" = "T"."id"
ORDER BY "P"."id";