use crate::document::{Document, DocumentSpec, Tag};
use console::{style, Style};
use kube::api::{Patch, PatchParams};
use kube::{Api, Client, ResourceExt};
use serde_yaml;
use similar::{ChangeTag, TextDiff};

mod document;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::try_default().await?;
    let documents: Api<Document> = Api::default_namespaced(client);
    let before = documents.get("example").await?;

    // manager-1としてApplyするケース
    let new_document = Document::new(
        "example",
        DocumentSpec {
            title: "Example document".to_string().into(),
            tags: vec![
                // key1は無いことに注目。
                Tag {
                    key: "key2".into(),
                    value: "new-value2!".into(),
                },
            ],
        },
    );
    let after = documents
        .patch(
            &new_document.name_unchecked(),
            &PatchParams {
                dry_run: true, // diffだけみたいのので、dry_runをtrueにすることで、実際にはApplyしない
                force: false,
                field_manager: Some("manager-1".into()),
                field_validation: None,
            },
            &Patch::Apply(new_document),
        )
        .await?;
    title("manager-1でApplyした場合のdiff:");
    print_document_diff(&before, &after);

    // manager-2としてApplyするケース
    let new_document = Document::new(
        "example",
        DocumentSpec {
            title: "Example document".to_string().into(),
            tags: vec![
                // key3は無いことに注目。
                Tag {
                    key: "key4".into(),
                    value: "new-value4!".into(),
                },
            ],
        },
    );
    let after = documents
        .patch(
            &new_document.name_unchecked(),
            &PatchParams {
                dry_run: true, // diffだけみたいのので、dry_runをtrueにすることで、実際にはApplyしない
                force: false,
                field_manager: Some("manager-2".into()),
                field_validation: None,
            },
            &Patch::Apply(new_document),
        )
        .await?;
    title("manager-2でApplyした場合のdiff:");
    print_document_diff(&before, &after);

    Ok(())
}

fn print_diff(old: &str, new: &str) {
    let diff = TextDiff::from_lines(old, new);
    for op in diff.ops() {
        for change in diff.iter_changes(op) {
            let (sign, style) = match change.tag() {
                ChangeTag::Delete => ("-", Style::new().red()),
                ChangeTag::Insert => ("+", Style::new().green()),
                ChangeTag::Equal => (" ", Style::new()),
            };
            print!("{}{}", style.apply_to(sign).bold(), style.apply_to(change));
        }
    }
}

fn print_document_diff(before: &Document, after: &Document) {
    let before_yaml = serde_yaml::to_string(&before.spec).unwrap();
    let after_yaml = serde_yaml::to_string(&after.spec).unwrap();
    print_diff(&before_yaml, &after_yaml);
}

fn title(title: &str) {
    println!("\n{}", style(title).bold().blue().on_white());
}
