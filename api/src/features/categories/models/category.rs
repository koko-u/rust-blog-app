use better_default::Default as BetterDefault;
use into_inner::IntoInner;

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
pub struct CategoryId(#[default(uuid::Uuid::now_v7())] uuid::Uuid);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct CategoryModel {
    pub id: CategoryId,
    pub user_id: u_models::UserId,
    pub name: String,
    pub slug: String,
}
