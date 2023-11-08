use crate::document::Document;
use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition;
use kube::api::{Patch, PatchParams};
use kube::{Api, Client, CustomResourceExt};

mod document;

#[tokio::main]
async fn main() {
    println!("{}", serde_yaml::to_string(&Document::crd()).unwrap());
    apply_crd().await.unwrap();
}

async fn apply_crd() -> Result<(), Box<dyn std::error::Error>> {
    println!("Applying CRD...");
    let client = Client::try_default().await?;
    let crds: Api<CustomResourceDefinition> = Api::all(client.clone());
    crds.patch(
        Document::crd_name(),
        &PatchParams::apply("kubectl-client-side-apply"),
        &Patch::Apply(Document::crd()),
    )
    .await?;
    Ok(())
}
