use base64::encode as encode_base64;
use base64::decode as decode_base64;
use chrono::{DateTime, Utc};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use url::form_urlencoded;
use urlencoding::encode as encode_url;

pub enum CosmosVerb {
    POST,
    GET,
}

impl CosmosVerb {
    fn as_str(&self) -> &'static str {
        match self {
            CosmosVerb::POST => "post",
            CosmosVerb::GET => "get",
        }
    }
}

pub enum CosmosResurceType {
    Database,
    Container,
    StoredProcedures,
    UserDefinedFunctions,
    Triggers,
    Users,
    Permissions,
    Documents,
}

impl CosmosResurceType {
    fn as_str(&self) -> &'static str {
        match self {
            CosmosResurceType::Database => "dbs",
            CosmosResurceType::Container => "colls",
            CosmosResurceType::StoredProcedures => "sprocs",
            CosmosResurceType::UserDefinedFunctions => "udfs",
            CosmosResurceType::Triggers => "triggers",
            CosmosResurceType::Users => "users",
            CosmosResurceType::Permissions => "permissions",
            CosmosResurceType::Documents => "docs",
        }
    }
}

pub fn get_authorization_token_using_master_key(
    verb: CosmosVerb,
    resource_type: CosmosResurceType,
    resource_id: String,
    date: &DateTime<Utc>,
    master_key: String,
) -> Result<String, String> {
    let formatted_date = format_date(&date);
    let primary = decode_base64(master_key).unwrap();

    let text = format!(
        "{}\n{}\n{}\n{}\n{}\n",
        verb.as_str(),
        resource_type.as_str(),
        resource_id.as_str(),
        formatted_date.to_lowercase(),
        ""
    );

    println!("{}", text);

    // Create alias for HMAC-SHA256
    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_from_slice(&primary).map_err(|op| op.to_string())?;
    mac.update(text.as_bytes());
    let result = mac.finalize();
    let signature = encode_base64(result.into_bytes());
    let par = form_urlencoded::byte_serialize(&format!("type=master&ver=1.0&sig={}", signature).as_bytes()).collect::<String>();
    println!("{}", par);
    Ok(par)
}

pub fn format_date(date: &DateTime<Utc>) -> String {
    date.format(&"%a, %d %b %Y %T GMT").to_string()
}