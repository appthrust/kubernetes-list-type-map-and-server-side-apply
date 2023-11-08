use crate::document::{Document, DocumentSpec, Tag};
use kube::api::{Patch, PatchParams};
use kube::{Api, Client, ResourceExt};

mod document;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::try_default().await?;
    let documents: Api<Document> = Api::default_namespaced(client);

    // manager-1としてApplyするケース
    let document = Document::new(
        "example",
        DocumentSpec {
            title: "Example document".to_string().into(),
            tags: vec![
                Tag {
                    key: "key1".into(),
                    value: "value1".into(),
                },
                Tag {
                    key: "key2".into(),
                    value: "value2".into(),
                },
            ],
        },
    );
    documents
        .patch(
            &document.name_unchecked(),
            &PatchParams {
                dry_run: false,
                force: false,
                field_manager: Some("manager-1".into()),
                field_validation: None,
            },
            &Patch::Apply(document),
        )
        .await?;

    // manager-2としてApplyするケース
    let document = Document::new(
        "example",
        DocumentSpec {
            title: "Example document".to_string().into(),
            tags: vec![
                Tag {
                    key: "key3".into(),
                    value: "value3".into(),
                },
                Tag {
                    key: "key4".into(),
                    value: "value4".into(),
                },
            ],
        },
    );
    documents
        .patch(
            &document.name_unchecked(),
            &PatchParams {
                dry_run: false,
                force: false,
                field_manager: Some("manager-2".into()),
                field_validation: None,
            },
            &Patch::Apply(document),
        )
        .await?;

    Ok(())
}
