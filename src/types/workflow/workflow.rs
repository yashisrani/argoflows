use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use serde::{Deserialize, Serialize};

/// `Workflow` is the definition of a workflow resource.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Workflow {
    /// `APIVersion` defines the versioned schema of this representation of an
    /// object. Servers should convert recognized schemas to the latest
    /// internal value, and may reject unrecognized values. More info:
    /// https://git.io.k8s.community/contributors/devel/sig-architecture/api-conventions.md#resources
    #[serde(rename = "apiVersion", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,

    /// `Kind` is a string value representing the REST resource this object
    /// represents. Servers may infer this from the endpoint the client submits
    ///  requests to. Cannot be updated. In CamelCase. More info:
    /// https://git.io.k8s.community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    #[serde(rename = "metadata")]
    pub metadata: Box<ObjectMeta>,

    #[serde(rename = "spec")]
    pub spec: Box<super::WorkflowSpec>,

    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Box<super::WorkflowStatus>>,
}

impl Workflow {
    pub fn new(metadata: ObjectMeta, spec: super::WorkflowSpec) -> Self {
        Workflow {
            metadata: Box::new(metadata),
            spec: Box::new(spec),
            ..Default::default()
        }
    }
}
