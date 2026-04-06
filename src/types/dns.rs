//! Types for the Azure DNS API (v1).
//!
//! Auto-generated from the Azure ARM REST Specification.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents the properties of the zone.
///
/// **Azure API**: `dns.v1.ZoneProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/dns//ZoneProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZoneProperties {
    /// The type of this DNS zone (Public or Private)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_type: Option<String>,

    /// The current number of record sets in this DNS zone
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_record_sets: Option<i64>,

    /// The maximum number of record sets that can be created in this DNS zone
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_number_of_record_sets: Option<i64>,

    /// The maximum number of records per record set that can be created in this DNS zone
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_number_of_records_per_record_set: Option<i64>,

    /// The name servers for this DNS zone
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub name_servers: Vec<String>,
}

impl ZoneProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            zone_type: Some("test-zone_type".into()),
            number_of_record_sets: Some(100),
            max_number_of_record_sets: Some(100),
            max_number_of_records_per_record_set: Some(100),
            name_servers: vec![],
        }
    }
}

/// Describes a DNS zone.
///
/// **Azure API**: `dns.v1.Zone`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/dns//Zone>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Zone {
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

    /// Resource location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    /// Resource tags
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,

    /// The etag of the zone
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,

    /// The properties of the zone
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ZoneProperties>,
}

impl Zone {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            name: Some("test-zone".into()),
            r#type: Some("test-type".into()),
            location: Some("test-location".into()),
            tags: Default::default(),
            etag: Some("test-etag".into()),
            properties: Some(ZoneProperties::fixture()),
        }
    }
}

/// The response to a Zone List or ListAll operation.
///
/// **Azure API**: `dns.v1.ZoneListResult`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/dns//ZoneListResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZoneListResult {
    /// Information about a DNS zone
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Zone>,

    /// The continuation token for the next page of results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl ZoneListResult {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            value: vec![],
            next_link: Some("test-next_link".into()),
        }
    }
}

/// Request body for creating or updating a DNS zone.
///
/// **Azure API**: `dns.v1.ZoneCreateRequest`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/dns//ZoneCreateRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZoneCreateRequest {
    /// Resource location
    pub location: String,

    /// Resource tags
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,

    /// The properties of the zone
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ZoneProperties>,
}

impl ZoneCreateRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            location: "test-location".into(),
            tags: Default::default(),
            properties: Some(ZoneProperties::fixture()),
        }
    }
}

/// An A record.
///
/// **Azure API**: `dns.v1.ARecord`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/dns//ARecord>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ARecord {
    /// The IPv4 address of this A record
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_address: Option<String>,
}

impl ARecord {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            ipv4_address: Some("test-ipv4_address".into()),
        }
    }
}

/// An AAAA record.
///
/// **Azure API**: `dns.v1.AaaaRecord`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/dns//AaaaRecord>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AaaaRecord {
    /// The IPv6 address of this AAAA record
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_address: Option<String>,
}

impl AaaaRecord {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            ipv6_address: Some("test-ipv6_address".into()),
        }
    }
}

/// A CNAME record.
///
/// **Azure API**: `dns.v1.CnameRecord`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/dns//CnameRecord>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CnameRecord {
    /// The canonical name for this CNAME record
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cname: Option<String>,
}

impl CnameRecord {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            cname: Some("test-cname".into()),
        }
    }
}

/// An MX record.
///
/// **Azure API**: `dns.v1.MxRecord`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/dns//MxRecord>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MxRecord {
    /// The preference value for this MX record
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<i32>,

    /// The domain name of the mail host for this MX record
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange: Option<String>,
}

impl MxRecord {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            preference: Some(100),
            exchange: Some("test-exchange".into()),
        }
    }
}

/// A TXT record.
///
/// **Azure API**: `dns.v1.TxtRecord`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/dns//TxtRecord>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxtRecord {
    /// The text value of this TXT record
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<String>,
}

impl TxtRecord {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self { value: vec![] }
    }
}

/// An NS record.
///
/// **Azure API**: `dns.v1.NsRecord`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/dns//NsRecord>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NsRecord {
    /// The name server name for this NS record
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsdname: Option<String>,
}

impl NsRecord {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            nsdname: Some("test-nsdname".into()),
        }
    }
}

/// Represents the properties of the records in the RecordSet.
///
/// **Azure API**: `dns.v1.RecordSetProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/dns//RecordSetProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordSetProperties {
    /// The metadata attached to the record set
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, String>,

    /// The TTL (time-to-live) of the records in the record set
    #[serde(rename = "TTL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i64>,

    /// Fully qualified domain name of the record set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,

    /// Provisioning State of the record set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,

    /// The list of A records in the record set
    #[serde(rename = "ARecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub a_records: Vec<ARecord>,

    /// The list of AAAA records in the record set
    #[serde(rename = "AAAARecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub aaaa_records: Vec<AaaaRecord>,

    /// The CNAME record in the record set
    #[serde(rename = "CNAMERecord")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cname_record: Option<CnameRecord>,

    /// The list of MX records in the record set
    #[serde(rename = "MXRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub mx_records: Vec<MxRecord>,

    /// The list of TXT records in the record set
    #[serde(rename = "TXTRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub txt_records: Vec<TxtRecord>,

    /// The list of NS records in the record set
    #[serde(rename = "NSRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ns_records: Vec<NsRecord>,
}

impl RecordSetProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            metadata: Default::default(),
            ttl: Some(100),
            fqdn: Some("test-fqdn".into()),
            provisioning_state: Some("test-provisioning_state".into()),
            a_records: vec![],
            aaaa_records: vec![],
            cname_record: Some(CnameRecord::fixture()),
            mx_records: vec![],
            txt_records: vec![],
            ns_records: vec![],
        }
    }
}

/// Describes a DNS record set (a collection of DNS records with the same name and type).
///
/// **Azure API**: `dns.v1.RecordSet`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/dns//RecordSet>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordSet {
    /// The ID of the record set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The name of the record set, relative to the name of the zone
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The type of the record set
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// The etag of the record set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,

    /// The properties of the record set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<RecordSetProperties>,
}

impl RecordSet {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            name: Some("test-record_set".into()),
            r#type: Some("test-type".into()),
            etag: Some("test-etag".into()),
            properties: Some(RecordSetProperties::fixture()),
        }
    }
}

/// The response to a record set List operation.
///
/// **Azure API**: `dns.v1.RecordSetListResult`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/dns//RecordSetListResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordSetListResult {
    /// Information about a record set within a DNS zone
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RecordSet>,

    /// The continuation token for the next page of results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl RecordSetListResult {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            value: vec![],
            next_link: Some("test-next_link".into()),
        }
    }
}

/// Request body for creating or updating a DNS record set.
///
/// **Azure API**: `dns.v1.RecordSetCreateRequest`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/dns//RecordSetCreateRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordSetCreateRequest {
    /// The etag of the record set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,

    /// The properties of the record set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<RecordSetProperties>,
}

impl RecordSetCreateRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            etag: Some("test-etag".into()),
            properties: Some(RecordSetProperties::fixture()),
        }
    }
}
