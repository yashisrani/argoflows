use serde::{Deserialize, Serialize};

/// ResourceTemplate is a template subtype to manipulate kubernetes resources
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceTemplate {
    /// Action is the action to perform to the resource. Must be one of: get,
    /// create, apply, delete, replace, patch.
    #[serde(rename = "action")]
    pub action: String,

    /// FailureCondition is a label selector expression which describes the 
    /// conditions of the k8s resource in which the step was considered failed.
    #[serde(rename = "failureCondition", skip_serializing_if = "Option::is_none")]
    pub failure_condition: Option<String>,

    /// Flags is a set of additional options passed to kubectl before submitting
    /// a resource I.e. to disable resource validation: flags: 
    /// [  \"--validate=false\"  # disable resource validation ].
    #[serde(rename = "flags", skip_serializing_if = "Option::is_none")]
    pub flags: Option<Vec<String>>,

    /// Manifest contains the kubernetes manifest.
    #[serde(rename = "manifest", skip_serializing_if = "Option::is_none")]
    pub manifest: Option<String>,

    #[serde(rename = "manifestFrom", skip_serializing_if = "Option::is_none")]
    pub manifest_from: Option<Box<super::ManifestFrom>>,
    
    /// MergeStrategy is the strategy used to merge a patch. It defaults to
    /// \"strategic\" Must be one of: strategic, merge, json
    #[serde(rename = "mergeStrategy", skip_serializing_if = "Option::is_none")]
    pub merge_strategy: Option<String>,

    /// SetOwnerReference sets the reference to the workflow on the
    /// OwnerReference of generated resource.
    #[serde(rename = "setOwnerReference", skip_serializing_if = "Option::is_none")]
    pub set_owner_reference: Option<bool>,

    /// SuccessCondition is a label selector expression which describes the 
    /// conditions of the k8s resource in which it is acceptable to proceed
    /// to the following step.
    #[serde(rename = "successCondition", skip_serializing_if = "Option::is_none")]
    pub success_condition: Option<String>,
}

impl ResourceTemplate {
    pub fn new(action: &str) -> ResourceTemplate {
        ResourceTemplate {
            action: action.to_string(),
            failure_condition: None,
            flags: None,
            manifest: None,
            manifest_from: None,
            merge_strategy: None,
            set_owner_reference: None,
            success_condition: None,
        }
    }
}
