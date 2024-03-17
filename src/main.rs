// use std::collections::HashMap;
mod cli;
use clap::error::ErrorKind;
use clap_complete::{generate_to, Shell};
use pact_broker::{HALClient, Link, PactBrokerError};
use serde_json::json;
use serde_json::Value;
use std::env;
use std::fs;
use std::io::Write;
use std::process::Command;
use std::str::FromStr;
mod pact_broker;
mod pact_plugin_cli;
use crate::cli::pact_mock_server_cli;
use crate::cli::pact_stub_server_cli;
use crate::cli::pact_verifier_cli;
use comfy_table::presets::UTF8_FULL;
use comfy_table::Table;

use ansi_term::Colour;
use maplit::hashmap;
use pact_models::http_utils::HttpAuth;
use tabled::{builder::Builder, settings::Style};

fn get_broker_url(args: &clap::ArgMatches) -> String {
    args.get_one::<String>("broker-base-url")
        .expect("url is required")
        .to_string()
}
// setup client with broker url and credentials
fn get_auth(args: &clap::ArgMatches) -> HttpAuth {
    let token = args.try_get_one::<String>("broker-token");
    let username = args.try_get_one::<String>("broker-username");
    let password = args.try_get_one::<String>("broker-password");
    let auth;

    match token {
        Ok(Some(token)) => {
            auth = HttpAuth::Token(token.to_string());
        }
        Ok(None) => match username {
            Ok(Some(username)) => match password {
                Ok(Some(password)) => {
                    auth = HttpAuth::User(username.to_string(), Some(password.to_string()));
                }
                Ok(None) => {
                    auth = HttpAuth::User(username.to_string(), None);
                }
                Err(_) => todo!(),
            },
            Ok(None) => {
                auth = HttpAuth::None;
            }
            Err(_) => todo!(),
        },
        Err(_) => todo!(),
    }

    auth
}

async fn get_broker_relation(
    hal_client: HALClient,
    relation: String,
    broker_url: String,
) -> String {
    let index_res: Result<Value, PactBrokerError> = hal_client.clone().fetch("/").await;
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

async fn follow_broker_relation(
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

fn generate_table(res: &Value, columns: Vec<&str>, names: Vec<Vec<&str>>) {
    let mut builder = Builder::default();
    builder.push_record(columns);

    if let Some(items) = res.get("pacts").unwrap().as_array() {
        for item in items {
            let mut values = vec![item; names.len()];

            for (i, name) in names.iter().enumerate() {
                for n in name.clone() {
                    values[i] = values[i].get(n).unwrap();
                }
            }

            let records: Vec<String> = values.iter().map(|v| v.to_string()).collect();
            builder.push_record(records.as_slice());
        }
    }
    let mut table = builder.build();
    table.with(Style::rounded());

    println!("{:#}", table);
}

pub fn main() {
    let app = cli::build_cli();
    let matches = app.clone().try_get_matches();

    match matches {
        Ok(results) => {
            match results.subcommand() {
                Some(("pact-broker", args)) => {
                    match args.subcommand() {
                        Some(("publish", args)) => {
                            println!("{:?}", args);
                            // // Ok(());
                        }
                        Some(("list-latest-pact-versions", args)) => {
                            // Handle list-latest-pact-versions command

                            // setup client with broker url and credentials
                            let broker_url = get_broker_url(args);
                            let auth = get_auth(args);
                            tokio::runtime::Runtime::new().unwrap().block_on(async {
                                // query pact broker index and get hal relation link
                                let hal_client: HALClient =
                                    HALClient::with_url(&broker_url, Some(auth.clone()));
                                let pb_latest_pact_versions_href_path = get_broker_relation(
                                    hal_client.clone(),
                                    "pb:latest-pact-versions".to_string(),
                                    broker_url,
                                )
                                .await;
                                // query the hal relation link to get the latest pact versions
                                let res = follow_broker_relation(
                                    hal_client.clone(),
                                    "pb:latest-pact-versions".to_string(),
                                    pb_latest_pact_versions_href_path,
                                )
                                .await;

                                // handle user args for additional processing
                                let output: Result<Option<&String>, clap::parser::MatchesError> =
                                    args.try_get_one::<String>("output");
                                // render result
                                match output {
                                    Ok(Some(output)) => {
                                        if output == "json" {
                                            let json: String =
                                                serde_json::to_string(&res.unwrap()).unwrap();
                                            println!("{}", json);
                                        } else if output == "table" {
                                            if let Ok(res) = res {
                                                generate_table(
                                                    &res,
                                                    vec![
                                                        "CONSUMER",
                                                        "CONSUMER_VERSION",
                                                        "PROVIDER",
                                                        "CREATED_AT",
                                                    ],
                                                    vec![
                                                        vec!["_embedded", "consumer", "name"],
                                                        vec![
                                                            "_embedded",
                                                            "consumer",
                                                            "_embedded",
                                                            "version",
                                                            "number",
                                                        ],
                                                        vec!["_embedded", "provider", "name"],
                                                        vec!["createdAt"],
                                                    ],
                                                );
                                            }
                                        }
                                    }
                                    Ok(None) => {
                                        println!("{:?}", res.clone());
                                    }
                                    Err(_) => todo!(),
                                }
                            });
                        }
                        Some(("create-environment", args)) => {
                            let name = args.get_one::<String>("name");
                            let display_name = args.get_one::<String>("display-name");
                            let production = args.get_flag("production");
                            let contact_name = args.get_one::<String>("contact-name");
                            let contact_email_address =
                                args.get_one::<String>("contact-email-address");
                            let broker_url = get_broker_url(args);
                            let auth = get_auth(args);
                            tokio::runtime::Runtime::new().unwrap().block_on(async {
                                let hal_client: HALClient =
                                    HALClient::with_url(&broker_url, Some(auth.clone()));

                            let mut payload = json!({});
                            payload["production"] = serde_json::Value::Bool(production);
                            if let Some(name) = name {
                                payload["name"] = serde_json::Value::String(name.to_string());
                            } else {
                                println!("❌ {}", Colour::Red.paint("Name is required"));
                                std::process::exit(1);
                            }
                            if let Some(contact_name) = contact_name {
                                payload["contacts"] = serde_json::Value::Array(vec![{
                                    let mut map = serde_json::Map::new();
                                    map.insert("name".to_string(), serde_json::Value::String(contact_name.to_string()));
                                    serde_json::Value::Object(map)
                                }]);
                            }
                            if let Some(display_name) = display_name {
                                payload["displayName"] = serde_json::Value::String(display_name.to_string());
                            }
                            if let Some(contact_email_address) = contact_email_address {
                               if payload["contacts"].is_array() {
                                    let contacts = payload["contacts"].as_array_mut().unwrap();
                                    let contact = contacts.get_mut(0).unwrap();
                                    let contact_map = contact.as_object_mut().unwrap();
                                    contact_map.insert("email".to_string(), serde_json::Value::String(contact_email_address.to_string()));
                                } else {
                                    payload["contacts"] = serde_json::Value::Array(vec![{
                                        let mut map = serde_json::Map::new();
                                        map.insert("email".to_string(), serde_json::Value::String(contact_email_address.to_string()));
                                        serde_json::Value::Object(map)
                                    }]);
                                }
                            }
                            let res = hal_client.post_json(&(broker_url + "/environments"), &payload.to_string()).await;

                            let default_output = "text".to_string();
                            let output = args.get_one::<String>("output").unwrap_or(&default_output);
                            let columns = vec!["ID", "NAME", "DISPLAY NAME", "PRODUCTION", "CONTACT NAME", "CONTACT EMAIL ADDRESS"];
                            let names = vec![
                                vec!["id"],
                                vec!["name"],
                                vec!["displayName"],
                                vec!["production"],
                                vec!["contactName"],
                                vec!["contactEmailAddress"],
                            ];
                            match res {
                                Ok(res) => {
                                        if output == "pretty" {
                                            let json = serde_json::to_string_pretty(&res).unwrap();
                                            println!("{}", json);
                                        } else if output == "json" {
                                            println!("{}", serde_json::to_string(&res).unwrap());
                                        } else if output == "id" {
                                            println!("{}", res["uuid"].to_string().trim_matches('"'));
                                        } else if output == "table" {
                                            generate_table(
                                                &res,
                                                columns,
                                                names,
                                            );
                                        }
                                        else {
                                            let uuid = res["uuid"].to_string();
                                            println!("✅ Created {} environment in the Pact Broker with UUID {}", Colour::Green.paint(name.unwrap()), Colour::Green.paint(uuid.trim_matches('"')));

                                        }
                                    std::process::exit(0);
                                }
                                Err(err) => {
                                    match err {
                                        // TODO process output based on user selection
                                        PactBrokerError::LinkError(error) => {
                                            println!("❌ {}", Colour::Red.paint(error));
                                            std::process::exit(1);
                                        }
                                        PactBrokerError::ContentError(error) => {
                                            println!("❌ {}", Colour::Red.paint(error));
                                            std::process::exit(1);
                                        }
                                        PactBrokerError::IoError(error) => {
                                            println!("❌ {}", Colour::Red.paint(error));
                                            std::process::exit(1);
                                        }
                                        PactBrokerError::NotFound(error) => {
                                            println!("❌ {}", Colour::Red.paint(error));
                                            std::process::exit(1);
                                        }
                                        PactBrokerError::ValidationError(errors) => {
                                            for error in errors {
                                                println!("❌ {}", Colour::Red.paint(error));
                                            }
                                            std::process::exit(1);
                                        }
                                        _ => {
                                            println!("❌ {}", Colour::Red.paint(err.to_string()));
                                            std::process::exit(1);
                                        }
                                    }
                                }
                            }
                        })
                        }
                        Some(("update-environment", args)) => {
                            let uuid = args.get_one::<String>("uuid").unwrap().to_string();
                            let name = args.get_one::<String>("name");
                            let display_name = args.get_one::<String>("display-name");
                            let production = args.get_flag("production");
                            let contact_name = args.get_one::<String>("contact-name");
                            let contact_email_address =
                                args.get_one::<String>("contact-email-address");
                            let broker_url = get_broker_url(args);
                            let auth = get_auth(args);
                            tokio::runtime::Runtime::new().unwrap().block_on(async {
                                let hal_client: HALClient =
                                    HALClient::with_url(&broker_url, Some(auth.clone()));

                            let mut payload = json!({});
                            payload["uuid"] = serde_json::Value::String(uuid);
                            payload["production"] = serde_json::Value::Bool(production);
                            if let Some(name) = name {
                                payload["name"] = serde_json::Value::String(name.to_string());
                            } else {
                                println!("❌ {}", Colour::Red.paint("Name is required"));
                                std::process::exit(1);
                            }
                            if let Some(contact_name) = contact_name {
                                payload["contacts"] = serde_json::Value::Array(vec![{
                                    let mut map = serde_json::Map::new();
                                    map.insert("name".to_string(), serde_json::Value::String(contact_name.to_string()));
                                    serde_json::Value::Object(map)
                                }]);
                            }
                            if let Some(display_name) = display_name {
                                payload["displayName"] = serde_json::Value::String(display_name.to_string());
                            }
                            if let Some(contact_email_address) = contact_email_address {
                               if payload["contacts"].is_array() {
                                    let contacts = payload["contacts"].as_array_mut().unwrap();
                                    let contact = contacts.get_mut(0).unwrap();
                                    let contact_map = contact.as_object_mut().unwrap();
                                    contact_map.insert("email".to_string(), serde_json::Value::String(contact_email_address.to_string()));
                                } else {
                                    payload["contacts"] = serde_json::Value::Array(vec![{
                                        let mut map = serde_json::Map::new();
                                        map.insert("email".to_string(), serde_json::Value::String(contact_email_address.to_string()));
                                        serde_json::Value::Object(map)
                                    }]);
                                }
                            }
                            let res = hal_client.post_json(&(broker_url + "/environments"), &payload.to_string()).await;

                            let default_output = "text".to_string();
                            let output = args.get_one::<String>("output").unwrap_or(&default_output);
                            let columns = vec!["ID", "NAME", "DISPLAY NAME", "PRODUCTION", "CONTACT NAME", "CONTACT EMAIL ADDRESS"];
                            let names = vec![
                                vec!["id"],
                                vec!["name"],
                                vec!["displayName"],
                                vec!["production"],
                                vec!["contactName"],
                                vec!["contactEmailAddress"],
                            ];
                            match res {
                                Ok(res) => {
                                        if output == "pretty" {
                                            let json = serde_json::to_string_pretty(&res).unwrap();
                                            println!("{}", json);
                                        } else if output == "json" {
                                            println!("{}", serde_json::to_string(&res).unwrap());
                                        } else if output == "id" {
                                            println!("{}", res["uuid"].to_string().trim_matches('"'));
                                        } else if output == "table" {
                                            generate_table(
                                                &res,
                                                columns,
                                                names,
                                            );
                                        }
                                        else {
                                            let uuid = res["uuid"].to_string();
                                            println!("✅ Updated {} environment in the Pact Broker with UUID {}", Colour::Green.paint(name.unwrap()), Colour::Green.paint(uuid.trim_matches('"')));

                                        }

                                    std::process::exit(0);
                                }
                                Err(err) => {
                                    match err {
                                        // TODO process output based on user selection
                                        PactBrokerError::LinkError(error) => {
                                            println!("❌ {}", Colour::Red.paint(error));
                                            std::process::exit(1);
                                        }
                                        PactBrokerError::ContentError(error) => {
                                            println!("❌ {}", Colour::Red.paint(error));
                                            std::process::exit(1);
                                        }
                                        PactBrokerError::IoError(error) => {
                                            println!("❌ {}", Colour::Red.paint(error));
                                            std::process::exit(1);
                                        }
                                        PactBrokerError::NotFound(error) => {
                                            println!("❌ {}", Colour::Red.paint(error));
                                            std::process::exit(1);
                                        }
                                        PactBrokerError::ValidationError(errors) => {
                                            for error in errors {
                                                println!("❌ {}", Colour::Red.paint(error));
                                            }
                                            std::process::exit(1);
                                        }
                                        _ => {
                                            println!("❌ {}", Colour::Red.paint(err.to_string()));
                                            std::process::exit(1);
                                        }
                                    }
                                }
                            }
                        })
                        }
                        Some(("describe-environment", args)) => {
                            let uuid = args.get_one::<String>("uuid").unwrap().to_string();
                            let broker_url = get_broker_url(args);
                            let auth = get_auth(args);
                            tokio::runtime::Runtime::new().unwrap().block_on(async {
                                let hal_client: HALClient =
                                    HALClient::with_url(&broker_url, Some(auth.clone()));
                                let res = hal_client
                                    .fetch(&(broker_url + "/environments/" + &uuid))
                                    .await;

                                let default_output = "text".to_string();
                                let output =
                                    args.get_one::<String>("output").unwrap_or(&default_output);
                                match res {
                                    Ok(res) => {
                                        if output == "pretty" {
                                            let json = serde_json::to_string_pretty(&res).unwrap();
                                            println!("{}", json);
                                        } else if output == "json" {
                                            println!("{}", serde_json::to_string(&res).unwrap());
                                        } else {
                                            let res_uuid = res["uuid"].to_string();
                                            let res_name = res["name"].to_string();
                                            let res_display_name = res["displayName"].to_string();
                                            let res_production = res["production"].to_string();
                                            let res_created_at = res["createdAt"].to_string();
                                            let res_contacts = res["contacts"].as_array();

                                            println!("✅");
                                            println!(
                                                "UUID {}",
                                                Colour::Green.paint(res_uuid.trim_matches('"'))
                                            );
                                            println!(
                                                "Name: {}",
                                                Colour::Green.paint(res_name.trim_matches('"'))
                                            );
                                            println!(
                                                "Display Name: {}",
                                                Colour::Green
                                                    .paint(res_display_name.trim_matches('"'))
                                            );
                                            println!(
                                                "Production: {}",
                                                Colour::Green
                                                    .paint(res_production.trim_matches('"'))
                                            );
                                            println!(
                                                "Created At: {}",
                                                Colour::Green
                                                    .paint(res_created_at.trim_matches('"'))
                                            );
                                            if let Some(contacts) = res_contacts {
                                                println!("Contacts:");
                                                for contact in contacts {
                                                    println!(" - Contact:");
                                                    if let Some(name) = contact["name"].as_str() {
                                                        println!("  - Name: {}", name);
                                                    }
                                                    if let Some(email) = contact["email"].as_str() {
                                                        println!("  - Email: {}", email);
                                                    }
                                                }
                                            }
                                        }

                                        std::process::exit(0);
                                    }
                                    Err(err) => {
                                        match err {
                                            // TODO process output based on user selection
                                            PactBrokerError::LinkError(error) => {
                                                println!("❌ {}", Colour::Red.paint(error));
                                                std::process::exit(1);
                                            }
                                            PactBrokerError::ContentError(error) => {
                                                println!("❌ {}", Colour::Red.paint(error));
                                                std::process::exit(1);
                                            }
                                            PactBrokerError::IoError(error) => {
                                                println!("❌ {}", Colour::Red.paint(error));
                                                std::process::exit(1);
                                            }
                                            PactBrokerError::NotFound(error) => {
                                                println!("❌ {}", Colour::Red.paint(error));
                                                std::process::exit(1);
                                            }
                                            PactBrokerError::ValidationError(errors) => {
                                                for error in errors {
                                                    println!("❌ {}", Colour::Red.paint(error));
                                                }
                                                std::process::exit(1);
                                            }
                                            _ => {
                                                println!(
                                                    "❌ {}",
                                                    Colour::Red.paint(err.to_string())
                                                );
                                                std::process::exit(1);
                                            }
                                        }
                                    }
                                }
                            })
                        }
                        Some(("delete-environment", args)) => {
                            let uuid = args.get_one::<String>("uuid").unwrap().to_string();
                            let broker_url = get_broker_url(args);
                            let auth = get_auth(args);
                            tokio::runtime::Runtime::new().unwrap().block_on(async {
                                let hal_client: HALClient =
                                    HALClient::with_url(&broker_url, Some(auth.clone()));
                            let res = hal_client.clone().fetch(&(broker_url.clone() + "/environments/" + &uuid)).await;
                            match res {
                                Ok(_) => {
                                    let name = res.clone().unwrap()["name"].to_string();
                                    let res = hal_client.clone().delete(&(broker_url.clone() + "/environments/" + &uuid)).await;
                                    match res {
                                        Ok(_) => {
                                            println!("✅ Deleted environment {} from the Pact Broker with UUID {}", Colour::Green.paint(name), Colour::Green.paint(uuid.trim_matches('"')));
                                            std::process::exit(0);
                                        }
                                        Err(err) => {
                                            match err {
                                                PactBrokerError::LinkError(error) => {
                                                    println!("❌ {}", Colour::Red.paint(error));
                                                    std::process::exit(1);
                                                }
                                                PactBrokerError::ContentError(error) => {
                                                    println!("❌ {}", Colour::Red.paint(error));
                                                    std::process::exit(1);
                                                }
                                                PactBrokerError::IoError(error) => {
                                                    println!("❌ {}", Colour::Red.paint(error));
                                                    std::process::exit(1);
                                                }
                                                PactBrokerError::NotFound(error) => {
                                                    println!("❌ {}", Colour::Red.paint(error));
                                                    std::process::exit(1);
                                                }
                                                PactBrokerError::ValidationError(errors) => {
                                                    for error in errors {
                                                        println!("❌ {}", Colour::Red.paint(error));
                                                    }
                                                    std::process::exit(1);
                                                }
                                                _ => {
                                                    println!("❌ {}", Colour::Red.paint(err.to_string()));
                                                    std::process::exit(1);
                                                }
                                            }
                                        }
                                    }
                                }
                                Err(err) => {
                                    match err {
                                        PactBrokerError::LinkError(error) => {
                                            println!("❌ {}", Colour::Red.paint(error));
                                            std::process::exit(1);
                                        }
                                        PactBrokerError::ContentError(error) => {
                                            println!("❌ {}", Colour::Red.paint(error));
                                            std::process::exit(1);
                                        }
                                        PactBrokerError::IoError(error) => {
                                            println!("❌ {}", Colour::Red.paint(error));
                                            std::process::exit(1);
                                        }
                                        PactBrokerError::NotFound(error) => {
                                            println!("❌ {}", Colour::Red.paint(error));
                                            std::process::exit(1);
                                        }
                                        PactBrokerError::ValidationError(errors) => {
                                            for error in errors {
                                                println!("❌ {}", Colour::Red.paint(error));
                                            }
                                            std::process::exit(1);
                                        }
                                        _ => {
                                            println!("❌ {}", Colour::Red.paint(err.to_string()));
                                            std::process::exit(1);
                                        }
                                    }
                                }
                            }

                        })
                        }
                        Some(("list-environments", args)) => {
                            let broker_url = get_broker_url(args);
                            let auth = get_auth(args);
                            tokio::runtime::Runtime::new().unwrap().block_on(async {
                                let hal_client: HALClient =
                                    HALClient::with_url(&broker_url, Some(auth.clone()));
                                let res = hal_client.fetch(&(broker_url + "/environments/")).await;

                                let default_output = "text".to_string();
                                let output =
                                    args.get_one::<String>("output").unwrap_or(&default_output);
                                match res {
                                    Ok(res) => {
                                        if output == "pretty" {
                                            let json = serde_json::to_string_pretty(&res).unwrap();
                                            println!("{}", json);
                                        } else if output == "json" {
                                            println!("{}", serde_json::to_string(&res).unwrap());
                                        } else {
                                            let mut table = Table::new();

                                            #[derive(Debug, serde::Deserialize)]
                                            struct Environment {
                                                uuid: String,
                                                name: String,
                                                displayName: String,
                                                production: bool,
                                                createdAt: String,
                                            }

                                            table.load_preset(UTF8_FULL).set_header(vec![
                                                "UUID",
                                                "NAME",
                                                "DISPLAY NAME",
                                                "PRODUCTION",
                                                "CREATED AT",
                                            ]);

                                            if let Some(embedded) = res["_embedded"].as_object() {
                                                if let Some(environments) =
                                                    embedded["environments"].as_array()
                                                {
                                                    for environment in environments {
                                                        let environment: Environment =
                                                            serde_json::from_value(
                                                                environment.clone(),
                                                            )
                                                            .unwrap();
                                                        table.add_row(vec![
                                                            environment.uuid,
                                                            environment.name,
                                                            environment.displayName,
                                                            environment.production.to_string(),
                                                            environment.createdAt,
                                                        ]);
                                                    }
                                                }
                                            }

                                            println!("{table}");
                                        }

                                        std::process::exit(0);
                                    }
                                    Err(err) => {
                                        match err {
                                            // TODO process output based on user selection
                                            PactBrokerError::LinkError(error) => {
                                                println!("❌ {}", Colour::Red.paint(error));
                                                std::process::exit(1);
                                            }
                                            PactBrokerError::ContentError(error) => {
                                                println!("❌ {}", Colour::Red.paint(error));
                                                std::process::exit(1);
                                            }
                                            PactBrokerError::IoError(error) => {
                                                println!("❌ {}", Colour::Red.paint(error));
                                                std::process::exit(1);
                                            }
                                            PactBrokerError::NotFound(error) => {
                                                println!("❌ {}", Colour::Red.paint(error));
                                                std::process::exit(1);
                                            }
                                            PactBrokerError::ValidationError(errors) => {
                                                for error in errors {
                                                    println!("❌ {}", Colour::Red.paint(error));
                                                }
                                                std::process::exit(1);
                                            }
                                            _ => {
                                                println!(
                                                    "❌ {}",
                                                    Colour::Red.paint(err.to_string())
                                                );
                                                std::process::exit(1);
                                            }
                                        }
                                    }
                                }
                            })
                        }
                        Some(("record-deployment", args)) => {
                            // Handle record-deployment command
                            // Ok(());
                        }
                        Some(("record-undeployment", args)) => {
                            // Handle record-undeployment command
                            // Ok(());
                        }
                        Some(("record-release", args)) => {
                            // Handle record-release command
                            // Ok(());
                        }
                        Some(("record-support-ended", args)) => {
                            // Handle record-support-ended command
                            // Ok(());
                        }
                        Some(("can-i-deploy", args)) => {
                            // TODO
                            // Query strings
                            // Async runtime

                            // Handle can-i-deploy command
                            // setup client with broker ucarl and credentials
                            let broker_url = get_broker_url(args);
                            let auth = get_auth(args);
                            // query pact broker index and get hal relation link
                            tokio::runtime::Runtime::new().unwrap().block_on(async {
                                let hal_client: HALClient =
                                    HALClient::with_url(&broker_url, Some(auth.clone()));
                                let matrix_href_path = "/matrix?pacticipant=Example+App&latest=true&latestby=cvp&latest=true".to_string();
                                // let matrix_href_path = "/matrix?q[][pacticipant]=Example+App&q[][latest]=true&latestby=cvp&latest=true".to_string();
                                // query the hal relation link to get the latest pact versions
                                let res = follow_broker_relation(
                                    hal_client.clone(),
                                    "pb:latest-pact-versions".to_string(),
                                    matrix_href_path,
                                )
                                .await;
                                match res {
                                    Ok(res) => {
                                        // handle user args for additional processing
                                        let output: Result<Option<&String>, clap::parser::MatchesError> =
                                            args.try_get_one::<String>("output");

                                        // render result
                                        match output {
                                            Ok(Some(output)) => {
                                                if output == "json" {
                                                    let json: String =
                                                        serde_json::to_string(&res.clone()).unwrap();
                                                    println!("{}", json);
                                                } else if output == "table" {
                                                    generate_table(
                                                        &res,
                                                        vec![
                                                            "CONSUMER",
                                                            "CONSUMER_VERSION",
                                                            "PROVIDER",
                                                            "CREATED_AT",
                                                        ],
                                                        vec![
                                                            vec!["_embedded", "consumer", "name"],
                                                            vec![
                                                                "_embedded",
                                                                "consumer",
                                                                "_embedded",
                                                                "version",
                                                                "number",
                                                            ],
                                                            vec!["_embedded", "provider", "name"],
                                                            vec!["createdAt"],
                                                        ],
                                                    );
                                                }
                                            }
                                            Ok(None) => {
                                                println!("{:?}", res.clone());
                                            }
                                            Err(res) => {
                                                println!("{:?}", res);
                                                // os.exit(1)
                                            }
                                        }
                                    }
                                    Err(res) => {
                                        println!("{:?}", res);
                                        // os.exit(1)
                                    }
                                }
                        })
                        }
                        Some(("can-i-merge", args)) => {
                            // Handle can-i-merge command
                            // Ok(());
                        }
                        Some(("create-or-update-pacticipant", args)) => {
                            // Handle create-or-update-pacticipant command
                            // Ok(());
                        }
                        Some(("describe-pacticipant", args)) => {
                            // Handle describe-pacticipants command
                            // Ok(());
                        }
                        Some(("list-pacticipants", args)) => {
                            // Handle list-pacticipants command
                            // Ok(());
                        }
                        Some(("create-webhook", args)) => {
                            // Handle create-webhook command
                            // Ok(());
                        }
                        Some(("create-or-update-webhook", args)) => {
                            // Handle create-or-update-webhook command
                            // Ok(());
                        }
                        Some(("test-webhook", args)) => {
                            // Handle test-webhook command

                            // Ok(());
                        }
                        Some(("delete-branch", args)) => {
                            // Handle delete-branch command
                            // Ok(());
                        }
                        Some(("create-version-tag", args)) => {
                            // Handle create-version-tag command
                            // Ok(());
                        }
                        Some(("describe-version", args)) => {
                            // Handle describe-version command
                            // Ok(());
                        }
                        Some(("create-or-update-version", args)) => {
                            // Handle create-or-update-version command
                            // Ok(());
                        }
                        Some(("generate-uuid", args)) => {
                            println!("{}", uuid::Uuid::new_v4());
                        }
                        _ => {
                            println!("⚠️  No option provided, try running pact-broker --help");
                        }
                    }
                }
                Some(("pactflow", args)) => {
                    match args.subcommand() {
                        Some(("publish-provider-contract", args)) => {
                            println!("{:?}", args);

                            // Ok(());
                        }
                        _ => {
                            println!("⚠️  No option provided, try running pactflow --help");

                            // Ok(());
                        }
                    }
                }
                Some(("completions", args)) => {
                    let mut cmd = cli::build_cli();
                    let shell: String = args
                        .get_one::<String>("shell")
                        .expect("a shell is required")
                        .to_string();
                    let out_dir: String = args
                        .get_one::<String>("dir")
                        .expect("a directory is expected")
                        .to_string();
                    let shell_enum = Shell::from_str(&shell).unwrap();
                    let _ = generate_to(shell_enum, &mut cmd, "pact_cli".to_string(), &out_dir);
                    println!(
                        "ℹ️  {} shell completions for pact_cli written to {}",
                        &shell_enum, &out_dir
                    );

                    // Ok(());
                }
                Some(("standalone", args)) => {
                    tokio::runtime::Runtime::new().unwrap().block_on(async {
                        let os = match env::consts::OS {
                            "macos" => "osx",
                            other => other,
                        };

                        let arch = match env::consts::ARCH {
                            "aarch64" => "arm64",
                            other => other,
                        };

                        // check if os/arch is supported
                        // supported are osx, linux and arm64, x86_64
                        if os != "osx" && os != "linux" {
                            println!("⚠️  Unsupported OS: {}", os);
                            std::process::exit(1);
                        }
                        if arch != "arm64" && arch != "x86_64" {
                            println!("⚠️  Unsupported architecture: {}", arch);
                            std::process::exit(1);
                        }

                        // Store the binary in the user's home .pact/traveling-broker directory
                        let home_dir = env::var("HOME").unwrap();
                        let pact_dir = format!("{}/.pact/traveling-broker", home_dir);
                        let _ = fs::create_dir_all(&pact_dir);
                        let broker_archive_path = format!("{}/traveling-pact-20230803-3.2.2-{}-{}-full.tar.gz", pact_dir, os, arch);
                        let app_path = format!("{}/pact-broker-app.sh", pact_dir);

                        // check is app path exists, if so, do not download the file

                        if !fs::metadata(&app_path).is_ok() {
                            // Download the correct version of the traveling ruby binary
                            let url = format!(
                                "https://github.com/YOU54F/traveling-ruby/releases/download/rel-20230803-pact/traveling-pact-20230803-3.2.2-{}-{}-full.tar.gz",
                                os, arch
                            );
                            let response = reqwest::get(&url).await.unwrap();
                            let body = response.bytes().await.unwrap();

                            let mut file = fs::File::create(&broker_archive_path).unwrap();
                            let _ = file.write_all(&body);

                            // Unpack the binary
                            Command::new("tar")
                                .arg("-xf")
                                .arg(&broker_archive_path)
                                .arg("-C")
                                .arg(&pact_dir)
                                .output()
                                .expect("Failed to unpack the binary");
                            let _ = fs::remove_file(broker_archive_path);
                    }
                        // Execute the pact-broker-app.sh file

                        let mut child = Command::new(&app_path).spawn().unwrap();

                        // Await SIGKILL from the user
                        let _ = tokio::signal::ctrl_c().await;

                        // Send SIGKILL to the app
                        let _ = child.kill();

                        Ok::<(), ()>(());

                    })
                }
                Some(("examples", args)) => {
                    let project_type = args.get_one::<String>("type").unwrap().as_str();
                    let project = &args
                        .get_one::<String>("project")
                        .map(|project| project.to_string());
                    let download_all = args.get_flag("all");

                    match project_type {
                        "bdct" => {
                            let projects = vec![
                                "example-bi-directional-consumer-cypress",
                                "example-bi-directional-provider-postman",
                                "example-bi-directional-consumer-msw",
                                "example-bi-directional-provider-dredd",
                                "example-bi-directional-provider-restassured",
                                "example-bi-directional-consumer-wiremock",
                                "example-bi-directional-consumer-nock",
                                "example-bi-directional-consumer-mountebank",
                                "example-bi-directional-consumer-dotnet",
                                "example-bi-directional-provider-dotnet",
                            ];

                            if download_all {
                                for project in projects {
                                    download_project(project);
                                }
                            } else if let Some(project) = project {
                                download_project(project);
                            } else {
                                println!("Please specify a project to download");
                                for project in projects {
                                    println!("{}", project);
                                }
                            }
                        }
                        "cdct" => {
                            let projects = vec![
                                "example-siren",
                                "example-provider",
                                "example-consumer",
                                "example-consumer-js-kafka",
                                "example-consumer-cypress",
                                "example-consumer-python",
                                "example-consumer-golang",
                                "example-consumer-java-kafka",
                                "example-consumer-java-junit",
                                "example-consumer-java-soap",
                                "example-consumer-dotnet",
                                "example-provider-golang",
                                "example-provider-springboot",
                                "example-provider-java-soap",
                                "example-provider-java-kafka",
                                "example-consumer-js-sns",
                                "example-provider-js-sns",
                                "example-provider-python",
                                "example-consumer-webhookless",
                                "example-provider-dotnet",
                                "pactflow-jsonschema-example",
                                "provider-driven-example",
                                "injected-provider-states-example",
                            ];

                            if download_all {
                                for project in projects {
                                    download_project(project);
                                }
                            } else if let Some(project) = project {
                                download_project(project);
                            } else {
                                println!("Please specify a project to download");
                                for project in projects {
                                    println!("{}", project);
                                }
                            }
                        }
                        "workshops" => {
                            let projects = vec![
                                "pact-workshop-js",
                                "pact-workshop-jvm-spring",
                                "pact-workshop-dotnet-core-v1",
                                "pact-workshop-Maven-Springboot-JUnit5",
                                "pact-workshop-go",
                            ];
                            let org = "pact-foundation";

                            if download_all {
                                for project in projects {
                                    download_project_with_org(org, project);
                                }
                            } else if let Some(project) = project {
                                download_project_with_org(org, project);
                            } else {
                                println!("Please specify a project to download");
                                for project in projects {
                                    println!("{}", project);
                                }
                            }
                        }
                        _ => {
                            println!("Sorry, you'll need to specify a valid option (bdct, cdct, workshops)");
                        }
                    }

                    fn download_project(project: &str) {
                        println!("Downloading {}", project);
                        // Implement the logic to download the project here
                        println!("Downloaded {}", project);
                    }

                    fn download_project_with_org(org: &str, project: &str) {
                        println!("Downloading project {}", project);
                        // Implement the logic to download the project with the specified organization here
                        println!("Downloaded project {}", project);
                    }
                }
                Some(("project", args)) => {
                    let language = args.get_one::<String>("language").unwrap().as_str();
                    match args.subcommand() {
                        Some(("install", args)) => {
                            match language {
                                "js" => {
                                    println!("To install Pact-JS, run the following command:");
                                    println!("`npm install @pact-foundation/pact`");
                                }
                                "golang" => {
                                    println!("To install Pact-Go, run the following command:");
                                    println!(
                                        "`go get github.com/pact-foundation/pact-go/v2@2.x.x`"
                                    );
                                    println!("# NOTE: If using Go 1.19 or later, you need to run go install instead");
                                    println!(
                                        "# go install github.com/pact-foundation/pact-go/v2@2.x.x"
                                    );
                                    println!("# download and install the required libraries. The pact-go will be installed into $GOPATH/bin, which is $HOME/go/bin by default.");
                                    println!("pact-go -l DEBUG install");
                                    println!("# 🚀 now write some tests!");
                                }
                                "ruby" => {
                                    println!("To install Pact-Ruby, run the following command:");
                                    println!("Add this line to your application's Gemfile:");
                                    println!("gem 'pact'");
                                    println!("# gem 'pact-consumer-minitest' for minitest");
                                    println!("And then execute:");
                                    println!("$ bundle");
                                    println!("Or install it yourself as:");
                                    println!("$ gem install pact");
                                }
                                "python" => {
                                    println!("To install Pact-Python, run the following command:");
                                    println!("`pip install pact-python`");
                                }
                                "java" => {
                                    println!("To install Pact-JVM, add the following dependency to your build file:");
                                    println!("`testImplementation 'au.com.dius.pact.consumer:junit5:4.6.5'`");
                                    println!("`testImplementation 'au.com.dius.pact.provider:junit5:4.6.5'`");
                                }
                                ".net" => {
                                    println!("To install Pact-.NET, add the following package to your project:");
                                    println!("`dotnet add package PactNet --version 4.5.0`");
                                }
                                "rust" => {
                                    println!("To install Pact-Rust, add the following dependency to your Cargo.toml file:");
                                    println!("`pact_consumer = \"0.0.1\"`");
                                    println!("`pact_verifier = \"0.0.1\"`");
                                    println!("`pact_models = \"0.0.1\"`");
                                    println!("`pact_matching = \"0.0.1\"`");
                                }
                                "php" => {
                                    println!("To install Pact-PHP, add the following dependency to your composer.json file:");
                                    println!("`\"pact-foundation/pact-php\": \"^9.0\"`");
                                    println!("To try out Pact-PHP build with the pact rust core:");
                                    println!("`\"pact-foundation/pact-php\": \"^10.0.0-alpha6\"`");
                                }
                                _ => {
                                    println!("⚠️  Invalid option provided");
                                    // Ok(());
                                }
                            }
                        }
                        Some(("new", args)) => {
                            // Handle delete command
                            // Ok(());
                        }
                        Some(("link", args)) => {
                            // Handle list command
                            // Ok(());
                        }
                        Some(("issue", args)) => {
                            // Handle update command
                            // Ok(());
                        }
                        Some(("docs", args)) => {
                            // Handle update command
                            // Ok(());
                        }
                        _ => {
                            println!("⚠️  No option provided, try running project --help");

                            // Ok(());
                        }
                    }
                }
                Some(("docker", args)) => {
                    match args.subcommand() {
                        Some(("start", args)) => {
                            let output = Command::new("docker")
                        .arg("run")
                        .arg("-d")
                        .arg("--name")
                        .arg("pact-broker")
                        .arg("-p")
                        .arg("9292:9292")
                        .arg("--env")
                        .arg("PACT_BROKER_PORT=9292")
                        .arg("--env")
                        .arg("PACT_BROKER_DATABASE_URL=sqlite:////tmp/pact_broker.sqlite")
                        .arg("--env")
                        .arg("PACT_BROKER_BASE_URL=http://localhost http://localhost http://localhost:9292 http://pact-broker:9292 https://host.docker.internal http://host.docker.internal http://host.docker.internal:9292")
                        .arg("pactfoundation/pact-broker:latest")
                        .output()
                        .expect("Failed to execute Docker command");

                            if output.status.success() {
                                println!("Docker container started successfully");
                            } else {
                                let error_message = String::from_utf8_lossy(&output.stderr);
                                println!("Failed to start Docker container: {}", error_message);
                            }
                        }
                        Some(("stop", args)) => {
                            let output = Command::new("docker")
                                .arg("stop")
                                .arg("pact-broker")
                                .output()
                                .expect("Failed to execute Docker command");

                            if output.status.success() {
                                println!("Docker container stopped successfully");
                            } else {
                                let error_message = String::from_utf8_lossy(&output.stderr);
                                println!("Failed to stop Docker container: {}", error_message);
                            }
                        }
                        Some(("remove", args)) => {
                            let output = Command::new("docker")
                                .arg("rm")
                                .arg("pact-broker")
                                .output()
                                .expect("Failed to execute Docker command");

                            if output.status.success() {
                                println!("Docker container removed successfully");
                            } else {
                                let error_message = String::from_utf8_lossy(&output.stderr);
                                println!("Failed to remove Docker container: {}", error_message);
                            }
                        }
                        _ => {
                            println!("⚠️  No option provided, try running pactflow --help");

                            // Ok(());
                        }
                    }
                }
                Some(("plugin", args)) => {
                    let _ = pact_plugin_cli::main::run(args);
                }
                Some(("mock", args)) => tokio::runtime::Runtime::new().unwrap().block_on(async {
                    let res = pact_mock_server_cli::main::handle_matches(args).await;
                    match res {
                        Ok(_) => {
                            std::process::exit(0);
                        }
                        Err(e) => {
                            std::process::exit(e);
                        }
                        _ => {
                            std::process::exit(1);
                        }
                    }
                }),
                Some(("stub", args)) => tokio::runtime::Runtime::new().unwrap().block_on(async {
                    let res = pact_stub_server_cli::main::handle_matches(args).await;
                    match res {
                        Ok(_) => {
                            std::process::exit(0);
                        }
                        Err(e) => {
                            println!("Error: {:?}", e);
                            std::process::exit(3);
                        }
                        _ => {
                            std::process::exit(1);
                        }
                    }
                }),
                Some(("verifier", args)) => {
                    tokio::runtime::Runtime::new().unwrap().block_on(async {
                        let res = pact_verifier_cli::main::handle_matches(args).await;
                        match res {
                            Ok(_) => {
                                std::process::exit(0);
                            }
                            Err(e) => {
                                std::process::exit(e);
                            }
                        }
                    });
                }
                _ => {
                    cli::build_cli().print_help().unwrap();

                    // Ok(());
                }
            }
        }
        Err(ref err) => match err.kind() {
            ErrorKind::DisplayHelp => {
                let _ = err.print();
            }
            ErrorKind::DisplayVersion => {
                let error_message = err.render().to_string();
                let mock_server_match = "pact_cli-mock \n".to_string();
                let verifier_match = "pact_cli-verifier \n".to_string();
                let stub_server_match = "pact_cli-stub \n".to_string();
                if verifier_match == error_message {
                    pact_verifier_cli::main::print_version(&verifier_match);
                    println!();
                } else if mock_server_match == error_message {
                    pact_mock_server_cli::main::print_version();
                    println!();
                } else if stub_server_match == error_message {
                    pact_stub_server_cli::main::print_version();
                    println!();
                }
            }
            _ => err.exit(),
        },
    }
}
