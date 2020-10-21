#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvailableOperation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<AvailableOperationDisplay>,
    #[serde(rename = "isDataAction", skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<available_operation::Origin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<AvailableOperationDisplayPropertyServiceSpecification>,
}
mod available_operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Origin {
        #[serde(rename = "user")]
        User,
        #[serde(rename = "system")]
        System,
        #[serde(rename = "user,system")]
        UserSystem,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvailableOperationDisplay {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvailableOperationDisplayPropertyServiceSpecification {
    #[serde(rename = "serviceSpecification", skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<AvailableOperationDisplayPropertyServiceSpecificationMetricsList>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvailableOperationDisplayPropertyServiceSpecificationMetricsItem {
    #[serde(rename = "aggregationType")]
    pub aggregation_type: available_operation_display_property_service_specification_metrics_item::AggregationType,
    #[serde(rename = "displayDescription")]
    pub display_description: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub name: String,
    pub unit: String,
}
mod available_operation_display_property_service_specification_metrics_item {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AggregationType {
        Average,
        Total,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvailableOperationDisplayPropertyServiceSpecificationMetricsList {
    #[serde(rename = "metricSpecifications", skip_serializing_if = "Vec::is_empty")]
    pub metric_specifications: Vec<AvailableOperationDisplayPropertyServiceSpecificationMetricsItem>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvailableOperationsListResponse {
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AvailableOperation>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CsrpError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<CsrpErrorBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CsrpErrorBody {
    #[serde(skip_serializing)]
    pub code: Option<String>,
    #[serde(skip_serializing)]
    pub details: Vec<CsrpErrorBody>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomizationHostName {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<customization_host_name::Type>,
}
mod customization_host_name {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "USER_DEFINED")]
        UserDefined,
        #[serde(rename = "PREFIX_BASED")]
        PrefixBased,
        #[serde(rename = "FIXED")]
        Fixed,
        #[serde(rename = "VIRTUAL_MACHINE_NAME")]
        VirtualMachineName,
        #[serde(rename = "CUSTOM_NAME")]
        CustomName,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomizationIpAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub argument: Option<String>,
    #[serde(rename = "ipAddress", skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<customization_ip_address::Type>,
}
mod customization_ip_address {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "CUSTOM")]
        Custom,
        #[serde(rename = "DHCP_IP")]
        DhcpIp,
        #[serde(rename = "FIXED_IP")]
        FixedIp,
        #[serde(rename = "USER_DEFINED")]
        UserDefined,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomizationIpSettings {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub gateway: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<CustomizationIpAddress>,
    #[serde(rename = "subnetMask", skip_serializing_if = "Option::is_none")]
    pub subnet_mask: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomizationIdentity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(rename = "hostName", skip_serializing_if = "Option::is_none")]
    pub host_name: Option<CustomizationHostName>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<customization_identity::Type>,
    #[serde(rename = "userData", skip_serializing_if = "Option::is_none")]
    pub user_data: Option<customization_identity::UserData>,
}
mod customization_identity {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "WINDOWS_TEXT")]
        WindowsText,
        #[serde(rename = "WINDOWS")]
        Windows,
        #[serde(rename = "LINUX")]
        Linux,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct UserData {
        #[serde(rename = "isPasswordPredefined", skip_serializing_if = "Option::is_none")]
        pub is_password_predefined: Option<bool>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomizationNicSetting {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adapter: Option<CustomizationIpSettings>,
    #[serde(rename = "macAddress", skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomizationPoliciesListResponse {
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CustomizationPolicy>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomizationPolicy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<CustomizationPolicyProperties>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomizationPolicyProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "privateCloudId", skip_serializing_if = "Option::is_none")]
    pub private_cloud_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specification: Option<CustomizationSpecification>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<customization_policy_properties::Type>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
mod customization_policy_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "LINUX")]
        Linux,
        #[serde(rename = "WINDOWS")]
        Windows,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomizationSpecification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<CustomizationIdentity>,
    #[serde(rename = "nicSettings", skip_serializing_if = "Vec::is_empty")]
    pub nic_settings: Vec<CustomizationNicSetting>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DedicatedCloudNode {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    pub location: String,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<DedicatedCloudNodeProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DedicatedCloudNodeListResponse {
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DedicatedCloudNode>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DedicatedCloudNodeProperties {
    #[serde(rename = "availabilityZoneId")]
    pub availability_zone_id: String,
    #[serde(rename = "availabilityZoneName", skip_serializing)]
    pub availability_zone_name: Option<String>,
    #[serde(rename = "cloudRackName", skip_serializing)]
    pub cloud_rack_name: Option<String>,
    #[serde(skip_serializing)]
    pub created: Option<serde_json::Value>,
    #[serde(rename = "nodesCount")]
    pub nodes_count: i64,
    #[serde(rename = "placementGroupId")]
    pub placement_group_id: String,
    #[serde(rename = "placementGroupName", skip_serializing)]
    pub placement_group_name: Option<String>,
    #[serde(rename = "privateCloudId", skip_serializing)]
    pub private_cloud_id: Option<String>,
    #[serde(rename = "privateCloudName", skip_serializing)]
    pub private_cloud_name: Option<String>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<String>,
    #[serde(rename = "purchaseId")]
    pub purchase_id: String,
    #[serde(rename = "skuDescription", skip_serializing_if = "Option::is_none")]
    pub sku_description: Option<SkuDescription>,
    #[serde(skip_serializing)]
    pub status: Option<dedicated_cloud_node_properties::Status>,
    #[serde(rename = "vmwareClusterName", skip_serializing)]
    pub vmware_cluster_name: Option<String>,
}
mod dedicated_cloud_node_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "unused")]
        Unused,
        #[serde(rename = "used")]
        Used,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DedicatedCloudService {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    pub location: String,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<DedicatedCloudServiceProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DedicatedCloudServiceListResponse {
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DedicatedCloudService>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DedicatedCloudServiceProperties {
    #[serde(rename = "gatewaySubnet")]
    pub gateway_subnet: String,
    #[serde(rename = "isAccountOnboarded", skip_serializing)]
    pub is_account_onboarded: Option<dedicated_cloud_service_properties::IsAccountOnboarded>,
    #[serde(skip_serializing)]
    pub nodes: Option<i64>,
    #[serde(rename = "serviceURL", skip_serializing)]
    pub service_url: Option<String>,
}
mod dedicated_cloud_service_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum IsAccountOnboarded {
        #[serde(rename = "notOnBoarded")]
        NotOnBoarded,
        #[serde(rename = "onBoarded")]
        OnBoarded,
        #[serde(rename = "onBoardingFailed")]
        OnBoardingFailed,
        #[serde(rename = "onBoarding")]
        OnBoarding,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GuestOsCustomization {
    #[serde(rename = "dnsServers", skip_serializing_if = "Vec::is_empty")]
    pub dns_servers: Vec<Ipv4Address>,
    #[serde(rename = "hostName", skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "policyId", skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GuestOsnicCustomization {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocation: Option<guest_osnic_customization::Allocation>,
    #[serde(rename = "dnsServers", skip_serializing_if = "Vec::is_empty")]
    pub dns_servers: Vec<Ipv4Address>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub gateway: Vec<Ipv4Address>,
    #[serde(rename = "ipAddress", skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<Ipv4Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask: Option<Ipv4Address>,
    #[serde(rename = "primaryWinsServer", skip_serializing_if = "Option::is_none")]
    pub primary_wins_server: Option<Ipv4Address>,
    #[serde(rename = "secondaryWinsServer", skip_serializing_if = "Option::is_none")]
    pub secondary_wins_server: Option<Ipv4Address>,
}
mod guest_osnic_customization {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Allocation {
        #[serde(rename = "static")]
        Static,
        #[serde(rename = "dynamic")]
        Dynamic,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ipv4Address {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationResource {
    #[serde(rename = "endTime", skip_serializing)]
    pub end_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<OperationError>,
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "startTime", skip_serializing)]
    pub start_time: Option<String>,
    #[serde(skip_serializing)]
    pub status: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateCloud {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateCloudProperties>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<private_cloud::Type>,
}
mod private_cloud {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.VMwareCloudSimple/privateClouds")]
        Microsoft_VMwareCloudSimplePrivateClouds,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateCloudList {
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateCloud>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateCloudProperties {
    #[serde(rename = "availabilityZoneId", skip_serializing_if = "Option::is_none")]
    pub availability_zone_id: Option<String>,
    #[serde(rename = "availabilityZoneName", skip_serializing_if = "Option::is_none")]
    pub availability_zone_name: Option<String>,
    #[serde(rename = "clustersNumber", skip_serializing_if = "Option::is_none")]
    pub clusters_number: Option<i64>,
    #[serde(rename = "createdBy", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "createdOn", skip_serializing_if = "Option::is_none")]
    pub created_on: Option<String>,
    #[serde(rename = "dnsServers", skip_serializing_if = "Vec::is_empty")]
    pub dns_servers: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<String>,
    #[serde(rename = "nsxType", skip_serializing_if = "Option::is_none")]
    pub nsx_type: Option<String>,
    #[serde(rename = "placementGroupId", skip_serializing_if = "Option::is_none")]
    pub placement_group_id: Option<String>,
    #[serde(rename = "placementGroupName", skip_serializing_if = "Option::is_none")]
    pub placement_group_name: Option<String>,
    #[serde(rename = "privateCloudId", skip_serializing_if = "Option::is_none")]
    pub private_cloud_id: Option<String>,
    #[serde(rename = "resourcePools", skip_serializing_if = "Vec::is_empty")]
    pub resource_pools: Vec<ResourcePool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "totalCpuCores", skip_serializing_if = "Option::is_none")]
    pub total_cpu_cores: Option<i64>,
    #[serde(rename = "totalNodes", skip_serializing_if = "Option::is_none")]
    pub total_nodes: Option<i64>,
    #[serde(rename = "totalRam", skip_serializing_if = "Option::is_none")]
    pub total_ram: Option<i64>,
    #[serde(rename = "totalStorage", skip_serializing_if = "Option::is_none")]
    pub total_storage: Option<f64>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "vSphereVersion", skip_serializing_if = "Option::is_none")]
    pub v_sphere_version: Option<String>,
    #[serde(rename = "vcenterFqdn", skip_serializing_if = "Option::is_none")]
    pub vcenter_fqdn: Option<String>,
    #[serde(rename = "vcenterRefid", skip_serializing_if = "Option::is_none")]
    pub vcenter_refid: Option<String>,
    #[serde(rename = "virtualMachineTemplates", skip_serializing_if = "Vec::is_empty")]
    pub virtual_machine_templates: Vec<VirtualMachineTemplate>,
    #[serde(rename = "virtualNetworks", skip_serializing_if = "Vec::is_empty")]
    pub virtual_networks: Vec<VirtualNetwork>,
    #[serde(rename = "vrOpsEnabled", skip_serializing_if = "Option::is_none")]
    pub vr_ops_enabled: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourcePool {
    pub id: String,
    #[serde(skip_serializing)]
    pub location: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "privateCloudId", skip_serializing)]
    pub private_cloud_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ResourcePoolProperties>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourcePoolProperties {
    #[serde(rename = "fullName", skip_serializing)]
    pub full_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourcePoolsListResponse {
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ResourcePool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuAvailability {
    #[serde(rename = "dedicatedAvailabilityZoneId", skip_serializing_if = "Option::is_none")]
    pub dedicated_availability_zone_id: Option<String>,
    #[serde(rename = "dedicatedAvailabilityZoneName", skip_serializing_if = "Option::is_none")]
    pub dedicated_availability_zone_name: Option<String>,
    #[serde(rename = "dedicatedPlacementGroupId", skip_serializing_if = "Option::is_none")]
    pub dedicated_placement_group_id: Option<String>,
    #[serde(rename = "dedicatedPlacementGroupName", skip_serializing_if = "Option::is_none")]
    pub dedicated_placement_group_name: Option<String>,
    pub limit: i64,
    #[serde(rename = "resourceType", skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "skuId", skip_serializing_if = "Option::is_none")]
    pub sku_id: Option<String>,
    #[serde(rename = "skuName", skip_serializing_if = "Option::is_none")]
    pub sku_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuAvailabilityListResponse {
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SkuAvailability>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuDescription {
    pub id: String,
    pub name: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tags {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Usage {
    #[serde(rename = "currentValue")]
    pub current_value: i64,
    pub limit: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<UsageName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<usage::Unit>,
}
mod usage {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Unit {
        Count,
        Bytes,
        Seconds,
        Percent,
        CountPerSecond,
        BytesPerSecond,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageListResponse {
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(skip_serializing)]
    pub value: Vec<Usage>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageName {
    #[serde(rename = "localizedValue", skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualDisk {
    #[serde(rename = "controllerId")]
    pub controller_id: String,
    #[serde(rename = "independenceMode")]
    pub independence_mode: virtual_disk::IndependenceMode,
    #[serde(rename = "totalSize")]
    pub total_size: i64,
    #[serde(rename = "virtualDiskId", skip_serializing_if = "Option::is_none")]
    pub virtual_disk_id: Option<String>,
    #[serde(rename = "virtualDiskName", skip_serializing)]
    pub virtual_disk_name: Option<String>,
}
mod virtual_disk {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum IndependenceMode {
        #[serde(rename = "persistent")]
        Persistent,
        #[serde(rename = "independent_persistent")]
        IndependentPersistent,
        #[serde(rename = "independent_nonpersistent")]
        IndependentNonpersistent,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualDiskController {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "subType", skip_serializing)]
    pub sub_type: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachine {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    pub location: String,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualMachineProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineListResponse {
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VirtualMachine>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineProperties {
    #[serde(rename = "amountOfRam")]
    pub amount_of_ram: i64,
    #[serde(skip_serializing)]
    pub controllers: Vec<VirtualDiskController>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customization: Option<GuestOsCustomization>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub disks: Vec<VirtualDisk>,
    #[serde(skip_serializing)]
    pub dnsname: Option<String>,
    #[serde(rename = "exposeToGuestVM", skip_serializing_if = "Option::is_none")]
    pub expose_to_guest_vm: Option<bool>,
    #[serde(skip_serializing)]
    pub folder: Option<String>,
    #[serde(rename = "guestOS", skip_serializing)]
    pub guest_os: Option<String>,
    #[serde(rename = "guestOSType", skip_serializing)]
    pub guest_os_type: Option<virtual_machine_properties::GuestOsType>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nics: Vec<VirtualNic>,
    #[serde(rename = "numberOfCores")]
    pub number_of_cores: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "privateCloudId")]
    pub private_cloud_id: String,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<String>,
    #[serde(rename = "publicIP", skip_serializing)]
    pub public_ip: Option<String>,
    #[serde(rename = "resourcePool", skip_serializing_if = "Option::is_none")]
    pub resource_pool: Option<ResourcePool>,
    #[serde(skip_serializing)]
    pub status: Option<virtual_machine_properties::Status>,
    #[serde(rename = "templateId", skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "vSphereNetworks", skip_serializing_if = "Vec::is_empty")]
    pub v_sphere_networks: Vec<String>,
    #[serde(rename = "vmId", skip_serializing)]
    pub vm_id: Option<String>,
    #[serde(skip_serializing)]
    pub vmwaretools: Option<String>,
}
mod virtual_machine_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum GuestOsType {
        #[serde(rename = "linux")]
        Linux,
        #[serde(rename = "windows")]
        Windows,
        #[serde(rename = "other")]
        Other,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "running")]
        Running,
        #[serde(rename = "suspended")]
        Suspended,
        #[serde(rename = "poweredoff")]
        Poweredoff,
        #[serde(rename = "updating")]
        Updating,
        #[serde(rename = "deallocating")]
        Deallocating,
        #[serde(rename = "deleting")]
        Deleting,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineStopMode {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<virtual_machine_stop_mode::Mode>,
}
mod virtual_machine_stop_mode {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Mode {
        #[serde(rename = "reboot")]
        Reboot,
        #[serde(rename = "suspend")]
        Suspend,
        #[serde(rename = "shutdown")]
        Shutdown,
        #[serde(rename = "poweroff")]
        Poweroff,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineTemplate {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualMachineTemplateProperties>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineTemplateListResponse {
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VirtualMachineTemplate>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineTemplateProperties {
    #[serde(rename = "amountOfRam", skip_serializing_if = "Option::is_none")]
    pub amount_of_ram: Option<i64>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub controllers: Vec<VirtualDiskController>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub disks: Vec<VirtualDisk>,
    #[serde(rename = "exposeToGuestVM", skip_serializing_if = "Option::is_none")]
    pub expose_to_guest_vm: Option<bool>,
    #[serde(rename = "guestOS", skip_serializing)]
    pub guest_os: Option<String>,
    #[serde(rename = "guestOSType", skip_serializing)]
    pub guest_os_type: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nics: Vec<VirtualNic>,
    #[serde(rename = "numberOfCores", skip_serializing_if = "Option::is_none")]
    pub number_of_cores: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "privateCloudId")]
    pub private_cloud_id: String,
    #[serde(rename = "vSphereNetworks", skip_serializing_if = "Vec::is_empty")]
    pub v_sphere_networks: Vec<String>,
    #[serde(rename = "vSphereTags", skip_serializing_if = "Vec::is_empty")]
    pub v_sphere_tags: Vec<String>,
    #[serde(skip_serializing)]
    pub vmwaretools: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetwork {
    #[serde(skip_serializing)]
    pub assignable: Option<bool>,
    pub id: String,
    #[serde(skip_serializing)]
    pub location: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualNetworkProperties>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkListResponse {
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VirtualNetwork>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkProperties {
    #[serde(rename = "privateCloudId", skip_serializing)]
    pub private_cloud_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNic {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customization: Option<GuestOsnicCustomization>,
    #[serde(rename = "ipAddresses", skip_serializing_if = "Vec::is_empty")]
    pub ip_addresses: Vec<String>,
    #[serde(rename = "macAddress", skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    pub network: VirtualNetwork,
    #[serde(rename = "nicType")]
    pub nic_type: virtual_nic::NicType,
    #[serde(rename = "powerOnBoot", skip_serializing_if = "Option::is_none")]
    pub power_on_boot: Option<bool>,
    #[serde(rename = "virtualNicId", skip_serializing_if = "Option::is_none")]
    pub virtual_nic_id: Option<String>,
    #[serde(rename = "virtualNicName", skip_serializing)]
    pub virtual_nic_name: Option<String>,
}
mod virtual_nic {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum NicType {
        E1000,
        #[serde(rename = "E1000E")]
        E1000e,
        #[serde(rename = "PCNET32")]
        Pcnet32,
        #[serde(rename = "VMXNET")]
        Vmxnet,
        #[serde(rename = "VMXNET2")]
        Vmxnet2,
        #[serde(rename = "VMXNET3")]
        Vmxnet3,
    }
}