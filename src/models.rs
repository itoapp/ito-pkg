use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct CargoToml {
    pub package: CargoPackage,
}

#[derive(Deserialize, Debug)]
pub struct CargoPackage {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PluginManifest {
    pub id: String,
    pub name: String,
    pub version: String, // String instead of Int to match semver
    pub min_app_version: String,
    pub url: Option<String>,
    #[serde(rename = "sourceUrl")]
    pub source_url: Option<String>,
    #[serde(rename = "contentRating")]
    pub content_rating: Option<i32>,
    pub nsfw: Option<i32>,
    pub language: Option<String>,
    pub languages: Option<Vec<String>>,
    #[serde(rename = "type")]
    pub plugin_type: String, // "manga" or "anime"
    pub author: Option<String>,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoPackage {
    pub id: String,
    pub name: String,
    pub version: String,
    pub min_app_version: String,
    pub download_url: String,
    pub icon_url: Option<String>,
    pub sha256: String,
    #[serde(rename = "type")]
    pub plugin_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoIndex {
    pub repo_name: String,
    pub repo_url: String,
    pub description: String,
    pub packages: Vec<RepoPackage>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_manifest_deserialization() {
        let json = r#"{
            "id": "com.example.plugin",
            "name": "Example Plugin",
            "version": "1.0.0",
            "min_app_version": "1.0.0",
            "type": "manga",
            "author": "Me"
        }"#;

        let manifest: PluginManifest = serde_json::from_str(json).unwrap();
        assert_eq!(manifest.id, "com.example.plugin");
        assert_eq!(manifest.name, "Example Plugin");
        assert_eq!(manifest.plugin_type, "manga");
        assert_eq!(manifest.author, Some("Me".to_string()));
    }
}
