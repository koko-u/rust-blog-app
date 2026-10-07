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
        let user_ids = data::CATEGORIES.iter().map(|it| it.to_user_id()).collect_vec();
        let names = data::CATEGORIES.iter().map(|it| it.to_name()).collect_vec();
        let slugs = data::CATEGORIES.iter().map(|it| it.to_slug()).collect_vec();
        sqlx::query!(
            r#"INSERT INTO "categories" ("user_id", "name", "slug")
               SELECT "user_id", "name", "slug"
               FROM
                   unnest($1::uuid[], $2::varchar[], $3::varchar[]) AS "A"("user_id", "name", "slug")
               ON CONFLICT ("slug") DO NOTHING"#,
            &user_ids,
            &names,
            &slugs
        )
        .execute(tx.as_mut())
        .await?;

        // create tags
        let user_ids = data::TAGS.iter().map(|it| it.to_user_id()).collect_vec();
        let names = data::TAGS.iter().map(|it| it.to_name()).collect_vec();
        sqlx::query!(
            r#"INSERT INTO "tags" ("user_id", "name")
               SELECT "user_id", "name"
               FROM
                   unnest($1::uuid[], $2::varchar[]) AS a("user_id","name")
               ON CONFLICT ("user_id","name") DO NOTHING"#,
            &user_ids,
            &names
        )
        .execute(tx.as_mut())
        .await?;

        tx.commit().await?;
    }

    Ok(())
}
