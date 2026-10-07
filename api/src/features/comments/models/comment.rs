use better_default::Default as BetterDefault;
use into_inner::IntoInner;

use crate::features::posts::models as p_models;
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
pub struct CommentId(#[default(uuid::Uuid::now_v7())] uuid::Uuid);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CommentModel {
    pub id: CommentId,
    pub post_id: p_models::PostId,
    pub user_id: u_models::UserId,
    pub content: String,
}
