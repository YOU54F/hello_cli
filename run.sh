#!/bin/sh
set -e

target/release/pact_cli --help
target/release/pact_cli pact-broker --help
target/release/pact_cli pactflow --help
target/release/pact_cli completions --help
target/release/pact_cli docker --help
target/release/pact_cli plugin --help
target/release/pact_cli plugin list --help
target/release/pact_cli plugin list installed --help
target/release/pact_cli plugin list known --help
target/release/pact_cli plugin env --help
target/release/pact_cli plugin install --help
target/release/pact_cli plugin remove --help
target/release/pact_cli plugin enable --help
target/release/pact_cli plugin disable --help
target/release/pact_cli plugin repository --help
target/release/pact_cli plugin repository validate --help
target/release/pact_cli plugin repository new --help
target/release/pact_cli plugin repository add-plugin-version --help
target/release/pact_cli plugin repository add-plugin-version git-hub --help
target/release/pact_cli plugin repository add-plugin-version file --help
target/release/pact_cli plugin repository add-all-plugin-versions --help
target/release/pact_cli plugin repository yank-version --help
target/release/pact_cli plugin repository list --help
target/release/pact_cli plugin repository list-versions --help
target/release/pact_cli stub --help
target/release/pact_cli verifier --help
target/release/pact_cli mock --help
target/release/pact_cli mock start --help
target/release/pact_cli mock list --help
target/release/pact_cli mock create --help
target/release/pact_cli mock verify --help
target/release/pact_cli mock shutdown --help
target/release/pact_cli mock shutdown-master --help

