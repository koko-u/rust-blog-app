use better_default::Default as BetterDefault;
use into_inner::IntoInner;

use crate::shared;

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
pub struct TagId(#[default(uuid::Uuid::now_v7())] uuid::Uuid);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TagModel {
    pub id: TagId,
    pub user_id: shared::models::UserId,
    pub name: String,
}
