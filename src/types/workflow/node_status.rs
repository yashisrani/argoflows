use serde::{Deserialize, Serialize};

use crate::types::template;

/// `NodeStatus` contains status information about an individual node
/// in the workflow.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct NodeStatus {
    /// `BoundaryID` indicates the node ID of the associated template root node
    /// in which this node belongs to.
    #[serde(rename = "boundaryID", skip_serializing_if = "Option::is_none")]
    pub boundary_id: Option<String>,

    /// `Children` is a list of child node IDs.
    #[serde(rename = "children", skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<String>>,

    /// `Daemoned` tracks whether or not this node was daemoned and need
    /// to be terminated.
    #[serde(rename = "daemoned", skip_serializing_if = "Option::is_none")]
    pub daemoned: Option<bool>,

    /// `DisplayName` is a human readable representation of the node.
    /// Unique within a template boundary.
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// `EstimatedDuration` in seconds.
    #[serde(rename = "estimatedDuration", skip_serializing_if = "Option::is_none")]
    pub estimated_duration: Option<i32>,

    /// Time is a wrapper around time.Time which supports correct marshaling
    /// to YAML and JSON.  Wrappers are provided for many of the factory
    /// methods that the time package offers.
    #[serde(rename = "finishedAt", skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<String>,

    /// `HostNodeName` name of the Kubernetes node on which the Pod is running,
    /// if applicable.
    #[serde(rename = "hostNodeName", skip_serializing_if = "Option::is_none")]
    pub host_node_name: Option<String>,

    /// `ID` is a unique identifier of a node within the workflow. It is
    /// implemented as a hash of the node name, which makes the
    /// ID deterministic.
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "inputs", skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Box<template::Inputs>>,

    #[serde(rename = "memoizationStatus", skip_serializing_if = "Option::is_none")]
    pub memoization_status: Option<Box<template::MemoizationStatus>>,

    /// A human readable message indicating details about why the node is
    /// in this condition.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// `Name` is unique name in the node tree used to generate the node ID.
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "nodeFlag", skip_serializing_if = "Option::is_none")]
    pub node_flag: Option<Box<super::NodeFlag>>,

    /// `OutboundNodes` tracks the node IDs which are considered \"outbound\"
    /// nodes to a template invocation. For every invocation of a template,
    /// there are nodes which we considered as \"outbound\". Essentially,
    /// these are last nodes in the execution sequence to run, before the
    /// template is considered completed. These nodes are then connected as
    /// parents to a following step.  In the case of single pod steps
    /// (i.e. container, script, resource templates), this list will be nil
    /// since the pod itself is already considered the \"outbound\" node.
    /// In the case of DAGs, outbound nodes are the \"target\" tasks
    /// (tasks with no children). In the case of steps, outbound nodes are all
    /// the containers involved in the last step group. NOTE: since templates
    /// are composable, the list of outbound nodes are carried upwards when a
    /// DAG/steps template invokes another DAG/steps template. In other words,
    /// the outbound nodes of a template, will be a superset of the outbound
    /// nodes of its last children.
    #[serde(rename = "outboundNodes", skip_serializing_if = "Option::is_none")]
    pub outbound_nodes: Option<Vec<String>>,

    #[serde(rename = "outputs", skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Box<template::Outputs>>,

    /// `Phase` a simple, high-level summary of where the node is in its lifecycle.
    /// Can be used as a state machine. Will be one of these values \"Pending\",
    /// \"Running\" before the node is completed, or \"Succeeded\", \"Skipped\",
    /// \"Failed\", \"Error\", or \"Omitted\" as a final state.
    #[serde(rename = "phase", skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,

    /// `PodIP` captures the IP of the pod for daemoned steps
    #[serde(rename = "podIP", skip_serializing_if = "Option::is_none")]
    pub pod_ip: Option<String>,

    /// `Progress` to completion.
    #[serde(rename = "progress", skip_serializing_if = "Option::is_none")]
    pub progress: Option<String>,

    /// `ResourcesDuration` is indicative, but not accurate, resource duration.
    /// This is populated when the nodes completes.
    #[serde(rename = "resourcesDuration", skip_serializing_if = "Option::is_none")]
    pub resources_duration: Option<::std::collections::HashMap<String, i64>>,

    /// Time is a wrapper around time.Time which supports correct marshaling
    /// to YAML and JSON.  Wrappers are provided for many of the factory
    /// methods that the time package offers.
    #[serde(rename = "startedAt", skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,

    #[serde(
        rename = "synchronizationStatus",
        skip_serializing_if = "Option::is_none"
    )]
    pub synchronization_status: Option<Box<super::NodeSynchronizationStatus>>,

    /// `TemplateName` is the template name which this node corresponds to. Not
    /// applicable to virtual nodes (e.g. Retry, StepGroup).
    #[serde(rename = "templateName", skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,

    #[serde(rename = "templateRef", skip_serializing_if = "Option::is_none")]
    pub template_ref: Option<Box<super::TemplateRef>>,

    /// `TemplateScope` is the template scope in which the template of this
    /// node was retrieved.
    #[serde(rename = "templateScope", skip_serializing_if = "Option::is_none")]
    pub template_scope: Option<String>,

    /// `Type` indicates type of node.
    #[serde(rename = "type")]
    pub r#type: String,
}

impl NodeStatus {
    pub fn new(id: &str, name: &str, r#type: &str) -> Self {
        NodeStatus {
            id: id.to_string(),
            name: name.to_string(),
            r#type: r#type.to_string(),
            ..Default::default()
        }
    }
}
