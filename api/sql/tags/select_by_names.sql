-- $1 names: &[&String]
WITH "params" AS (SELECT DISTINCT "name"
                  FROM unnest($1::varchar[]) as "K"("name"))
SELECT "T"."id",
       "T"."user_id",
       "T"."name"
FROM "tags" AS "T"
         INNER JOIN
     "params" AS "P"
     ON
         "T"."name" = "P"."name";