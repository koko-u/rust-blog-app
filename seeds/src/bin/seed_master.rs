use itertools::Itertools;
use seeds::create_pool;
use seeds::data;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let pool = create_pool().await?;
    {
        let mut tx = pool.begin().await?;

        // create categories
        let names = data::CATEGORIES.iter().map(|it| it.to_name()).collect_vec();
        let slugs = data::CATEGORIES.iter().map(|it| it.to_slug()).collect_vec();
        sqlx::query!(
            r#"INSERT INTO "categories" ("user_id", "name", "slug")
               SELECT $1::uuid, "name", "slug"
               FROM
                   unnest($2::varchar[], $3::varchar[]) AS "A"("name", "slug")
               ON CONFLICT ("slug") DO NOTHING"#,
            data::USER_ID,
            &names,
            &slugs
        )
        .execute(tx.as_mut())
        .await?;

        // create tags
        let names = data::TAGS.iter().cloned().collect_vec();
        sqlx::query!(
            r#"INSERT INTO "tags" ("user_id", "name")
               SELECT $1::uuid, "name"
               FROM
                   unnest($2::varchar[]) AS a("name")
               ON CONFLICT ("user_id","name") DO NOTHING"#,
            data::USER_ID,
            &names
        )
        .execute(tx.as_mut())
        .await?;

        tx.commit().await?;
    }

    Ok(())
}
