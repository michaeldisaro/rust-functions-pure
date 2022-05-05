use std::collections::HashSet;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct CosmosQueryResponse<T> {
    pub Documents: Vec<T>,
    pub _count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationChannel {
    EMAIL,
    WEBHOOK,
}
impl NotificationChannel {
    fn as_str(&self) -> &'static str {
        match self {
            NotificationChannel::EMAIL => "EMAIL",
            NotificationChannel::WEBHOOK => "WEBHOOK",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Scope {
    LOCAL,
    NATIONAL,
}
impl Scope {
    fn as_str(&self) -> &'static str {
        match self {
            Scope::LOCAL => "LOCAL",
            Scope::NATIONAL => "NATIONAL",
        }
    }
}
impl Default for Scope {
    fn default() -> Self {
        Scope::LOCAL
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Category {
    STANDARD,
    SPECIAL,
}
impl Category {
    fn as_str(&self) -> &'static str {
        match self {
            Category::STANDARD => "STANDARD",
            Category::SPECIAL => "SPECIAL",
        }
    }
}
impl Default for Category {
    fn default() -> Self {
        Category::STANDARD
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceMetadataPublic {
    pub address: Option<String>,
    pub app_android: Option<String>,
    pub app_ios: Option<String>,
    pub category: Category,
    pub cta: Option<String>,
    pub custom_special_flow: Option<String>,
    pub description: Option<String>,
    pub email: Option<String>,
    pub pec: Option<String>,
    pub phone: Option<String>,
    pub privacy_url: Option<String>,
    pub scope: Scope,
    pub support_url: Option<String>,
    pub token_name: Option<String>,
    pub tos_url: Option<String>,
    pub web_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServicePublic {
    pub available_notification_channels: Option<Vec<NotificationChannel>>,
    pub department_name: String,
    pub organization_fiscal_code: String,
    pub organization_name: String,
    pub service_id: String,
    pub service_metadata: Option<ServiceMetadataPublic>,
    pub service_name: String,
    pub version: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[allow(non_snake_case)]
pub struct ServiceMetada {
    pub address: Option<String>,
    pub appAndroid: Option<String>,
    pub appIos: Option<String>,
    pub category: Category,
    pub cta: Option<String>,
    pub customSpecialFlow: Option<String>,
    pub description: Option<String>,
    pub email: Option<String>,
    pub pec: Option<String>,
    pub phone: Option<String>,
    pub privacyUrl: Option<String>,
    pub scope: Scope,
    pub supportUrl: Option<String>,
    pub tokenName: Option<String>,
    pub tosUrl: Option<String>,
    pub webUrl: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Service {
    pub authorizedCIDRs: HashSet<String>,
    pub authorizedRecipients: HashSet<String>,
    pub departmentName: String,
    pub isVisible: bool, // false by default
    pub maxAllowedPaymentAmount: u64,
    pub organizationName: String,
    pub organizationFiscalCode: String,
    pub requireSecureChannels: bool, // false by default
    pub serviceId: String,
    pub serviceMetadata: Option<ServiceMetada>,
    pub serviceName: String,
    pub version: u32,
    pub _ts: u64,
}
