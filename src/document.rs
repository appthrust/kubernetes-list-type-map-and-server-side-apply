use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(CustomResource, Debug, Serialize, Deserialize, Default, Clone, JsonSchema)]
#[kube(group = "suin.jp", version = "v1", kind = "Document", namespaced)]
pub struct DocumentSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[schemars(schema_with = "tags")]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, JsonSchema)]
pub struct Tag {
    pub key: String,
    pub value: String,
}

fn tags(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
    let mut schema = gen.subschema_for::<Vec<Tag>>().into_object().clone();
    schema.extensions = [
        ("x-kubernetes-list-type".into(), json!("map")),
        ("x-kubernetes-list-map-keys".into(), json!(["key"])),
    ]
    .into();
    schema.into()
}
