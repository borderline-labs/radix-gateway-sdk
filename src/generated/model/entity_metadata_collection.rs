use serde::{Serialize, Deserialize};
use super::{EntityMetadataItem, ResultSetCursorMixin};
///Entity metadata collection.
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityMetadataCollection {
    #[serde(flatten)]
    pub result_set_cursor_mixin: ResultSetCursorMixin,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<EntityMetadataItem>,
}
impl std::fmt::Display for EntityMetadataCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for EntityMetadataCollection {
    type Target = ResultSetCursorMixin;
    fn deref(&self) -> &Self::Target {
        &self.result_set_cursor_mixin
    }
}
impl std::ops::DerefMut for EntityMetadataCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.result_set_cursor_mixin
    }
}
