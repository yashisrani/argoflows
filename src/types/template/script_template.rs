use k8s_openapi::api::core;
use serde::{Deserialize, Serialize};

/// ScriptTemplate is a template subtype to enable scripting through code steps.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScriptTemplate {
    /// Arguments to the entrypoint. The container image's CMD is used if this
    /// is not provided. Variable references $(VAR_NAME) are expanded using the
    /// container's environment. If a variable cannot be resolved, the
    /// reference in the input string will be unchanged. Double $$ are reduced
    /// to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e.
    /// \"$$(VAR_NAME)\" will produce the string literal \"$(VAR_NAME)\".
    /// Escaped references will never be expanded, regardless of whether the
    /// variable exists or not. Cannot be updated. More info:
    /// https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
    #[serde(rename = "args", skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,

    /// Entrypoint array. Not executed within a shell. The container image's
    /// ENTRYPOINT is used if this is not provided. Variable references
    /// $(VAR_NAME) are expanded using the container's environment. If a
    /// variable cannot be resolved, the reference in the input string will be
    /// unchanged. Double $$ are reduced to a single $, which allows for
    /// escaping the $(VAR_NAME) syntax: i.e. \"$$(VAR_NAME)\" will produce the
    /// string literal \"$(VAR_NAME)\". Escaped references will never be
    /// expanded, regardless of whether the variable exists or not. Cannot be
    /// updated. More info:
    /// https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
    #[serde(rename = "command", skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,

    /// List of environment variables to set in the container. Cannot be updated.
    #[serde(rename = "env", skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<core::v1::EnvVar>>,

    /// List of sources to populate environment variables in the container. The
    /// keys defined within a source must be a C_IDENTIFIER. All invalid keys
    /// will be reported as an event when the container is starting. When a key
    /// exists in multiple sources, the value associated with the last source
    /// will take precedence. Values defined by an Env with a duplicate key
    /// will take precedence. Cannot be updated.
    #[serde(rename = "envFrom", skip_serializing_if = "Option::is_none")]
    pub env_from: Option<Vec<core::v1::EnvFromSource>>,

    /// Container image name. More info:
    /// https://kubernetes.io/docs/concepts/containers/images.
    /// This field is optional to allow higher level config management to
    /// default or override container images in workload controllers like
    /// Deployments and StatefulSets.
    #[serde(rename = "image")]
    pub image: String,

    /// Image pull policy. One of Always, Never, IfNotPresent. Defaults to
    /// Always if :latest tag is specified, or IfNotPresent otherwise.
    /// Cannot be updated. More info:
    /// https://kubernetes.io/docs/concepts/containers/images#updating-images
    #[serde(rename = "imagePullPolicy", skip_serializing_if = "Option::is_none")]
    pub image_pull_policy: Option<String>,

    #[serde(rename = "lifecycle", skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<Box<core::v1::Lifecycle>>,

    #[serde(rename = "livenessProbe", skip_serializing_if = "Option::is_none")]
    pub liveness_probe: Option<Box<core::v1::Probe>>,

    /// Name of the container specified as a DNS_LABEL. Each container in a pod
    /// must have a unique name (DNS_LABEL). Cannot be updated.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// List of ports to expose from the container. Exposing a port here gives the system additional information about the network connections a container uses, but is primarily informational. Not specifying a port here DOES NOT prevent that port from being exposed. Any port which is listening on the default \"0.0.0.0\" address inside a container will be accessible from the network. Cannot be updated.
    #[serde(rename = "ports", skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<core::v1::ContainerPort>>,

    #[serde(rename = "readinessProbe", skip_serializing_if = "Option::is_none")]
    pub readiness_probe: Option<Box<core::v1::Probe>>,

    #[serde(rename = "resources", skip_serializing_if = "Option::is_none")]
    pub resources: Option<Box<core::v1::ResourceRequirements>>,

    #[serde(rename = "securityContext", skip_serializing_if = "Option::is_none")]
    pub security_context: Option<Box<core::v1::SecurityContext>>,

    /// Source contains the source code of the script to execute.
    #[serde(rename = "source")]
    pub source: String,

    #[serde(rename = "startupProbe", skip_serializing_if = "Option::is_none")]
    pub startup_probe: Option<Box<core::v1::Probe>>,

    /// Whether this container should allocate a buffer for stdin in the
    /// container runtime. If this is not set, reads from stdin in the container
    /// will always result in EOF. Default is false.
    #[serde(rename = "stdin", skip_serializing_if = "Option::is_none")]
    pub stdin: Option<bool>,

    /// Whether the container runtime should close the stdin channel after it
    /// has been opened by a single attach. When stdin is true the stdin stream
    /// will remain open across multiple attach sessions. If stdinOnce is set
    /// to true, stdin is opened on container start, is empty until the first
    /// client attaches to stdin, and then remains open and accepts data until
    /// the client disconnects, at which time stdin is closed and remains closed
    /// until the container is restarted. If this flag is false, a container
    /// processes that reads from stdin will never receive an EOF. Default is false.
    #[serde(rename = "stdinOnce", skip_serializing_if = "Option::is_none")]
    pub stdin_once: Option<bool>,

    /// Optional: Path at which the file to which the container's termination
    /// message will be written is mounted into the container's filesystem.
    /// Message written is intended to be brief final status, such as an
    /// assertion failure message. Will be truncated by the node if greater than
    /// 4096 bytes. The total message length across all containers will be
    /// limited to 12kb. Defaults to /dev/termination-log. Cannot be updated.
    #[serde(
        rename = "terminationMessagePath",
        skip_serializing_if = "Option::is_none"
    )]
    pub termination_message_path: Option<String>,

    /// Indicate how the termination message should be populated. File will use
    /// the contents of terminationMessagePath to populate the container status
    /// message on both success and failure. FallbackToLogsOnError will use the
    /// last chunk of container log output if the termination message file is
    /// empty and the container exited with an error. The log output is limited
    /// to 2048 bytes or 80 lines, whichever is smaller.
    /// Defaults to File. Cannot be updated.
    #[serde(
        rename = "terminationMessagePolicy",
        skip_serializing_if = "Option::is_none"
    )]
    pub termination_message_policy: Option<String>,

    /// Whether this container should allocate a TTY for itself, also requires 'stdin' to be true. Default is false.
    #[serde(rename = "tty", skip_serializing_if = "Option::is_none")]
    pub tty: Option<bool>,

    /// volumeDevices is the list of block devices to be used by the container.
    #[serde(rename = "volumeDevices", skip_serializing_if = "Option::is_none")]
    pub volume_devices: Option<Vec<core::v1::VolumeDevice>>,

    /// Pod volumes to mount into the container's filesystem. Cannot be updated.
    #[serde(rename = "volumeMounts", skip_serializing_if = "Option::is_none")]
    pub volume_mounts: Option<Vec<core::v1::VolumeMount>>,

    /// Container's working directory. If not specified, the container runtime's
    /// default will be used, which might be configured in the container image.
    /// Cannot be updated.
    #[serde(rename = "workingDir", skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<String>,
}

impl ScriptTemplate {
    pub fn new(image: &str, source: &str) -> ScriptTemplate {
        ScriptTemplate {
            args: None,
            command: None,
            env: None,
            env_from: None,
            image: image.to_string(),
            image_pull_policy: None,
            lifecycle: None,
            liveness_probe: None,
            name: None,
            ports: None,
            readiness_probe: None,
            resources: None,
            security_context: None,
            source: source.to_string(),
            startup_probe: None,
            stdin: None,
            stdin_once: None,
            termination_message_path: None,
            termination_message_policy: None,
            tty: None,
            volume_devices: None,
            volume_mounts: None,
            working_dir: None,
        }
    }
}
