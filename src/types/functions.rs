//! Types for the Azure Functions API (v1).
//!
//! Auto-generated from the Azure ARM REST Specification.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for an Azure Function App.
///
/// **Azure API**: `functions.v1.FunctionAppSiteConfig`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/appservice//FunctionAppSiteConfig>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionAppSiteConfig {
    /// Number of workers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_workers: Option<i32>,

    /// .NET Framework version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_framework_version: Option<String>,

    /// Linux app framework and version (e.g. PYTHON|3.11)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linux_fx_version: Option<String>,

    /// App command line to launch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_command_line: Option<String>,
}

impl FunctionAppSiteConfig {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            number_of_workers: Some(100),
            net_framework_version: Some("test-net_framework_version".into()),
            linux_fx_version: Some("test-linux_fx_version".into()),
            app_command_line: Some("test-app_command_line".into()),
        }
    }
}

/// Properties of a Function App site resource.
///
/// **Azure API**: `functions.v1.FunctionAppProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/appservice//FunctionAppProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionAppProperties {
    /// Current state of the app (Running, Stopped, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// Hostnames associated with the app
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub host_names: Vec<String>,

    /// Default hostname of the app
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_host_name: Option<String>,

    /// Kind of resource (functionapp, linux)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// Resource group the app belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,

    /// Resource ID of the hosting plan
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_farm_id: Option<String>,

    /// Configuration of the app
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_config: Option<FunctionAppSiteConfig>,
}

impl FunctionAppProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            state: Some("test-state".into()),
            host_names: vec![],
            default_host_name: Some("test-default_host_name".into()),
            kind: Some("test-kind".into()),
            resource_group: Some("test-resource_group".into()),
            server_farm_id: Some("test-server_farm_id".into()),
            site_config: Some(FunctionAppSiteConfig::fixture()),
        }
    }
}

/// A Function App resource (Azure Web Sites Site).
///
/// **Azure API**: `functions.v1.FunctionApp`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/appservice//FunctionApp>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionApp {
    /// Fully qualified resource ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The name of the resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The type of the resource
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// Resource location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    /// Resource tags
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,

    /// Kind of resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// Function App properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<FunctionAppProperties>,
}

impl FunctionApp {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            name: Some("test-function_app".into()),
            r#type: Some("test-type".into()),
            location: Some("test-location".into()),
            tags: Default::default(),
            kind: Some("test-kind".into()),
            properties: Some(FunctionAppProperties::fixture()),
        }
    }
}

/// Result of a list Function Apps operation.
///
/// **Azure API**: `functions.v1.FunctionAppListResult`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/appservice//FunctionAppListResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionAppListResult {
    /// List of Function Apps
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<FunctionApp>,

    /// Link to next page of results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl FunctionAppListResult {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            value: vec![],
            next_link: Some("test-next_link".into()),
        }
    }
}

/// Properties for creating or updating a Function App.
///
/// **Azure API**: `functions.v1.FunctionAppCreateOrUpdateProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/appservice//FunctionAppCreateOrUpdateProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionAppCreateOrUpdateProperties {
    /// Resource ID of the App Service Plan (hosting plan)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_farm_id: Option<String>,

    /// Configuration of the app
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_config: Option<FunctionAppSiteConfig>,
}

impl FunctionAppCreateOrUpdateProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            server_farm_id: Some("test-server_farm_id".into()),
            site_config: Some(FunctionAppSiteConfig::fixture()),
        }
    }
}

/// Request body for creating or updating a Function App.
///
/// **Azure API**: `functions.v1.FunctionAppCreateRequest`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/appservice//FunctionAppCreateRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionAppCreateRequest {
    /// Resource location
    pub location: String,

    /// Resource tags
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,

    /// Kind of app (functionapp, functionapp,linux)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// Function App create/update properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<FunctionAppCreateOrUpdateProperties>,
}

impl FunctionAppCreateRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            location: "test-location".into(),
            tags: Default::default(),
            kind: Some("test-kind".into()),
            properties: Some(FunctionAppCreateOrUpdateProperties::fixture()),
        }
    }
}

/// Properties of an individual Azure Function.
///
/// **Azure API**: `functions.v1.FunctionProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/appservice//FunctionProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionProperties {
    /// Name of the function
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Function App ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_app_id: Option<String>,

    /// Script root path href
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_root_path_href: Option<String>,

    /// Script href
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_href: Option<String>,

    /// Config href
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_href: Option<String>,

    /// Whether the function is disabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_disabled: Option<bool>,
}

impl FunctionProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: Some("test-function_properties".into()),
            function_app_id: Some("test-function_app_id".into()),
            script_root_path_href: Some("test-script_root_path_href".into()),
            script_href: Some("test-script_href".into()),
            config_href: Some("test-config_href".into()),
            is_disabled: Some(false),
        }
    }
}

/// An individual Azure Function within a Function App.
///
/// **Azure API**: `functions.v1.Function`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/appservice//Function>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Function {
    /// Fully qualified resource ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The name of the function
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The type of the resource
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// Function properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<FunctionProperties>,
}

impl Function {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            name: Some("test-function".into()),
            r#type: Some("test-type".into()),
            properties: Some(FunctionProperties::fixture()),
        }
    }
}

/// Result of a list Functions operation.
///
/// **Azure API**: `functions.v1.FunctionListResult`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/appservice//FunctionListResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionListResult {
    /// List of Functions
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Function>,

    /// Link to next page of results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl FunctionListResult {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            value: vec![],
            next_link: Some("test-next_link".into()),
        }
    }
}

/// Result of listing application settings.
///
/// **Azure API**: `functions.v1.AppSettingsResult`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/appservice//AppSettingsResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettingsResult {
    /// Resource ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Resource name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Resource type
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// Application settings key-value pairs
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub properties: HashMap<String, String>,
}

impl AppSettingsResult {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            name: Some("test-app_settings_result".into()),
            r#type: Some("test-type".into()),
            properties: Default::default(),
        }
    }
}

/// Request body for updating application settings.
///
/// **Azure API**: `functions.v1.AppSettingsUpdateRequest`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/appservice//AppSettingsUpdateRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettingsUpdateRequest {
    /// Application settings key-value pairs
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub properties: HashMap<String, String>,
}

impl AppSettingsUpdateRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            properties: Default::default(),
        }
    }
}
