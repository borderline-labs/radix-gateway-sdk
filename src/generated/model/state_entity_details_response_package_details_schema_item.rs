use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct StateEntityDetailsResponsePackageDetailsSchemaItem {
    ///Hex-encoded binary blob.
    pub schema_hash_hex: String,
    ///Hex-encoded binary blob.
    pub schema_hex: String,
}
impl std::fmt::Display for StateEntityDetailsResponsePackageDetailsSchemaItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
