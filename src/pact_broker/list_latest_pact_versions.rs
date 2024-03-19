use crate::pact_broker::{HALClient, Link, PactBrokerError};
use comfy_table::{presets::UTF8_FULL, Table};
use maplit::hashmap;
use serde_json::Value;

use super::{
    types::{BrokerDetails, OutputType},
    utils::generate_table,
};

pub async fn get_broker_relation(
    hal_client: HALClient,
    relation: String,
    broker_url: String,
) -> String {
    let index_res: Result<Value, PactBrokerError> = hal_client.clone().fetch("/").await;
    match index_res {
        Ok(_) => {
            let index_res_clone = index_res.clone().unwrap();
            index_res_clone
                .get("_links")
                .unwrap()
                .get(relation)
                .unwrap()
                .get("href")
                .unwrap()
                .to_string()
                .split(&broker_url)
                .collect::<Vec<&str>>()[1]
                .to_string()
                .replace("\"", "")
                .to_string()
        }
        Err(err) => {
            return err.to_string();
        }
    }
}

pub async fn follow_broker_relation(
    hal_client: HALClient,
    relation: String,
    relation_href: String,
) -> Result<Value, PactBrokerError> {
    let link = Link {
        name: relation,
        href: Some(relation_href),
        templated: false,
        title: None,
    };
    let template_values = hashmap! {};
    hal_client.fetch_url(&link, &template_values).await
}

pub fn list_latest_pact_versions(
    broker_details: &BrokerDetails,
    output_type: OutputType,
    verbose: bool,
) -> Result<String, PactBrokerError> {
    // setup client with broker url and credentials
    let broker_url = &broker_details.url;
    let auth = &broker_details.auth;
    let res = tokio::runtime::Runtime::new().unwrap().block_on(async {
        // query pact broker index and get hal relation link
        let hal_client: HALClient = HALClient::with_url(broker_url, auth.clone());
        let pb_latest_pact_versions_href_path = get_broker_relation(
            hal_client.clone(),
            "pb:latest-pact-versions".to_string(),
            broker_url.to_string(),
        )
        .await;

        // query the hal relation link to get the latest pact versions
        let res = follow_broker_relation(
            hal_client.clone(),
            "pb:latest-pact-versions".to_string(),
            pb_latest_pact_versions_href_path,
        )
        .await;
        match res {
            Ok(result) => match output_type {
                OutputType::Json => {
                    let json: String = serde_json::to_string(&result).unwrap();
                    println!("{}", json);
                    return Ok(json);
                }
                OutputType::Table => {
                    let table = generate_table(
                        &result,
                        vec!["CONSUMER", "CONSUMER_VERSION", "PROVIDER", "CREATED_AT"],
                        vec![
                            vec!["_embedded", "consumer", "name"],
                            vec!["_embedded", "consumer", "_embedded", "version", "number"],
                            vec!["_embedded", "provider", "name"],
                            vec!["createdAt"],
                        ],
                    );
                    println!("{table}");
                    return Ok(table.to_string());
                }

                OutputType::Text => {
                    let text = result.to_string();
                    println!("{:?}", text);
                    return Ok(text);
                }
                OutputType::Pretty => {
                    let json: String = serde_json::to_string(&result).unwrap();
                    println!("{}", json);
                    return Ok(json);
                }
            },
            Err(err) => Err(err),
        }
    });
    match res {
        Ok(result) => {
            return Ok(result);
        }
        Err(err) => {
            return Err(err);
        }
    }
}
