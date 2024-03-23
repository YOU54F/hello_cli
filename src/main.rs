// use std::collections::HashMap;
mod cli;
use clap::error::ErrorKind;
use clap_complete::{generate_to, Shell};
use pact_broker::list_latest_pact_versions::list_latest_pact_versions;
use pact_broker::types::{BrokerDetails, OutputType};
use pact_broker::{HALClient, PactBrokerError};
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use serde_json::Value;
use std::env;
use std::fs;
use std::io::{Read, Write};
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
use pact_models::http_utils::HttpAuth;

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

// Helper function to handle errors
fn handle_error(err: PactBrokerError) {
    match err {
        PactBrokerError::LinkError(error)
        | PactBrokerError::ContentError(error)
        | PactBrokerError::IoError(error)
        | PactBrokerError::NotFound(error) => {
            println!("❌ {}", Colour::Red.paint(error));
        }
        PactBrokerError::ValidationError(errors) => {
            for error in errors {
                println!("❌ {}", Colour::Red.paint(error));
            }
        }
        _ => {
            println!("❌ {}", Colour::Red.paint(err.to_string()));
        }
    }
    std::process::exit(1);
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
                            println!("Unimplemented");
                            std::process::exit(1);
                        }
                        Some(("list-latest-pact-versions", args)) => {
                            // Handle list-latest-pact-versions command

                            // setup client with broker url and credentials
                            let broker_url = get_broker_url(args);
                            let auth = get_auth(args);
                            let broker_details = BrokerDetails {
                                url: broker_url.clone(),
                                auth: Some(auth),
                            };
                            let default_output: String = "text".to_string();
                            let output_arg: &String =
                                args.get_one::<String>("output").unwrap_or(&default_output);
                            let output = match output_arg.as_str() {
                                "json" => OutputType::Json,
                                "table" => OutputType::Table,
                                "pretty" => OutputType::Pretty,
                                _ => OutputType::Text,
                            };

                            let verbose = args.get_flag("verbose");
                            let _ = list_latest_pact_versions(&broker_details, output, verbose);
                            // tokio::runtime::Runtime::new().unwrap().block_on(async {
                            //     // query pact broker index and get hal relation link
                            //     let hal_client: HALClient =
                            //         HALClient::with_url(&broker_url, Some(auth.clone()));
                            //     let pb_latest_pact_versions_href_path = get_broker_relation(
                            //         hal_client.clone(),
                            //         "pb:latest-pact-versions".to_string(),
                            //         broker_url,
                            //     )
                            //     .await;
                            //     // query the hal relation link to get the latest pact versions
                            //     let res = follow_broker_relation(
                            //         hal_client.clone(),
                            //         "pb:latest-pact-versions".to_string(),
                            //         pb_latest_pact_versions_href_path,
                            //     )
                            //     .await;

                            //     // handle user args for additional processing
                            //     let output: Result<Option<&String>, clap::parser::MatchesError> =
                            //         args.try_get_one::<String>("output");
                            //     // render result
                            //     match output {
                            //         Ok(Some(output)) => {
                            //             if output == "json" {
                            //                 let json: String =
                            //                     serde_json::to_string(&res.unwrap()).unwrap();
                            //                 println!("{}", json);
                            //             } else if output == "table" {
                            //                 if let Ok(res) = res {
                            //                     pact_broker::utils::generate_table(
                            //                         &res,
                            //                         vec![
                            //                             "CONSUMER",
                            //                             "CONSUMER_VERSION",
                            //                             "PROVIDER",
                            //                             "CREATED_AT",
                            //                         ],
                            //                         vec![
                            //                             vec!["_embedded", "consumer", "name"],
                            //                             vec![
                            //                                 "_embedded",
                            //                                 "consumer",
                            //                                 "_embedded",
                            //                                 "version",
                            //                                 "number",
                            //                             ],
                            //                             vec!["_embedded", "provider", "name"],
                            //                             vec!["createdAt"],
                            //                         ],
                            //                     );
                            //                 }
                            //             }
                            //         }
                            //         Ok(None) => {
                            //             println!("{:?}", res.clone());
                            //         }
                            //         Err(_) => todo!(),
                            //     }
                            // });
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

                            let default_output: String = "text".to_string();
                            let output = args.get_one::<String>("output").unwrap_or(&default_output);
                            match res {
                                Ok(res) => {
                                        if output == "pretty" {
                                            let json = serde_json::to_string_pretty(&res).unwrap();
                                            println!("{}", json);
                                        } else if output == "json" {
                                            println!("{}", serde_json::to_string(&res).unwrap());
                                        } else if output == "id" {
                                            println!("{}", res["uuid"].to_string().trim_matches('"'));
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
                                            let table = pact_broker::utils::generate_table(
                                                &res,
                                                columns,
                                                names,
                                            );
                                            println!("{table}");
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
                            let version = args.get_one::<String>("version");
                            let pacticipant = args.get_one::<String>("pacticipant");
                            let environment = args.get_one::<String>("environment");
                            let application_instance =
                                args.get_one::<String>("application-instance");
                            let broker_url = get_broker_url(args);
                            let auth = get_auth(args);
                            tokio::runtime::Runtime::new().unwrap().block_on(async {
                                let hal_client: HALClient =
                                    HALClient::with_url(&broker_url, Some(auth.clone()));

                            let res = hal_client.clone()
                                .fetch(
                                    &(broker_url.clone()
                                        + "/pacticipants/"
                                        + &pacticipant.unwrap()
                                        + "/versions/"
                                        + &version.unwrap()),
                                )
                                .await;

                            #[derive(Debug, Deserialize, Serialize)]
                            struct PacticipantVersions {
                                _embedded: Embedded,
                                _links: Links,
                                createdAt: String,
                                number: String,
                            }

                            #[derive(Debug, Deserialize, Serialize)]
                            struct Links {
                                #[serde(rename = "self")]
                                self_link: Link,
                                #[serde(rename = "pb:pacticipant")]
                                pacticipant_link: Link,
                                #[serde(rename = "pb:tag")]
                                tag_link: Link,
                                #[serde(rename = "pb:latest-verification-results-where-pacticipant-is-consumer")]
                                latest_verification_results_link: Link,
                                #[serde(rename = "pb:pact-versions")]
                                pact_versions: Vec<Link>,
                                #[serde(rename = "pb:record-deployment")]
                                record_deployment: Vec<Link>,
                                #[serde(rename = "pb:record-release")]
                                record_release: Vec<Link>,
                                curies: Vec<Curies>,
                            }

                            #[derive(Debug, Deserialize, Serialize)]
                            struct Link {
                                href: String,
                                name: Option<String>,
                                title: Option<String>,
                                templated: Option<bool>,
                            }

                            #[derive(Debug, Deserialize, Serialize)]
                            struct Curies {
                                name: String,
                                href: String,
                                templated: bool
                            }

                            #[derive(Debug, Deserialize, Serialize)]
                            struct Embedded {
                                branchVersions: Vec<BranchVersion>,
                                tags: Vec<Tag>,
                            }

                            #[derive(Debug, Deserialize, Serialize)]
                            struct BranchVersion {
                                _links: VersionLinks,
                                latest: bool,
                                name: String,
                            }

                            #[derive(Debug, Deserialize, Serialize)]
                            struct Tag {
                                _links: TagLinks,
                                name: String,
                            }

                            #[derive(Debug, Deserialize, Serialize)]
                            struct VersionLinks {
                                #[serde(rename = "self")]
                                self_link: Link,
                                name: Option<String>,
                                title: Option<String>,
                            }

                            #[derive(Debug, Deserialize, Serialize)]
                            struct TagLinks {
                                #[serde(rename = "self")]
                                self_link: Link,
                                name: Option<String>,
                                title: Option<String>,
                            }
                            match res {
                                Ok(res) => {

                                    let result: Result<PacticipantVersions, serde_json::Error> = serde_json::from_value(res);
                                    match result {
                                        Ok(data) => {
                                        match data._links.record_deployment.iter().find(|x| x.name == Some(environment.unwrap().to_string())) {
                                            Some(link) => {
                                                let link_record_deployment_href = &link.href;

                                                // println!("✅ Found environment {} with {}", Colour::Green.paint(environment.unwrap()), Colour::Green.paint(link_record_deployment_href.clone()));

                                                // <- "POST /pacticipants/Example%20App/versions/5556b8149bf8bac76bc30f50a8a2dd4c22c85f30/deployed-versions/environment/c540ce64-5493-48c5-ab7c-28dae27b166b HTTP/1.1\r\nAccept: application/hal+json\r\nUser-Agent: Ruby\r\nContent-Type: application/json\r\nHost: localhost:9292\r\nContent-Length: 44\r\n\r\n"
                                                // <- "{\"applicationInstance\":\"foo\",\"target\":\"foo\"}"


                                                let mut payload = json!({});
                                                payload["target"] = serde_json::Value::String(environment.unwrap().to_string());
                                                if let Some(application_instance) = application_instance {
                                                    payload["applicationInstance"] = serde_json::Value::String(application_instance.to_string());
                                                }
                                                let res: Result<Value, PactBrokerError> = hal_client.clone().post_json(&(link_record_deployment_href.clone()), &payload.to_string()).await;
                                                let default_output = "text".to_string();
                                                let output = args.get_one::<String>("output").unwrap_or(&default_output);
                                                match res {
                                                    Ok(res) => {
                                                            if output == "pretty" {
                                                                let json = serde_json::to_string_pretty(&res).unwrap();
                                                                println!("{}", json);
                                                            } else if output == "json" {
                                                                println!("{}", serde_json::to_string(&res).unwrap());
                                                            } else if output == "id" {
                                                                println!("{}", res["uuid"].to_string().trim_matches('"'));
                                                            }
                                                            else {
                                                                // let uuid = res["uuid"].to_string();
                                                                println!("✅ Recorded deployment of {} version {} to {} environment{} in the Pact Broker.", Colour::Green.paint(pacticipant.unwrap()), Colour::Green.paint(version.unwrap()),Colour::Green.paint(environment.unwrap()), application_instance.map(|instance| format!(" (application instance {})", Colour::Green.paint(instance))).unwrap_or_default());
                                                                // println!("✅ Created {} environment in the Pact Broker with UUID {}", Colour::Green.paint(name.unwrap()), Colour::Green.paint(uuid.trim_matches('"')));

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
                                                        }
                                            None => {
                                                println!("❌ Environment {} does not exist", Colour::Red.paint(environment.unwrap()));
                                                std::process::exit(1);
                                            }}
                                        }
                                        Err(err) => {
                                            println!("Error: {}", err);
                                            println!("❌ {}", Colour::Red.paint("Failed to record deployment"));
                                            std::process::exit(1);
                                        }
                                    }
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
                        }})
                        }
                        Some(("record-undeployment", args)) => {
                            // 1. Check broker index link for connection
                            // <- "GET /? HTTP/1.1\r\nAccept: application/hal+json\r\nUser-Agent: Ruby\r\nHost: localhost:9292\r\n\r\n"
                            // -> "HTTP/1.1 200 OK\r\n"
                            // 2. Call environments and check the specified enviroment exists, get the environment link
                            // <- "GET /environments? HTTP/1.1\r\nAccept: application/hal+json\r\nUser-Agent: Ruby\r\nHost: localhost:9292\r\n\r\n"
                            // -> "HTTP/1.1 200 OK\r\n"
                            // 3. Call the environment link and check the specified version exists, get the version link
                            // <- "GET /environments/c540ce64-5493-48c5-ab7c-28dae27b166b? HTTP/1.1\r\nAccept: application/hal+json\r\nUser-Agent: Ruby\r\nHost: localhost:9292\r\n\r\n"
                            // -> "HTTP/1.1 200 OK\r\n"
                            // 4. Call the /environments/c540ce64-5493-48c5-ab7c-28dae27b166b/deployed-versions/currently-deployed?pacticipant=Example+App link, and check our app is currently deployed
                            // <- "GET /environments/c540ce64-5493-48c5-ab7c-28dae27b166b/deployed-versions/currently-deployed?pacticipant=Example+App HTTP/1.1\r\nAccept: application/hal+json\r\nUser-Agent: Ruby\r\nHost: localhost:9292\r\n\r\n"
                            // -> "HTTP/1.1 200 OK\r\n"
                            // 5. perform a patch request to the /environments/c540ce64-5493-48c5-ab7c-28dae27b166b/deployed-versions/9b756f93-19a2-4ca7-ae36-c0b917ac1f21 link to set currentlyDeployed to false
                            // <- "PATCH /deployed-versions/9b756f93-19a2-4ca7-ae36-c0b917ac1f21 HTTP/1.1\r\nAccept: application/hal+json\r\nUser-Agent: Ruby\r\nContent-Type: application/merge-patch+json\r\nHost: localhost:9292\r\nContent-Length: 27\r\n\r\n"
                            // <- "{\"currentlyDeployed\":false}"

                            let pacticipant = args.get_one::<String>("pacticipant");
                            let environment = args.get_one::<String>("environment");
                            let application_instance =
                                args.get_one::<String>("application-instance");
                            let broker_url = get_broker_url(args);
                            let auth = get_auth(args);
                            tokio::runtime::Runtime::new().unwrap().block_on(async {
                            let hal_client: HALClient = HALClient::with_url(&broker_url, Some(auth.clone()));


                            let res = hal_client.clone()
                                .fetch(&(broker_url.clone() + "/"))
                                .await;
                            match res {
                                Ok(_) => {
                                    // Handle success
                                }
                                Err(err) => {
                                    handle_error(err);
                                }
                            }

                            #[derive(Debug, serde::Deserialize)]
                            struct Environment {
                                uuid: String,
                                name: String,
                                displayName: String,
                                production: bool,
                                createdAt: String,
                            }

                            let res = hal_client.clone()
                                .fetch(&(broker_url.clone() + "/environments?"))
                                .await;
                                match res {
                                    Ok(response) => {
                                        let environments: Vec<Environment> = response["_embedded"]["environments"]
                                            .as_array()
                                            .unwrap()
                                            .iter()
                                            .map(|env| serde_json::from_value(env.clone()).unwrap())
                                            .collect();
                                        let environment_exists = environments.iter().any(|env| env.name == environment.clone().unwrap().to_string());
                                        if environment_exists {
                                            let environment_uuid = &environments.iter().find(|env| env.name == environment.clone().unwrap().to_string()).unwrap().uuid;
                                            // Use environment_uuid in step 3

                                            // 3. Call the environment link and check the specified version exists, get the version link
                                            let res = hal_client.clone()
                                            .fetch(&(broker_url.clone() + "/environments/" + &environment_uuid + "?"))
                                            .await;
                                        match res {
                                            Ok(result) => {
                                                // print!("✅ Environment found");
                                                // print!("🧹 Undeploying {} from {} environment", pacticipant.unwrap(), environment.unwrap());
                                                // print!("Result JSON: {:#?}", result);
                                                // todo - handle application instance

                                                let currently_deployed_link = result["_links"]["pb:currently-deployed-deployed-versions"]["href"].as_str().unwrap();
                                                let pacticipant_query = format!("?pacticipant={}", urlencoding::encode(pacticipant.unwrap()));

                                                let res = hal_client.clone()
                                                    .fetch(&(currently_deployed_link.to_owned() + &pacticipant_query))
                                                    .await;
                                                match res {
                                                    Ok(result) => {
                                                        // Handle success
                                                        // print!("🧹 Found currently deployed versions");
                                                        // print!("Result JSON: {:#?}", result);
                                                        if let Some(embedded) = result["_embedded"].as_object() {
                                                            if let Some(deployedVersions) = embedded["deployedVersions"].as_array() {
                                                                if deployedVersions.len() == 0 {
                                                                    print!("❌ No currently deployed versions in {} environment", environment.unwrap());
                                                                    std::process::exit(1);
                                                                }
                                                                for deployedVersion in deployedVersions {
                                                                    let pacticipantName = deployedVersion["_embedded"]["pacticipant"]["name"].as_str().unwrap();
                                                                    if pacticipantName == pacticipant.unwrap() {
                                                                        let self_href = deployedVersion["_links"]["self"]["href"].as_str().unwrap();
                                                                        // Send a patch request with the user's payload to selfHref
                                                                        // print!("🧹 Undeploying {} from {} environment", pacticipant.unwrap(), environment.unwrap());
                                                                        // print!("🧹 Sending a patch request to {}", self_href);
                                                                        let mut payload = json!({});
                                                                        payload["currentlyDeployed"] = serde_json::Value::Bool(false);
                                                                        // let pacticipant_query = format!("?pacticipant={}", urlencoding::encode(pacticipant.unwrap()));
                                                                        let res = hal_client.clone().patch_json(self_href, &payload.to_string()).await;
                                                                        match res {
                                                                            Ok(_) => {
                                                                                // Handle success
                                                                                print!("✅ ♻️ Undeployed {} from {} environment", Colour::Green.paint(pacticipant.unwrap()), Colour::Green.paint(environment.unwrap()));
                                                                            }
                                                                            Err(err) => {
                                                                                handle_error(err);
                                                                            }
                                                                        }
                                                                    } else {
                                                                        print!("❌ No currently deployed versions found for {} in {} environment" ,pacticipant.unwrap(), environment.unwrap());
                                                                        std::process::exit(1);
                                                                    }
                                                                }
                                                            } else {
                                                                print!("❌ No currently deployed versions in {} environment", environment.unwrap());
                                                                std::process::exit(1);
                                                            }
                                                            }
                                                            else {
                                                                print!("❌ Could not process hal relation link");
                                                                std::process::exit(1);
                                                            }
                                                    }
                                                    Err(err) => {
                                                        handle_error(err);
                                                    }
                                                }
                                            }
                                            Err(err) => {
                                                handle_error(err);
                                            }
                                        }
                                        } else {
                                            println!("❌ Environment not found");
                                            std::process::exit(1);
                                        }
                                    }
                                    Err(err) => {
                                        handle_error(err);
                                        }
                                    }
                                })
                        }
                        Some(("record-release", args)) => {
                            // 1. Check broker index link for connection
                            // 2, Check version exists "GET /pacticipants/{pacticipant}/versions/{versions}?
                            // "{\"number\":\"5556b8149bf8bac76bc30f50a8a2dd4c22c85f30\",\"createdAt\":\"2024-03-17T07:11:23+00:00\",\"_embedded\":{\"branchVersions\":[{\"name\":\"main\",\"latest\":true,\"_links\":{\"self\":{\"title\":\"Branch version\",\"name\":\"main\",\"href\":\"http://localhost:9292/pacticipants/Example%20App/branches/main/versions/5556b8149bf8bac76bc30f50a8a2dd4c22c85f30\"}}}],\"tags\":[{\"name\":\"main\",\"_links\":{\"self\":{\"title\":\"Tag\",\"name\":\"main\",\"href\":\"http://localhost:9292/pacticipants/Example%20App/versions/5556b8149bf8bac76bc30f50a8a2dd4c22c85f30/tags/main\"}}}]},\"_links\":{\"self\":{\"title\":\"Version\",\"name\":\"5556b8149bf8bac76bc30f50a8a2dd4c22c85f30\",\"href\":\"http://localhost:9292/pacticipants/Example%20App/versions/5556b8149bf8bac76bc30f50a8a2dd4c22c85f30\"},\"pb:pacticipant\":{\"title\":\"Pacticipant\",\"name\":\"Example App\",\"href\":\"http://localhost:9292/pacticipants/Example%20App\"},\"pb:tag\":{\"href\":\"http://localhost:9292/pacticipants/Example%20App/versions/5556b8149bf8bac76bc30f50a8a2dd4c22c85f30/tags/{tag}\",\"title\":\"Get, create or delete a tag for this pacticipant version\",\"templated\":true},\"pb:latest-verification-results-where-pacticipant-is-consumer\":{\"title\":\"Latest verification results for consumer version\",\"href\":\"http://localhost:9292/verification-results/consumer/Example%20App/version/5556b8149bf8bac76bc30f50a8a2dd4c22c85f30/latest\"},\"pb:pact-versions\":[{\"title\":\"Pact\",\"name\":\"Pact between Example App (5556b8149bf8bac76bc30f50a8a2dd4c22c85f30) and Example API\",\"href\":\"http://localhost:9292/pacts/provider/Example%20API/consumer/Example%20App/version/5556b8149bf8bac76bc30f50a8a2dd4c22c85f30\"}],\"pb:record-deployment\":[{\"title\":\"Record deployment to Production\",\"name\":\"production\",\"href\":\"http://localhost:9292/pacticipants/Example%20App/versions/5556b8149bf8bac76bc30f50a8a2dd4c22c85f30/deployed-versions/environment/c540ce64-5493-48c5-ab7c-28dae27b166b\"},{\"title\":\"Record deployment to Test\",\"name\":\"test\",\"href\":\"http://localhost:9292/pacticipants/Example%20App/versions/5556b8149bf8bac76bc30f50a8a2dd4c22c85f30/deployed-versions/environment/cf7dcfdb-3645-4b16-b2f7-7ecb4b6045e0\"}],\"pb:record-release\":[{\"title\":\"Record release to Production\",\"name\":\"production\",\"href\":\"http://localhost:9292/pacticipants/Example%20App/versions/5556b8149bf8bac76bc30f50a8a2dd4c22c85f30/released-versions/environment/c540ce64-5493-48c5-ab7c-28dae27b166b\"},{\"title\":\"Record release to Test\",\"name\":\"test\",\"href\":\"http://localhost:9292/pacticipants/Example%20App/versions/5556b8149bf8bac76bc30f50a8a2dd4c22c85f30/released-versions/environment/cf7dcfdb-3645-4b16-b2f7-7ecb4b6045e0\"}],\"curies\":[{\"name\":\"pb\",\"href\":\"http://localhost:9292/doc/{rel}?context=version\",\"templated\":true}]}}"
                            // 3. Find the pb:record-release link for the specified environment
                            // 4. Send a POST request to the pb:record-release link with an empty payload
                            // 5. Handle the response

                            let version = args.get_one::<String>("version");
                            let pacticipant = args.get_one::<String>("pacticipant");
                            let environment = args.get_one::<String>("environment");
                            let broker_url = get_broker_url(args);
                            let auth = get_auth(args);
                            tokio::runtime::Runtime::new().unwrap().block_on(async {
                            let hal_client: HALClient =
                                HALClient::with_url(&broker_url, Some(auth.clone()));

                            let res = hal_client.clone()
                                .fetch(
                                    &(broker_url.clone()
                                        + "/pacticipants/"
                                        + &pacticipant.unwrap()
                                        + "/versions/"
                                        + &version.unwrap()),
                                )
                                .await;

                            #[derive(Debug, Deserialize, Serialize)]
                            struct PacticipantVersions {
                                _embedded: Embedded,
                                _links: Links,
                                createdAt: String,
                                number: String,
                            }

                            #[derive(Debug, Deserialize, Serialize)]
                            struct Links {
                                #[serde(rename = "self")]
                                self_link: Link,
                                #[serde(rename = "pb:pacticipant")]
                                pacticipant_link: Link,
                                #[serde(rename = "pb:tag")]
                                tag_link: Link,
                                #[serde(rename = "pb:latest-verification-results-where-pacticipant-is-consumer")]
                                latest_verification_results_link: Link,
                                #[serde(rename = "pb:pact-versions")]
                                pact_versions: Vec<Link>,
                                #[serde(rename = "pb:record-deployment")]
                                record_deployment: Vec<Link>,
                                #[serde(rename = "pb:record-release")]
                                record_release: Vec<Link>,
                                curies: Vec<Curies>,
                            }

                            #[derive(Debug, Deserialize, Serialize)]
                            struct Link {
                                href: String,
                                name: Option<String>,
                                title: Option<String>,
                                templated: Option<bool>,
                            }

                            #[derive(Debug, Deserialize, Serialize)]
                            struct Curies {
                                name: String,
                                href: String,
                                templated: bool
                            }

                            #[derive(Debug, Deserialize, Serialize)]
                            struct Embedded {
                                branchVersions: Vec<BranchVersion>,
                                tags: Vec<Tag>,
                            }

                            #[derive(Debug, Deserialize, Serialize)]
                            struct BranchVersion {
                                _links: VersionLinks,
                                latest: bool,
                                name: String,
                            }

                            #[derive(Debug, Deserialize, Serialize)]
                            struct Tag {
                                _links: TagLinks,
                                name: String,
                            }

                            #[derive(Debug, Deserialize, Serialize)]
                            struct VersionLinks {
                                #[serde(rename = "self")]
                                self_link: Link,
                                name: Option<String>,
                                title: Option<String>,
                            }

                            #[derive(Debug, Deserialize, Serialize)]
                            struct TagLinks {
                                #[serde(rename = "self")]
                                self_link: Link,
                                name: Option<String>,
                                title: Option<String>,
                            }
                            match res {
                                Ok(res) => {

                                    let result: Result<PacticipantVersions, serde_json::Error> = serde_json::from_value(res);
                                    match result {
                                        Ok(data) => {
                                        match data._links.record_release.iter().find(|x| x.name == Some(environment.unwrap().to_string())) {
                                            Some(link) => {
                                                let record_release_href = &link.href;

                                                // println!("✅ Found environment {} with {}", Colour::Green.paint(environment.unwrap()), Colour::Green.paint(link_record_deployment_href.clone()));

                                                // <- "POST /pacticipants/Example%20App/versions/5556b8149bf8bac76bc30f50a8a2dd4c22c85f30/deployed-versions/environment/c540ce64-5493-48c5-ab7c-28dae27b166b HTTP/1.1\r\nAccept: application/hal+json\r\nUser-Agent: Ruby\r\nContent-Type: application/json\r\nHost: localhost:9292\r\nContent-Length: 44\r\n\r\n"
                                                // <- "{\"applicationInstance\":\"foo\",\"target\":\"foo\"}"


                                                let mut payload = json!({});
                                                let res: Result<Value, PactBrokerError> = hal_client.clone().post_json(&(record_release_href.clone()), &payload.to_string()).await;
                                                let default_output = "text".to_string();
                                                let output = args.get_one::<String>("output").unwrap_or(&default_output);
                                                match res {
                                                    Ok(res) => {
                                                            if output == "pretty" {
                                                                let json = serde_json::to_string_pretty(&res).unwrap();
                                                                println!("{}", json);
                                                            } else if output == "json" {
                                                                println!("{}", serde_json::to_string(&res).unwrap());
                                                            } else if output == "id" {
                                                                println!("{}", res["uuid"].to_string().trim_matches('"'));
                                                            }
                                                            else {
                                                                // let uuid = res["uuid"].to_string();
                                                                println!("✅ Recorded release of {} version {} to {} environment in the Pact Broker.", Colour::Green.paint(pacticipant.unwrap()), Colour::Green.paint(version.unwrap()),Colour::Green.paint(environment.unwrap()));
                                                                // println!("✅ Created {} environment in the Pact Broker with UUID {}", Colour::Green.paint(name.unwrap()), Colour::Green.paint(uuid.trim_matches('"')));

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
                                                        }
                                            None => {
                                                println!("❌ Environment {} does not exist", Colour::Red.paint(environment.unwrap()));
                                                std::process::exit(1);
                                            }}
                                        }
                                        Err(err) => {
                                            println!("Error: {}", err);
                                            println!("❌ {}", Colour::Red.paint("Failed to record release"));
                                            std::process::exit(1);
                                        }
                                    }
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
                        }})
                        }
                        Some(("record-support-ended", args)) => {
                            // 1. Check broker index link for connection
                            // <- "GET /? HTTP/1.1\r\nAccept: application/hal+json\r\nUser-Agent: Ruby\r\nHost: localhost:9292\r\n\r\n"
                            // -> "HTTP/1.1 200 OK\r\n"
                            // 2. Call environments and check the specified enviroment exists, get the environment link
                            // <- "GET /environments? HTTP/1.1\r\nAccept: application/hal+json\r\nUser-Agent: Ruby\r\nHost: localhost:9292\r\n\r\n"
                            // -> "HTTP/1.1 200 OK\r\n"
                            // 3. Call the environment link and check the specified version exists, get the version link
                            // <- "GET /environments/c540ce64-5493-48c5-ab7c-28dae27b166b? HTTP/1.1\r\nAccept: application/hal+json\r\nUser-Agent: Ruby\r\nHost: localhost:9292\r\n\r\n"
                            // -> "HTTP/1.1 200 OK\r\n"
                            // 4. Call the /environments/c540ce64-5493-48c5-ab7c-28dae27b166b/deployed-versions/currently-deployed?pacticipant=Example+App link, and check our app is currently deployed
                            // <- "GET /environments/c540ce64-5493-48c5-ab7c-28dae27b166b/deployed-versions/currently-deployed?pacticipant=Example+App HTTP/1.1\r\nAccept: application/hal+json\r\nUser-Agent: Ruby\r\nHost: localhost:9292\r\n\r\n"
                            // -> "HTTP/1.1 200 OK\r\n"
                            // 5. perform a patch request to the /environments/c540ce64-5493-48c5-ab7c-28dae27b166b/deployed-versions/9b756f93-19a2-4ca7-ae36-c0b917ac1f21 link to set currentlyDeployed to false
                            // <- "PATCH /deployed-versions/9b756f93-19a2-4ca7-ae36-c0b917ac1f21 HTTP/1.1\r\nAccept: application/hal+json\r\nUser-Agent: Ruby\r\nContent-Type: application/merge-patch+json\r\nHost: localhost:9292\r\nContent-Length: 27\r\n\r\n"
                            // <- "{\"currentlyDeployed\":false}"

                            let pacticipant = args.get_one::<String>("pacticipant");
                            let environment = args.get_one::<String>("environment");
                            let broker_url = get_broker_url(args);
                            let auth = get_auth(args);
                            tokio::runtime::Runtime::new().unwrap().block_on(async {
                                let hal_client: HALClient = HALClient::with_url(&broker_url, Some(auth.clone()));


                                let res = hal_client.clone()
                                    .fetch(&(broker_url.clone() + "/"))
                                    .await;
                                match res {
                                    Ok(_) => {
                                        // Handle success
                                    }
                                    Err(err) => {
                                        handle_error(err);
                                    }
                                }

                                #[derive(Debug, serde::Deserialize)]
                                struct Environment {
                                    uuid: String,
                                    name: String,
                                    displayName: String,
                                    production: bool,
                                    createdAt: String,
                                }

                                let res = hal_client.clone()
                                    .fetch(&(broker_url.clone() + "/environments?"))
                                    .await;
                                    match res {
                                        Ok(response) => {
                                            let environments: Vec<Environment> = response["_embedded"]["environments"]
                                                .as_array()
                                                .unwrap()
                                                .iter()
                                                .map(|env| serde_json::from_value(env.clone()).unwrap())
                                                .collect();
                                            let environment_exists = environments.iter().any(|env| env.name == environment.clone().unwrap().to_string());
                                            if environment_exists {
                                                let environment_uuid = &environments.iter().find(|env| env.name == environment.clone().unwrap().to_string()).unwrap().uuid;
                                                // Use environment_uuid in step 3

                                                // 3. Call the environment link and check the specified version exists, get the version link
                                                let res = hal_client.clone()
                                                .fetch(&(broker_url.clone() + "/environments/" + &environment_uuid + "?"))
                                                .await;
                                            match res {
                                                Ok(result) => {
                                                    // print!("✅ Environment found");
                                                    // print!("🧹 Undeploying {} from {} environment", pacticipant.unwrap(), environment.unwrap());
                                                    // print!("Result JSON: {:#?}", result);
                                                    // todo - handle application instance

                                                    let currently_supported_released_link = result["_links"]["pb:currently-supported-released-versions"]["href"].as_str().unwrap();
                                                    let pacticipant_query = format!("?pacticipant={}", urlencoding::encode(pacticipant.unwrap()));

                                                    let res = hal_client.clone()
                                                        .fetch(&(currently_supported_released_link.to_owned() + &pacticipant_query))
                                                        .await;
                                                    match res {
                                                        Ok(result) => {
                                                            // Handle success
                                                            // print!("🧹 Found currently deployed versions");
                                                            // print!("Result JSON: {:#?}", result);
                                                            if let Some(embedded) = result["_embedded"].as_object() {
                                                                if let Some(releasedVersions) = embedded["releasedVersions"].as_array() {
                                                                    if releasedVersions.len() == 0 {
                                                                        print!("❌ No currently released versions in {} environment", environment.unwrap());
                                                                        std::process::exit(1);
                                                                    }
                                                                    for releasedVersion in releasedVersions {
                                                                        let pacticipantName = releasedVersion["_embedded"]["pacticipant"]["name"].as_str().unwrap();
                                                                        if pacticipantName == pacticipant.unwrap() {
                                                                            let self_href = releasedVersion["_links"]["self"]["href"].as_str().unwrap();
                                                                            // Send a patch request with the user's payload to selfHref
                                                                            // print!("🧹 Undeploying {} from {} environment", pacticipant.unwrap(), environment.unwrap());
                                                                            // print!("🧹 Sending a patch request to {}", self_href);
                                                                            let mut payload = json!({});
                                                                            payload["currentlySupported"] = serde_json::Value::Bool(false);
                                                                            // let pacticipant_query = format!("?pacticipant={}", urlencoding::encode(pacticipant.unwrap()));
                                                                            let res = hal_client.clone().patch_json(self_href, &payload.to_string()).await;
                                                                            match res {
                                                                                Ok(_) => {
                                                                                    // Handle success
                                                                                    print!("✅ ♻️ Recorded support ended {} from {} environment", Colour::Green.paint(pacticipant.unwrap()), Colour::Green.paint(environment.unwrap()));
                                                                                }
                                                                                Err(err) => {
                                                                                    handle_error(err);
                                                                                }
                                                                            }
                                                                        } else {
                                                                            print!("❌ No currently released versions found for {} in {} environment" ,pacticipant.unwrap(), environment.unwrap());
                                                                            std::process::exit(1);
                                                                        }
                                                                    }
                                                                } else {
                                                                    print!("❌ No currently released versions in {} environment", environment.unwrap());
                                                                    std::process::exit(1);
                                                                }
                                                                }
                                                            else {
                                                                print!("❌ Could not process hal relation link");
                                                                std::process::exit(1);
                                                            }
                                                        }
                                                        Err(err) => {
                                                            handle_error(err);
                                                        }
                                                    }
                                                }
                                                Err(err) => {
                                                    handle_error(err);
                                                }
                                            }
                                            } else {
                                                println!("❌ Environment not found");
                                                std::process::exit(1);
                                            }
                                        }
                                        Err(err) => {
                                            handle_error(err);
                                            }
                                        }
                            })
                        }
                        Some(("can-i-deploy", args)) => {
                            let pacticipant = args.get_one::<String>("pacticipant");
                            let version = args.get_one::<String>("version");
                            let ignore = args.get_flag("ignore");
                            let latest = args.get_flag("latest");
                            let branch = args.get_one::<String>("branch");
                            let main_branch = args.get_flag("main-branch");
                            let no_main_branch = args.get_flag("no-main-branch");
                            let skip_main_branch = args.get_flag("skip-main-branch");
                            let to_environment = args.get_one::<String>("to-environment");
                            let to = args.get_one::<String>("to");
                            let retry_while_unknown = args.get_one::<String>("retry-while-unknown");
                            let retry_interval = args.get_one::<String>("retry-interval");
                            let dry_run = args.get_flag("dry-run");
                            let no_dry_run = args.get_flag("no-dry-run");
                            let skip_dry_run = args.get_flag("skip-dry-run");

                            let broker_url = get_broker_url(args);
                            let auth = get_auth(args);

                            #[derive(Debug, serde::Deserialize)]
                            struct Summary {
                                deployable: Option<bool>,
                                reason: String,
                                success: u32,
                                failed: u32,
                                unknown: u32,
                            }

                            #[derive(Debug, serde::Deserialize)]
                            struct Notice {
                                #[serde(rename = "type")]
                                notice_type: String,
                                text: String,
                            }

                            #[derive(Debug, serde::Deserialize)]
                            struct Version {
                                number: String,
                                branch: String,
                                branches: Vec<Branch>,
                                branchVersions: Vec<BranchVersion>,
                                environments: Vec<Environment>,
                                _links: Links,
                                tags: Vec<Tag>,
                            }

                            #[derive(Debug, serde::Deserialize)]
                            struct Branch {
                                name: String,
                                latest: Option<bool>,
                                _links: Links,
                            }

                            #[derive(Debug, serde::Deserialize)]
                            struct BranchVersion {
                                name: String,
                                latest: Option<bool>,
                                _links: Links,
                            }

                            #[derive(Debug, serde::Deserialize)]
                            struct Environment {
                                uuid: String,
                                name: String,
                                displayName: String,
                                production: Option<bool>,
                                createdAt: String,
                                _links: Links,
                            }

                            #[derive(Debug, serde::Deserialize)]
                            struct Links {
                                #[serde(rename = "self")]
                                self_link: SelfLink,
                            }

                            #[derive(Debug, serde::Deserialize)]
                            struct SelfLink {
                                href: String,
                            }

                            #[derive(Debug, serde::Deserialize)]
                            struct Tag {
                                name: String,
                                latest: Option<bool>,
                                _links: Links,
                            }

                            #[derive(Debug, serde::Deserialize)]
                            struct Consumer {
                                name: String,
                                version: Option<Version>,
                                _links: Links,
                            }

                            #[derive(Debug, serde::Deserialize)]
                            struct Provider {
                                name: String,
                                version: Option<Version>,
                                _links: Links,
                            }

                            #[derive(Debug, serde::Deserialize)]
                            struct Pact {
                                createdAt: String,
                                _links: Links,
                            }

                            #[derive(Debug, serde::Deserialize)]
                            struct VerificationResult {
                                success: Option<bool>,
                                verifiedAt: Option<String>,
                                _links: Links,
                            }

                            #[derive(Debug, serde::Deserialize)]
                            struct MatrixItem {
                                consumer: Consumer,
                                provider: Provider,
                                pact: Pact,
                                verificationResult: Option<VerificationResult>,
                            }

                            #[derive(Debug, serde::Deserialize)]
                            struct Data {
                                summary: Summary,
                                notices: Vec<Notice>,
                                matrix: Vec<MatrixItem>,
                            }

                            tokio::runtime::Runtime::new().unwrap().block_on(async {
                                let hal_client: HALClient =
                                    HALClient::with_url(&broker_url, Some(auth.clone()));
                                // let matrix_href_path = "/matrix?q[][pacticipant]=Example+App&q[][latest]=true&q[][branch]=foo&latestby=cvp&latest=true".to_string();
                                // let matrix_href_path = "/matrix?q[][pacticipant]=Example+App&q[][version]=5556b8149bf8bac76bc30f50a8a2dd4c22c85f30&latestby=cvp&latest=true".to_string();
                                let mut matrix_href_path = "/matrix?".to_string();

                                if let Some(pacticipant) = pacticipant {
                                    matrix_href_path
                                        .push_str(&format!("q[][pacticipant]={}&", pacticipant));
                                }

                                if let Some(version) = version {
                                    matrix_href_path
                                        .push_str(&format!("q[][version]={}&", version));
                                }

                                if latest {
                                    matrix_href_path.push_str("latest=true&");
                                }

                                if let Some(branch) = branch {
                                    matrix_href_path.push_str(&format!("q[][branch]={}&", branch));
                                }

                                if let Some(to_environment) = to_environment {
                                    matrix_href_path
                                        .push_str(&format!("environment={}&", to_environment));
                                }
                                if let Some(to) = to {
                                    matrix_href_path.push_str(&format!("tag={}&", to));
                                }

                                matrix_href_path.push_str("latestby=cvp");
                                // query the hal relation link to get the latest pact versions
                                let res = hal_client
                                    .clone()
                                    .fetch(&(broker_url.clone() + &matrix_href_path))
                                    .await;
                                match res {
                                    Ok(res) => {
                                        // handle user args for additional processing
                                        let output: Result<
                                            Option<&String>,
                                            clap::parser::MatchesError,
                                        > = args.try_get_one::<String>("output");

                                        // render result
                                        match output {
                                            Ok(Some(output)) => {
                                                if output == "json" {
                                                    let json: String =
                                                        serde_json::to_string(&res.clone())
                                                            .unwrap();
                                                    println!("{}", json);
                                                } else if output == "table" {
                                                    println!("{:?}", res.clone());

                                                    let data: Data = match serde_json::from_str(
                                                        &res.clone().to_string(),
                                                    ) {
                                                        Ok(data) => data,
                                                        Err(err) => {
                                                            println!(
                                                                "❌ {}",
                                                                Colour::Red.paint(err.to_string())
                                                            );
                                                            Data {
                                                                summary: Summary {
                                                                    deployable: Some(false),
                                                                    success: 0,
                                                                    failed: 0,
                                                                    reason: "No summary found"
                                                                        .to_string(),
                                                                    unknown: 1,
                                                                },
                                                                notices: Vec::new(),
                                                                matrix: Vec::new(),
                                                            }
                                                        }
                                                    };

                                                    if data.matrix.len() > 0 {
                                                        let mut table = Table::new();

                                                        table.load_preset(UTF8_FULL).set_header(
                                                            vec![
                                                                "CONSUMER",
                                                                "C.VERSION",
                                                                "PROVIDER",
                                                                "P.VERSION",
                                                                "SUCCESS?",
                                                                "RESULT",
                                                            ],
                                                        );
                                                        for matrix_item in data.matrix {
                                                            let verification_result = &matrix_item
                                                                .verificationResult
                                                                .map(|result| {
                                                                    result
                                                                        .success
                                                                        .unwrap_or(false)
                                                                        .to_string()
                                                                })
                                                                .unwrap_or_else(|| {
                                                                    "false".to_string()
                                                                });

                                                            table.add_row(vec![
                                                                matrix_item.consumer.name,
                                                                matrix_item
                                                                    .consumer
                                                                    .version
                                                                    .map(|result| {
                                                                        result.number.to_string()
                                                                    })
                                                                    .unwrap_or_else(|| {
                                                                        "unknown".to_string()
                                                                    }),
                                                                matrix_item.provider.name,
                                                                matrix_item
                                                                    .provider
                                                                    .version
                                                                    .map(|result| {
                                                                        result.number.to_string()
                                                                    })
                                                                    .unwrap_or_else(|| {
                                                                        "unknown".to_string()
                                                                    }),
                                                                verification_result.to_string(),
                                                                verification_result.to_string(),
                                                            ]);
                                                        }
                                                        println!("{table}");
                                                    }

                                                    if data.notices.len() > 0 {
                                                        for notice in data.notices {
                                                            if notice.notice_type == "warning" {
                                                                println!(
                                                                    "⚠️ {}",
                                                                    Colour::Yellow
                                                                        .paint(notice.text)
                                                                );
                                                            } else if notice.notice_type == "error"
                                                            {
                                                                println!(
                                                                    "❌ {}",
                                                                    Colour::Red.paint(notice.text)
                                                                );
                                                            } else {
                                                                println!(
                                                                    "📌 {}",
                                                                    Colour::Green
                                                                        .paint(notice.text)
                                                                );
                                                            }
                                                        }
                                                    }
                                                    if data.summary.deployable.unwrap_or(false) {
                                                        let computer_says_yes =
                                                            Colour::Green.paint("\\o/");
                                                        println!(
                                                            r"✅ Computer says yes {}",
                                                            computer_says_yes
                                                        );
                                                    } else {
                                                        let computer_says_no =
                                                            Colour::Red.paint("¯\\_(ツ)_/¯");
                                                        println!(
                                                            r"❌ Computer says no {}",
                                                            computer_says_no
                                                        );
                                                        std::process::exit(1);
                                                    }
                                                }
                                            }
                                            _ => {
                                                println!("{:?}", res.clone());
                                            }
                                            Err(res) => {
                                                println!("no output match provided {:?}", res);
                                                std::process::exit(1);
                                            }
                                        }
                                    }
                                    Err(res) => {
                                        handle_error(res);
                                    }
                                }
                            })
                        }
                        Some(("can-i-merge", args)) => {
                            // Handle can-i-merge command
                            println!("Unimplemented");
                            std::process::exit(1);
                        }
                        Some(("create-or-update-pacticipant", args)) => {
                            // Handle create-or-update-pacticipant command
                            println!("Unimplemented");
                            std::process::exit(1);
                        }
                        Some(("describe-pacticipant", args)) => {
                            // Handle describe-pacticipants command
                            println!("Unimplemented");
                            std::process::exit(1);
                        }
                        Some(("list-pacticipants", args)) => {
                            // Handle list-pacticipants command
                            println!("Unimplemented");
                            std::process::exit(1);
                        }
                        Some(("create-webhook", args)) => {
                            // Handle create-webhook command
                            println!("Unimplemented");
                            std::process::exit(1);
                        }
                        Some(("create-or-update-webhook", args)) => {
                            // Handle create-or-update-webhook command
                            println!("Unimplemented");
                            std::process::exit(1);
                        }
                        Some(("test-webhook", args)) => {
                            // Handle test-webhook command

                            println!("Unimplemented");
                            std::process::exit(1);
                        }
                        Some(("delete-branch", args)) => {
                            // Handle delete-branch command
                            println!("Unimplemented");
                            std::process::exit(1);
                        }
                        Some(("create-version-tag", args)) => {
                            // Handle create-version-tag command
                            println!("Unimplemented");
                            std::process::exit(1);
                        }
                        Some(("describe-version", args)) => {
                            // Handle describe-version command
                            println!("Unimplemented");
                            std::process::exit(1);
                        }
                        Some(("create-or-update-version", args)) => {
                            // Handle create-or-update-version command
                            println!("Unimplemented");
                            std::process::exit(1);
                        }
                        Some(("generate-uuid", args)) => {
                            println!("{}", uuid::Uuid::new_v4());
                        }
                        _ => {
                            println!("⚠️  No option provided, try running pact-broker --help");
                        }
                    }
                }
                Some(("pactflow", args)) => match args.subcommand() {
                    Some(("publish-provider-contract", args)) => {
                        println!("{:?}", args);

                        println!("Unimplemented");
                        std::process::exit(1);
                    }
                    _ => {
                        println!("⚠️  No option provided, try running pactflow --help");
                    }
                },
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
                }
                Some(("standalone", args)) => {
                    let home_dir = env::var("HOME").unwrap();
                    let pact_dir = format!("{}/.pact/traveling-broker", home_dir);
                    let pid_file_path = format!("{}/pact_broker-standalone.pid", pact_dir);

                    match args.subcommand() {
                        Some(("start", args)) => {
                            tokio::runtime::Runtime::new().unwrap().block_on(async {
                            let mut os = match env::consts::OS {
                                "macos" => "osx",
                                other => other,
                            };

                            let arch = match env::consts::ARCH {
                                "aarch64" => "arm64",
                                other => other,
                            };

                            // check if os/arch is supported
                            // supported are osx, linux, windows and arm64, x86_64
                            if os != "osx" && os != "linux" && os != "windows" {
                                println!("⚠️  Unsupported OS: {}", os);
                                std::process::exit(1);
                            }
                            if arch != "arm64" && arch != "x86_64" {
                                println!("⚠️  Unsupported architecture: {}", arch);
                                std::process::exit(1);
                            }

                            // Store the binary in the user's home .pact/traveling-broker directory
                            let _ = fs::create_dir_all(&pact_dir);
                            let broker_archive_path = if os == "windows" {
                                format!("{}/packed-broker.zip", pact_dir)
                            } else {
                                format!("{}/traveling-pact-20230803-3.2.2-{}-{}-full.tar.gz", pact_dir, os, arch)
                            };
                            let app_path = if os == "windows" {
                                format!("{}/packed-broker/pact-broker-app.bat", pact_dir)
                            } else {
                                format!("{}/pact-broker-app.sh", pact_dir)
                            };

                            // check is app path exists, if so, do not download the file

                            if !fs::metadata(&app_path).is_ok() {
                                // Download the correct version of the traveling ruby binary
                                let url = if os == "windows" {
                                    format!(
                                        "https://github.com/YOU54F/test/releases/download/0.0.0/packed-broker.zip",
                                    )
                                } else {
                                    let mut os_variant: String = os.to_string();
                                    if os == "linux" && cfg!(target_env = "musl") {
                                        let output = Command::new("ldd")
                                            .arg("/bin/sh")
                                            .output()
                                            .ok();

                                        if let Some(output) = output {
                                            let output_str = String::from_utf8_lossy(&output.stdout);
                                            if output_str.contains("musl") {
                                                println!("🚀 Detected musl libc, downloading musl version");
                                                os_variant.push_str("-musl");
                                            }
                                        } else {
                                            println!("⚠️  Failed to execute ldd command, downloading glibc version");
                                        }
                                    }
                                    let download_url = format!(
                                        "https://github.com/YOU54F/traveling-ruby/releases/download/rel-20230803-pact/traveling-pact-20230803-3.2.2-{}-{}-full.tar.gz",
                                        os_variant, arch
                                    );
                                    download_url

                                };
                                println!("🚀 Downloading Pact Broker binary from {}", url);
                                let response = reqwest::get(&url).await.unwrap();
                                let body = response.bytes().await.unwrap();

                                let mut file = fs::File::create(&broker_archive_path).unwrap();
                                let _ = file.write_all(&body);

                                // Unpack the binary
                                println!("🚀 Unpacking the binary...");
                                if os == "windows" {
                                    if Command::new("unzip").output().is_ok() {
                                        println!("Unpacking {} to {}, tool: {}", broker_archive_path, pact_dir, "unzip");
                                        Command::new("unzip")
                                            .arg(&broker_archive_path)
                                            .arg("-d")
                                            .arg(&pact_dir)
                                            .output()
                                            .expect("Failed to unpack the binary");
                                    } else {
                                        println!("Unpacking {} to {}, tool: {}", broker_archive_path, pact_dir, "pwsh Expand-Archive");
                                        Command::new("powershell")
                                            .arg("-Command")
                                            .arg(format!(
                                                "Expand-Archive -Path '{}' -DestinationPath '{}'",
                                                &broker_archive_path, &pact_dir
                                            ))
                                            .output()
                                            .expect("Failed to unpack the binary");
                                    }
                                } else {
                                    println!("Unpacking {} to {}, tool: {}", broker_archive_path, pact_dir, "tar");
                                    Command::new("tar")
                                        .arg("-xf")
                                        .arg(&broker_archive_path)
                                        .arg("-C")
                                        .arg(&pact_dir)
                                        .output()
                                        .expect("Failed to unpack the binary");
                                }
                                println!("🚀 Removing the archive at {}", broker_archive_path);
                                let _ = fs::remove_file(broker_archive_path);
                        } else {
                            println!("🚀 Pact Broker binary already exists at {}", app_path);
                        }
                            // Execute the pact-broker-app.sh file
                            println!("🚀 Starting Pact Broker (this may take a few seconds)...");
                            println!("🚀 Running: {}", app_path);
                            let mut child_cmd = Command::new(&app_path);

                            if let Ok(mut child) = child_cmd.spawn() {
                                let pid = child.id();
                                let mut pid_file = fs::File::create(&pid_file_path).unwrap();
                                let _ = pid_file.write_all(pid.to_string().as_bytes());
                                println!("🚀 Pact Broker is running on http://localhost:9292");
                                println!("🚀 PID: {}", pid);

                                // we should support a detach flag to run the broker in the background
                                let detach = args.get_flag("detach");
                                if detach {
                                    println!("🚀 Running in the background");
                                    std::process::exit(0);
                                } else {
                                // Await SIGKILL from the user
                                let _ = tokio::signal::ctrl_c().await;

                                // Send SIGKILL to the app
                                let _ = child.kill();
                                let _ = fs::remove_file(&pid_file_path);
                                std::process::exit(0);
                                }

                            } else {
                                println!("{} didn't start", app_path);
                                std::process::exit(1);
                            }
                        });
                        }
                        Some(("stop", args)) => {
                            // Stop the broker
                            let pid_file = fs::File::open(&pid_file_path);
                            match pid_file {
                                Ok(mut file) => {
                                    let mut pid = String::new();
                                    file.read_to_string(&mut pid).unwrap();
                                    let pid = pid.trim().parse::<u32>().unwrap();
                                    println!("🚀 Stopping Pact Broker with PID: {}", pid);
                                    Command::new("kill")
                                        .arg(pid.to_string())
                                        .output()
                                        .expect("⚠️ Failed to stop the broker");
                                    let _ = fs::remove_file(&pid_file_path);
                                    println!("🛑 Pact Broker stopped");
                                    std::process::exit(0);
                                }
                                Err(_) => {
                                    println!("⚠️ Pact Broker is not running");
                                    std::process::exit(1);
                                }
                            }
                        }
                        _ => {
                            println!("⚠️  No option provided, try running standalone --help");
                        }
                    }
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
                        println!("Unimplemented");
                        std::process::exit(1);
                    }

                    fn download_project_with_org(org: &str, project: &str) {
                        println!("Downloading project {}", project);
                        // Implement the logic to download the project with the specified organization here
                        println!("Downloaded project {}", project);

                        println!("Unimplemented");
                        std::process::exit(1);
                    }
                }
                Some(("project", args)) => {
                    match args.subcommand() {
                        Some(("install", args)) => {
                            let language = args.get_one::<String>("language").unwrap().as_str();
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
                            println!("Unimplemented");
                            std::process::exit(1);
                        }
                        Some(("link", args)) => {
                            println!("Unimplemented");
                            std::process::exit(1);
                        }
                        Some(("issue", args)) => {
                            println!("Unimplemented");
                            std::process::exit(1);
                        }
                        Some(("docs", args)) => {
                            println!("Unimplemented");
                            std::process::exit(1);
                        }
                        _ => {
                            println!("⚠️  No option provided, try running project --help");
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
                _ => cli::build_cli().print_help().unwrap(),
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

// pub fn list_latest_pact_versions(broker_details: &BrokerDetails, output_type: OutputType, verbose: bool) {
//     // setup client with broker url and credentials
//     let broker_url = &broker_details.url;
//     let auth = &broker_details.auth;
//     tokio::runtime::Runtime::new().unwrap().block_on(async {
//         // query pact broker index and get hal relation link
//         let hal_client: HALClient = HALClient::with_url(broker_url, auth.clone());
//         let pb_latest_pact_versions_href_path = get_broker_relation(
//             hal_client.clone(),
//             "pb:latest-pact-versions".to_string(),
//             broker_url.to_string()
//         )
//         .await;
//         // query the hal relation link to get the latest pact versions
//         let res = follow_broker_relation(
//             hal_client.clone(),
//             "pb:latest-pact-versions".to_string(),
//             pb_latest_pact_versions_href_path,
//         )
//         .await;

//         // handle user args for additional processing

//         if verbose {
//             println!("Verbose mode is enabled");
//         }
//     });
// }
