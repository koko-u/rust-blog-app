use std::collections;

use itertools::Itertools;
use seeds::create_pool;
use seeds::data;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let pool = create_pool().await?;
    {
        let mut tx = pool.begin().await?;

        for post in data::POSTS.iter() {
            let post_id = sqlx::query_scalar!(
                r#"INSERT INTO "posts" AS "P" ("user_id", "category_id", "title", "slug", "content")
                SELECT
                    $1::uuid,
                    "C"."id",
                    $3::varchar,
                    $4::varchar,
                    $5::varchar
                FROM
                    "categories" AS "C"
                WHERE "name" = $2::varchar
                  AND "user_id" = $1::uuid
                ON CONFLICT ("slug") DO NOTHING
                RETURNING "P"."id""#,
                post.user_id,
                post.category_name,
                post.title,
                post.slug,
                post.content.as_ref(),
            )
            .fetch_optional(tx.as_mut())
            .await?;

            if let Some(post_id) = post_id {
                let names = post
                    .tag_names
                    .iter()
                    .cloned()
                    .collect::<collections::HashSet<String>>();
                let names = names.into_iter().collect_vec();

                sqlx::query!(
                    r#"INSERT INTO "post_tags" ("user_id", "post_id", "tag_id")
                        SELECT $1::uuid,
                               $2::uuid,
                               "T"."id"
                        FROM "tags" AS "T"
                                 INNER JOIN
                             unnest($3::varchar[]) AS "P"("name")
                             ON
                                 "T"."name" = "P"."name"
                                 AND
                                 "T"."user_id" = $1::uuid"#,
                    post.user_id,
                    post_id,
                    &names
                )
                .execute(tx.as_mut())
                .await?;
            }
        }

        tx.commit().await?;
    }

    Ok(())
}
