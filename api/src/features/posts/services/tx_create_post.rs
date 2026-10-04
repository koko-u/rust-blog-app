use crate::errors;
use crate::features::categories::models as c_models;
use crate::features::posts::commands;
use crate::features::posts::models;
use crate::features::posts::repositories;
use crate::shared;

/// ブログ投稿を作成します
///
/// slug を title から生成して適切な内容を insert します
pub async fn tx_create_post(
    tx: &mut shared::Tx<'_>,
    command: &commands::CreatePostCommand,
) -> Result<models::PostModel, errors::ApiError> {
    let commands::CreatePostCommand {
        user_id,
        category_id,
        title,
        content,
        ..
    } = command;

    // try to create post
    let base_slug = rslug::slugify!(command.title.as_str());
    let mut slug = base_slug.clone();

    let mut i = 1;
    let created_row = loop {
        if i >= 100 {
            return Err(errors::ApiError::Slug {
                message: "Failed to create post's slug".to_string(),
                max_count: 100,
            });
        }
        let row =
            repositories::insert_optional(tx, *user_id, *category_id, title, &slug, content.as_ref()).await?;
        if let Some(row) = row {
            break models::PostModel::from(row);
        }

        slug = format!("{base_slug}-{i}");
        i += 1;
    };

    Ok(created_row)
}
