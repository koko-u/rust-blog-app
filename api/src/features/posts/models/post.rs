use better_default::Default as BetterDefault;
use into_inner::IntoInner;

use crate::features::categories::models as c_models;
use crate::features::tags::models as t_models;
use crate::features::users::models as u_models;

#[derive(
    Debug,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Hash,
    BetterDefault,
    derive_more::Display,
    derive_more::From,
    derive_more::FromStr,
    into_inner::IntoInner,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[display("{}", _0)]
#[serde(transparent)]
#[schema(value_type = uuid::Uuid)]
pub struct PostId(#[default(uuid::Uuid::now_v7())] uuid::Uuid);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PostModel {
    pub id: PostId,
    pub user_id: u_models::UserId,
    pub category: c_models::CategoryModel,
    pub title: String,
    pub slug: String,
    pub content: Option<String>,
    pub tags: Vec<t_models::TagModel>,
}
