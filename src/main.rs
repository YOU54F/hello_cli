// use std::collections::HashMap;
mod cli;
use clap::error::ErrorKind;
use clap_complete::{generate_to, Shell};
use pact_broker::{HALClient, Link, PactBrokerError};
use serde_json::Value;
use std::process::ExitCode;
use std::process::ExitStatus;
use std::process::Termination;
use std::str::FromStr;
mod pact_broker;
mod pact_plugin_cli;
use crate::cli::pact_mock_server_cli;
use crate::cli::pact_stub_server_cli;
use crate::cli::pact_verifier_cli;

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
                            print!("{:?}", args);
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
                            // Handle create-environment command
                            // Ok(());
                        }
                        Some(("update-environment", args)) => {
                            // Handle update-environment command
                            // Ok(());
                        }
                        Some(("describe-environment", args)) => {
                            // Handle describe-environment command
                            // Ok(());
                        }
                        Some(("delete-environment", args)) => {
                            // Handle delete-environment command
                            // Ok(());
                        }
                        Some(("list-environments", args)) => {
                            // Handle list-environments command
                            // Ok(());
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

                            //     // Handle can-i-deploy command
                            //     // setup client with broker ucarl and credentials
                            //     let broker_url = get_broker_url(args);
                            //     let auth = get_auth(args);
                            //     // query pact broker index and get hal relation link
                            //     let hal_client: HALClient =
                            //         HALClient::with_url(&broker_url, Some(auth.clone()));
                            //     let matrix_href_path = "/matrix?pacticipant=Example+App&latest=true&latestby=cvp&latest=true".to_string();
                            //     // let matrix_href_path = "/matrix?q[][pacticipant]=Example+App&q[][latest]=true&latestby=cvp&latest=true".to_string();
                            //     // query the hal relation link to get the latest pact versions
                            //     let res = follow_broker_relation(
                            //         hal_client.clone(),
                            //         "pb:latest-pact-versions".to_string(),
                            //         matrix_href_path,
                            //     )
                            //     .await;
                            //     match res {
                            //         Ok(res) => {
                            //             // handle user args for additional processing
                            //             let output: Result<Option<&String>, clap::parser::MatchesError> =
                            //                 args.try_get_one::<String>("output");

                            //             // render result
                            //             match output {
                            //                 Ok(Some(output)) => {
                            //                     if output == "json" {
                            //                         let json: String =
                            //                             serde_json::to_string(&res.clone()).unwrap();
                            //                         println!("{}", json);
                            //                     } else if output == "table" {
                            //                         generate_table(
                            //                             &res,
                            //                             vec![
                            //                                 "CONSUMER",
                            //                                 "CONSUMER_VERSION",
                            //                                 "PROVIDER",
                            //                                 "CREATED_AT",
                            //                             ],
                            //                             vec![
                            //                                 vec!["_embedded", "consumer", "name"],
                            //                                 vec![
                            //                                     "_embedded",
                            //                                     "consumer",
                            //                                     "_embedded",
                            //                                     "version",
                            //                                     "number",
                            //                                 ],
                            //                                 vec!["_embedded", "provider", "name"],
                            //                                 vec!["createdAt"],
                            //                             ],
                            //                         );
                            //                     }
                            //                 }
                            //                 Ok(None) => {
                            //                     println!("{:?}", res.clone());
                            //                 }
                            //                 Err(res) => {
                            //                     println!("{:?}", res);
                            //                     // os.exit(1)
                            //                 }
                            //             }
                            //         }
                            //         Err(res) => {
                            //             println!("{:?}", res);
                            //             // os.exit(1)
                            //         }
                            //     }
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
                            // Handle generate-uuid command
                            // Ok(());
                        }

                        _ => {
                            println!("⚠️  No option provided, try running pact-broker --help");

                            // Ok(());
                        }
                    }
                }
                Some(("pactflow", args)) => {
                    match args.subcommand() {
                        Some(("publish-provider-contract", args)) => {
                            print!("{:?}", args);

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
                    print!(
                        "ℹ️  {} shell completions for pact_cli written to {}",
                        &shell_enum, &out_dir
                    );

                    // Ok(());
                }
                Some(("plugin", args)) => {
                    let _ = pact_plugin_cli::main::run(args);
                }
                Some(("mock", args)) => {
                    tokio::runtime::Runtime::new().unwrap().block_on(async {
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
                    })
                }
                Some(("stub", args)) => {
                    tokio::runtime::Runtime::new().unwrap().block_on(async {
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
                    })
                }
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
