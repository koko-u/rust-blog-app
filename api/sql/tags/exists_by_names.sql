-- $1 names: &[&String]
WITH "tag_names" AS (SELECT "name"
                     FROM unnest($1::varchar[]) AS a("name"))
SELECT "N"."name"           AS "name!",
       "T"."id" IS NOT NULL AS "exists!"
FROM "tag_names" AS "N"
         LEFT OUTER JOIN
     "tags" AS "T"
     ON
         "N"."name" = "T"."name"