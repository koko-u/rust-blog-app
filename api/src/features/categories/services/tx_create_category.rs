use crate::errors;
use crate::features::categories::commands;
use crate::features::categories::models;
use crate::features::categories::repositories;
use crate::shared;

/// カテゴリを作成します
///
/// slug を name から生成して適切な内容を insert します
pub async fn tx_create_category(
    command: &commands::CreateCategoryCommand,
    tx: &mut shared::Tx<'_>,
) -> Result<models::CategoryModel, errors::ApiError> {
    let commands::CreateCategoryCommand { user_id, name } = command;

    // try to create category
    let base_slug = rslug::slugify!(command.name.as_str());
    let mut slug = base_slug.clone();

    let mut i = 1;
    let created_row = loop {
        if i >= 100 {
            return Err(errors::ApiError::Slug {
                message: "Failed to create category slug".to_string(),
                max_count: 100,
            });
        }
        let row = repositories::insert_optional(tx, *user_id, name, &slug).await?;
        if let Some(row) = row {
            break models::CategoryModel::from(row);
        }

        slug = format!("{base_slug}-{i}");
        i += 1;
    };

    Ok(created_row)
}
