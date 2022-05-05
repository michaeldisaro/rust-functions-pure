use chrono::Utc;
use cosmos::*;
use std::env;
use std::net::Ipv4Addr;
use types::*;
use warp::{http::Response, reject, Filter};

mod cosmos;
mod types;

#[tokio::main]
async fn main() {
    let example1 = warp::get()
        .and(warp::path("api"))
        .and(warp::path("v1"))
        .and(warp::path("services"))
        .and(warp::path::param::<String>())
        .and_then(|id: String| async move {
            get_value(
                &format!(
                    "{}dbs/{}/colls/{}/docs",
                    env::var("COSMOSDB_URI").unwrap(),
                    env::var("COSMOSDB_NAME").unwrap(),
                    "services"
                ),
                &id,
                &env::var("COSMOSDB_NAME").unwrap(),
                &env::var("COSMOSDB_KEY").unwrap(),
            )
            .await
            .map(|_service| warp::reply::json(&_service))
            .map_err(|_err| reject::reject())
        });

    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    warp::serve(example1)
        .run((Ipv4Addr::UNSPECIFIED, port))
        .await
}

async fn get_value(
    url: &str,
    service_id: &str,
    db_name: &str,
    db_key: &str,
) -> Result<ServicePublic, String> {
    let date = Utc::now();
    let auth_token = get_authorization_token_using_master_key(
        CosmosVerb::POST,
        CosmosResurceType::Documents,
        format!("dbs/{}/colls/{}", db_name, "services"),
        &date,
        db_key.to_owned(),
    )?;

    let req = reqwest::Client::new()
        .post(url)
        .header("x-ms-documentdb-isquery", "True")
        .header("x-ms-date", format_date(&date))
        .header("x-ms-version", "2018-12-31")
        .header("Content-Type", "application/query+json")
        .header("authorization", auth_token)
        .body(format!(
            "{{ \"query\": \"SELECT TOP 1 * FROM c WHERE c.serviceId = '{}' ORDER BY c.version DESC\"}}",
            service_id
        ));

    let res = req.send().await.map_err(|err| err.to_string())?;

    let cosmos_response: CosmosQueryResponse<Service> =
        res.json().await.map_err(|err| err.to_string())?;
    let retrieved_service = &cosmos_response.Documents[0];
    let service: ServicePublic = ServicePublic {
        available_notification_channels: if retrieved_service.requireSecureChannels {
            Some(vec![NotificationChannel::WEBHOOK])
        } else {
            Some(vec![
                NotificationChannel::WEBHOOK,
                NotificationChannel::EMAIL,
            ])
        },
        department_name: String::from(&retrieved_service.departmentName),
        organization_fiscal_code: String::from(&retrieved_service.organizationFiscalCode),
        organization_name: String::from(&retrieved_service.organizationName),
        service_id: String::from(&retrieved_service.serviceId),
        service_metadata: retrieved_service.serviceMetadata.as_ref().map(|s| {
            ServiceMetadataPublic {
                address: s.address.as_ref().map(|a| a.to_string()),
                app_android: s.appAndroid.as_ref().map(|a| a.to_string()),
                app_ios: s.appIos.as_ref().map(|a| a.to_string()),
                category: s.category,
                cta: s.cta.as_ref().map(|a| a.to_string()),
                custom_special_flow: s.customSpecialFlow.as_ref().map(|a| a.to_string()),
                description: s.description.as_ref().map(|a| a.to_string()),
                email: s.email.as_ref().map(|a| a.to_string()),
                pec: s.pec.as_ref().map(|a| a.to_string()),
                phone: s.phone.as_ref().map(|a| a.to_string()),
                privacy_url: s.privacyUrl.as_ref().map(|a| a.to_string()),
                scope: s.scope,
                support_url: s.supportUrl.as_ref().map(|a| a.to_string()),
                token_name: s.tokenName.as_ref().map(|a| a.to_string()),
                tos_url: s.tosUrl.as_ref().map(|a| a.to_string()),
                web_url: s.webUrl.as_ref().map(|a| a.to_string()),
            }
        }),
        service_name: String::from(&retrieved_service.serviceName),
        version: retrieved_service.version,
    };
    Ok(service.to_owned())
}
