use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize,FromRow)]
pub(crate) struct CodeDetails{
    pub(crate) id: i32,
    pub(crate) code: String,
    pub(crate) description: String,
    pub(crate) longdescription: String,

}

#[derive(Debug, Serialize, Deserialize,FromRow)]
pub(crate) struct AlterTerm{
    pub(crate) id: i32,
    pub(crate) code: String,
    pub(crate) alterdescription: String,
    pub(crate) longdescription: String,

}