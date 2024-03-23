use clap::{Arg, Command};
pub mod pact_mock_server_cli;
pub mod pact_stub_server_cli;
pub mod pact_verifier_cli;

pub fn build_cli() -> Command {
    let app = Command::new("pact_cli")
        .about("A pact cli tool")
        .subcommand(
            Command::new("pact-broker")
                .args(add_output_arguments(
                    ["json", "text", "table", "pretty"].to_vec(),
                    "text",
                ))
                .subcommand(add_publish_pacts_subcommand())
                .subcommand(add_list_latest_pact_versions_subcommand())
                .subcommand(add_create_environment_subcommand())
                .subcommand(add_update_environment_subcommand())
                .subcommand(add_delete_environment_subcommand())
                .subcommand(add_describe_environment_subcommand())
                .subcommand(add_list_environments_subcommand())
                .subcommand(add_record_deployment_subcommand())
                .subcommand(add_record_undeployment_subcommand())
                .subcommand(add_record_release_subcommand())
                .subcommand(add_record_support_ended_subcommand())
                .subcommand(add_can_i_deploy_subcommand())
                .subcommand(add_can_i_merge_subcommand())
                .subcommand(add_create_or_update_pacticipant_subcommand())
                .subcommand(add_describe_pacticipant_subcommand())
                .subcommand(add_list_pacticipants_subcommand())
                .subcommand(add_create_webhook_subcommand())
                .subcommand(add_create_or_update_webhook_subcommand())
                .subcommand(add_test_webhook_subcommand())
                .subcommand(add_delete_branch_subcommand())
                .subcommand(add_create_version_tag_subcommand())
                .subcommand(add_describe_version_subcommand())
                .subcommand(add_create_or_update_version_subcommand())
                .subcommand(add_generate_uuid_subcommand()),
        )
        .subcommand(Command::new("pactflow").subcommand(add_publish_provider_contract_subcommand()))
        .subcommand(add_completions_subcommand())
        .subcommand(add_docker_broker_subcommand())
        .subcommand(add_examples_subcommand())
        .subcommand(add_project_subcommand())
        .subcommand(add_standalone_broker_subcommand())
        .subcommand(add_plugin_cli_subcommand().arg_required_else_help(true))
        .subcommand(pact_mock_server_cli::main::setup_args())
        .subcommand(pact_stub_server_cli::main::build_args())
        .subcommand(
            pact_verifier_cli::main::build_args()
                .arg_required_else_help(true)
                .disable_version_flag(true),
        );
    // Continue adding other subcommands as needed
    // ...
    app
}

fn add_completions_subcommand() -> Command {
    Command::new("completions") 
    .about("Generates completion scripts for your shell")
    .arg(Arg::new("shell")
        .value_name("SHELL")
        .required(true)
        .value_parser(clap::builder::PossibleValuesParser::new(&["bash", "fish", "zsh", "powershell", "elvish"]))
        .help("The shell to generate the script for"))
    .arg(Arg::new("dir")
        .short('d')
        .long("dir")
        .value_name("DIRECTORY")
        .required(false)
        .default_value(".")
        .num_args(1)
        .value_parser(clap::builder::NonEmptyStringValueParser::new())
        .help("The directory to write the shell completions to, default is the current directory"))
}
fn add_docker_broker_subcommand() -> Command {
    Command::new("docker")
        .about("Run the Pact Broker as a Docker container")
        .subcommand(Command::new("start").about("Start the Pact Broker as a Docker container"))
        .subcommand(Command::new("stop").about("Stop the Pact Broker Docker container"))
        .subcommand(Command::new("remove").about("Remove the Pact Broker Docker container"))
}
fn add_standalone_broker_subcommand() -> Command {
    Command::new("standalone")
        .about(
            "Install & Run the Pact Broker with a bundled ruby runtime in $HOME/traveling-broker",
        )
        .subcommand(
            Command::new("start")
                .about("Download and Start the Pact Broker")
                .arg(
                    Arg::new("detach")
                        .short('d')
                        .long("detach")
                        .num_args(0)
                        .action(clap::ArgAction::SetTrue)
                        .help("Run the Pact Broker in the background"),
                ),
        )
        .subcommand(Command::new("stop").about("Stop the Pact Broker"))
}

fn add_plugin_cli_subcommand() -> Command {
    Command::new("plugin") 
    .arg_required_else_help(true)
    .about("CLI utility for Pact plugins")
    .arg(Arg::new("yes")
        .short('y')
        .long("yes")
        .num_args(0)
        .action(clap::ArgAction::SetTrue)
        .help("Automatically answer Yes for all prompts"))
    .arg(Arg::new("debug")
        .short('d')
        .long("debug")
        .num_args(0)
        .action(clap::ArgAction::SetTrue)
        .help("Enable debug level logs"))
    .arg(Arg::new("trace")
        .short('t')
        .long("trace")
        .num_args(0)
        .action(clap::ArgAction::SetTrue)
        .help("Enable trace level logs"))
    .arg(Arg::new("cli_version")
        .short('v')
        .long("version")
        .help("Print CLI version")
        .num_args(0))
    .subcommand(Command::new("list")
        .about("List the installed plugins")
        .arg_required_else_help(true)
        .subcommand(Command::new("installed")
            .about("List installed plugins"))
        .subcommand(Command::new("known")
            .about("List known plugins")
            .arg(Arg::new("show_all_versions")
                .short('a')
                .long("show-all-versions")
                .help("Display all versions of the known plugins")
                .action(clap::ArgAction::SetTrue)
            )
            ))
    .subcommand(Command::new("env")
        .about("Print out the Pact plugin environment config"))
    .subcommand(Command::new("install")
        .about("Install a plugin \n\nA plugin can be either installed from a URL, or for a known plugin, by name (and optionally version)")
        .arg_required_else_help(true)
        .arg(Arg::new("source_type")
            .short('t')
            .long("source-type")
            .num_args(1)
            .value_name("SOURCE_TYPE")
            .help("The type of source to fetch the plugin files from. Will default to Github releases.")
            .value_parser(clap::builder::PossibleValuesParser::new(&["github"])))
        .arg(Arg::new("yes")
            .short('y')
            .long("yes")
            .action(clap::ArgAction::SetTrue)
            .help("Automatically answer Yes for all prompts"))
        .arg(Arg::new("skip_if_installed")
            .long("skip-if-installed")
            .action(clap::ArgAction::SetTrue)
            .short('s')
            .help("Skip installing the plugin if the same version is already installed"))
        .arg(Arg::new("source")
            .help("Where to fetch the plugin files from. This should be a URL or the name of a known plugin.")
            .value_name("SOURCE")
            .required(true))
        .arg(Arg::new("version")
            .long("version")
            .short('v')
            .num_args(1)
            .help("The version to install. This is only used for known plugins.")
            .value_name("VERSION")))  
    .subcommand(Command::new("remove")
        .about("Remove a plugin")
        .arg(Arg::new("yes")
            .short('y')
            .long("yes")
            .action(clap::ArgAction::SetTrue)
            .help("Automatically answer Yes for all prompts"))
        .arg(Arg::new("name")
            .value_name("NAME")
            .required(true)
            .help("Plugin name"))
        .arg(Arg::new("version")
            .value_name("VERSION")
            // .value_parser(clap::builder::NonEmptyStringValueParser::new())
            .help("Plugin version. Not required if there is only one plugin version."))

    )
    .subcommand(Command::new("enable")
    .arg_required_else_help(true)
        .about("Enable a plugin version")
        .arg(Arg::new("name")
        .required(true)
        .help("Plugin name"))
    .arg(Arg::new("version")
        .help("Plugin version. Not required if there is only one plugin version.")
        .value_name("VERSION"))
    )
    .subcommand(Command::new("disable")
    .arg_required_else_help(true)
        .about("Disable a plugin version")
        .arg(Arg::new("name")
        .required(true)
        .help("Plugin name"))
    .arg(Arg::new("version")
        .help("Plugin version. Not required if there is only one plugin version.")
        .value_name("VERSION"))
    )
    .subcommand(Command::new("repository")
        .arg_required_else_help(true)
        .about("Sub-commands for dealing with a plugin repository")
        .subcommand(Command::new("validate")
        .about("Check the consistency of the repository index file")
        .arg(Arg::new("filename")
        .value_name("FILENAME")
        .required(true)
            .help("Filename to validate")))
    .subcommand(Command::new("new")
        .about("Create a new blank repository index file")
        .arg(Arg::new("filename")
        .value_name("FILENAME")
            .help("Filename to use for the new file. By default will use repository.index"))
        .arg(Arg::new("overwrite")
            .short('o')
            .long("overwrite")
            .num_args(0)
            .help(" Overwrite any existing file?"))
        )
        .subcommand(Command::new("add-plugin-version")
            .about("Add a plugin version to the index file (will update existing entry)")
            .arg_required_else_help(true)
            .subcommand_required(true)
        .subcommand(Command::new("file")
            .about("Add an entry for a local plugin manifest file to the repository file")
            .arg(Arg::new("repository_file")
            .value_name("REPOSITORY_FILE")
                .required(true)
                .help("Repository index file to update"))
            .arg(Arg::new("name")
            .value_name("FILE")
            .required(true)
                .help("Path to the local plugin manifest file")))
        .subcommand(Command::new("git-hub")
            .about("Add an entry for a GitHub Release to the repository file")
            .arg(Arg::new("repository_file")
            .value_name("REPOSITORY_FILE")
                .required(true)
                .help("Repository index file to update"))
            .arg(Arg::new("url")
            .value_name("URL")
            .required(true)
                .help("Base URL for GitHub APIs, will default to https://api.github.com/repos/")))
        )
        .subcommand(Command::new("add-all-plugin-versions")
        .about("Add all versions of a plugin to the index file (will update existing entries)")
        .arg(Arg::new("repository_file")
            .value_name("REPOSITORY_FILE")
            .required(true)
            .help("Repository index file to update"))
        .arg(Arg::new("owner")
            .value_name("OWNER")
            .required(true)
            .help("Repository owner to load versions from"))
        .arg(Arg::new("repository")
            .value_name("REPOSITORY")
            .required(true)
            .help("Repository to load versions from"))
        .arg(Arg::new("base_url")
            .value_name("BASE_URL")
            .help("Base URL for GitHub APIs, will default to https://api.github.com/repos/")))
    .subcommand(Command::new("yank-version")
        .about("Remove a plugin version from the index file"))
    .subcommand(Command::new("list")
        .about("List all plugins found in the index file")
        .arg(Arg::new("filename")
        .value_name("FILENAME")
            .required(true)
            .help("Filename to list entries from")))
    .subcommand(Command::new("list-versions")
        .about("List all plugin versions found in the index file")
        .arg(Arg::new("filename")
        .value_name("FILENAME")
            .required(true)
            .help("Filename to list versions from"))
            .arg(Arg::new("name")
            .value_name("NAME")
            .required(true)
            .help("Plugin entry to list versions for"))   ) 

    )
}

// determine if worth pulling out arguments
// grep -o 'Arg::new("[^"]*")' cli.rs | sort | uniq -c | sort -nr
//   10 Arg::new("pacticipant")
//   8 Arg::new("version")
//   5 Arg::new("uuid")
//   5 Arg::new("branch")
//   4 Arg::new("tag")
//   4 Arg::new("name")
//   4 Arg::new("environment")
//   3 Arg::new("provider")
//   3 Arg::new("display-name")

fn add_examples_subcommand() -> Command {
    Command::new("examples")
        .about("download example projects")
        .arg(
            Arg::new("type")
                .short('t')
                .long("type")
                .num_args(1)
                .value_parser(clap::builder::PossibleValuesParser::new(&[
                    "bdct",
                    "cdct",
                    "workshops",
                ]))
                .required(true)
                .help("Specify the project type (bdct, cdct, workshops)"),
        )
        .arg(
            Arg::new("project")
                .short('p')
                .long("project")
                .num_args(1)
                .help("Specify the project to download"),
        )
        .arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .help("Download all projects")
                .action(clap::ArgAction::SetTrue),
        )
}

fn add_project_subcommand() -> Command {
    Command::new("project")
        .about("Pact project actions for setting up and managing pact projects")
        .subcommand(
            Command::new("install").about("install pact").arg(
                Arg::new("language")
                    .short('l')
                    .long("language")
                    .num_args(1)
                    .value_parser(clap::builder::PossibleValuesParser::new(&[
                        "js", "golang", "ruby", "python", "java", ".net", "rust", "php",
                    ]))
                    .required(true)
                    .help("Specify the language to install pact for"),
            ),
        )
        .subcommand(
            Command::new("new").about("create new pact project").arg(
                Arg::new("language")
                    .short('l')
                    .long("language")
                    .num_args(1)
                    .value_parser(clap::builder::PossibleValuesParser::new(&[
                        "js", "golang", "ruby", "python", "java", ".net", "rust", "php",
                    ]))
                    .required(true)
                    .help("Specify the language for the new pact project"),
            ),
        )
        .subcommand(
            Command::new("link").about("link pact project").arg(
                Arg::new("language")
                    .short('l')
                    .long("language")
                    .num_args(1)
                    .value_parser(clap::builder::PossibleValuesParser::new(&[
                        "js", "golang", "ruby", "python", "java", ".net", "rust", "php",
                    ]))
                    .required(true)
                    .help("Specify the language of the pact project to link"),
            ),
        )
        .subcommand(
            Command::new("issue").about("create pact issue").arg(
                Arg::new("language")
                    .short('l')
                    .long("language")
                    .num_args(1)
                    .value_parser(clap::builder::PossibleValuesParser::new(&[
                        "js", "golang", "ruby", "python", "java", ".net", "rust", "php",
                    ]))
                    .required(true)
                    .help("Specify the language for creating the pact issue"),
            ),
        )
        .subcommand(
            Command::new("docs").about("open pact documentation").arg(
                Arg::new("language")
                    .short('l')
                    .long("language")
                    .num_args(1)
                    .value_parser(clap::builder::PossibleValuesParser::new(&[
                        "js", "golang", "ruby", "python", "java", ".net", "rust", "php",
                    ]))
                    .required(true)
                    .help("Specify the language for opening the pact documentation"),
            ),
        )
}

fn add_broker_auth_arguments() -> Vec<Arg> {
    vec![
        Arg::new("broker-base-url")
            .short('b')
            .long("broker-base-url")
            .num_args(1)
            .help("The base URL of the Pact Broker")
            .required(true)
            .value_name("PACT_BROKER_BASE_URL")
            .env("PACT_BROKER_BASE_URL"),
        Arg::new("broker-username")
            .short('u')
            .long("broker-username")
            .num_args(1)
            .help("Pact Broker basic auth username")
            .value_name("PACT_BROKER_USERNAME")
            .env("PACT_BROKER_USERNAME"),
        Arg::new("broker-password")
            .short('p')
            .long("broker-password")
            .num_args(1)
            .help("Pact Broker basic auth password")
            .value_name("PACT_BROKER_PASSWORD")
            .env("PACT_BROKER_PASSWORD"),
        Arg::new("broker-token")
            .short('k')
            .long("broker-token")
            .num_args(1)
            .help("Pact Broker bearer token")
            .value_name("PACT_BROKER_TOKEN")
            .env("PACT_BROKER_TOKEN"),
    ]
}
fn add_output_arguments(
    value_parser_args: Vec<&'static str>,
    default_value: &'static str,
) -> Vec<Arg> {
    vec![Arg::new("output")
        .short('o')
        .long("output")
        .value_name("OUTPUT")
        .value_parser(clap::builder::PossibleValuesParser::new(&value_parser_args))
        .default_value(default_value) // Fix: Remove the borrow operator
        .value_name("OUTPUT")
        .help(format!("Value must be one of {:?}", value_parser_args))]
}

fn add_verbose_arguments() -> Vec<Arg> {
    vec![Arg::new("verbose")
        .short('v')
        .long("verbose")
        .num_args(0)
        .help("Verbose output.")]
}

fn add_publish_pacts_subcommand() -> Command {
    Command::new("publish")
    .args(add_broker_auth_arguments())
    .about("Publishes pacts to the Pact Broker")
    .arg(Arg::new("PACT_DIRS_OR_FILES")
        .num_args(0..=1)
        .required(true)
        .help("Pact directories or files"))
    .arg(Arg::new("consumer-app-version")
        .short('a')
        .long("consumer-app-version")
        .value_parser(clap::builder::NonEmptyStringValueParser::new())
        .help("The consumer application version"))
    .arg(Arg::new("branch")
        // .short('h')
        .long("branch")
        .value_parser(clap::builder::NonEmptyStringValueParser::new())
        .help("Repository branch of the consumer version"))
    .arg(Arg::new("auto-detect-version-properties")
        .short('r')
        .long("auto-detect-version-properties")
        .num_args(0)
        .help("Automatically detect the repository commit, branch and build URL from known CI environment variables or git CLI. Supports Buildkite, Circle CI, Travis CI, GitHub Actions, Jenkins, Hudson, AppVeyor, GitLab, CodeShip, Bitbucket and Azure DevOps."))
    .arg(Arg::new("tag")
        .short('t')
        .long("tag")
        .num_args(0..=1)
        .help("Tag name for consumer version. Can be specified multiple times."))
    .arg(Arg::new("tag-with-git-branch")
        .short('g')
        .long("tag-with-git-branch")
        .num_args(0)
        .help("Tag consumer version with the name of the current git branch. Supports Buildkite, Circle CI, Travis CI, GitHub Actions, Jenkins, Hudson, AppVeyor, GitLab, CodeShip, Bitbucket and Azure DevOps."))
    .arg(Arg::new("build-url")
        .long("build-url")
        .num_args(1)
        .help("The build URL that created the pact"))
    .arg(Arg::new("merge")
        .long("merge")
        .num_args(0)
        .help("If a pact already exists for this consumer version and provider, merge the contents. Useful when running Pact tests concurrently on different build nodes."))
        .args(add_output_arguments(["json", "text"].to_vec(),"text"))
.args(add_verbose_arguments())
}

fn add_list_latest_pact_versions_subcommand() -> Command {
    Command::new("list-latest-pact-versions")
        .about("List the latest pact for each integration")
        .args(add_broker_auth_arguments())
        .args(add_verbose_arguments())
        .args(add_output_arguments(["json", "table"].to_vec(), "table"))
}
fn add_create_environment_subcommand() -> Command {
    Command::new("create-environment")
    .about("Create an environment resource in the Pact Broker to represent a real world deployment or release environment")
    .arg(Arg::new("name")
        .long("name")
        .value_name("NAME")
        .required(true)
        .help("The uniquely identifying name of the environment as used in deployment code"))
    .arg(Arg::new("display-name")
        .long("display-name")
        .value_name("DISPLAY_NAME")
        .help("The display name of the environment"))
    .arg(Arg::new("production")
        .long("production")
        .action(clap::ArgAction::SetTrue)
        .help("Whether or not this environment is a production environment. This is currently informational only."))
    .arg(Arg::new("contact-name")
        .long("contact-name")
        .value_name("CONTACT_NAME")
        .help("The name of the team/person responsible for this environment"))
    .arg(Arg::new("contact-email-address")
        .long("contact-email-address")
        .value_name("CONTACT_EMAIL_ADDRESS")
        .help("The email address of the team/person responsible for this environment"))
        .args(add_output_arguments(["json", "text", "id"].to_vec(), "text"))

.args(add_broker_auth_arguments())
.args(add_verbose_arguments())
}
fn add_update_environment_subcommand() -> Command {
    Command::new("update-environment")
    .about("Update an environment resource in the Pact Broker")
    .arg(Arg::new("uuid")
        .long("uuid")
        .value_name("UUID")
        .required(true)
        .help("The UUID of the environment to update"))
    .arg(Arg::new("name")
        .long("name")
        .value_name("NAME")
        .help("The uniquely identifying name of the environment as used in deployment code"))
    .arg(Arg::new("display-name")
        .long("display-name")
        .value_name("DISPLAY_NAME")
        .help("The display name of the environment"))
    .arg(Arg::new("production")
        .long("production")
        .action(clap::ArgAction::SetTrue)
        .help("Whether or not this environment is a production environment. This is currently informational only."))
    .arg(Arg::new("contact-name")
        .long("contact-name")
        .value_name("CONTACT_NAME")
        .help("The name of the team/person responsible for this environment"))
    .arg(Arg::new("contact-email-address")
        .long("contact-email-address")
        .value_name("CONTACT_EMAIL_ADDRESS")
        .help("The email address of the team/person responsible for this environment"))
        .args(add_output_arguments(["json", "text", "id"].to_vec(), "text"))
.args(add_broker_auth_arguments())
.args(add_verbose_arguments())
}
fn add_describe_environment_subcommand() -> Command {
    Command::new("describe-environment")
        .about("Describe an environment")
        .arg(
            Arg::new("uuid")
                .long("uuid")
                .value_name("UUID")
                .required(true)
                .help("The UUID of the environment to describe"),
        )
        .args(add_output_arguments(["json", "text"].to_vec(), "text"))
        .args(add_broker_auth_arguments())
        .args(add_verbose_arguments())
}
fn add_delete_environment_subcommand() -> Command {
    Command::new("delete-environment")
        .about("Delete an environment")
        .arg(
            Arg::new("uuid")
                .long("uuid")
                .value_name("UUID")
                .required(true)
                .help("The UUID of the environment to delete"),
        )
        // .args(add_output_arguments(["json", "text"].to_vec(), "text"))
        .args(add_broker_auth_arguments())
        .args(add_verbose_arguments())
}

fn add_list_environments_subcommand() -> Command {
    Command::new("list-environments")
        .about("List environments")
        .args(add_output_arguments(
            ["json", "text", "pretty"].to_vec(),
            "text",
        ))
        .args(add_broker_auth_arguments())
        .args(add_verbose_arguments())
}
fn add_record_deployment_subcommand() -> Command {
    Command::new("record-deployment")
    .about("Record deployment of a pacticipant version to an environment")
    .arg(Arg::new("pacticipant")
        .short('a')
        .long("pacticipant")
        .value_name("PACTICIPANT")
        .value_parser(clap::builder::NonEmptyStringValueParser::new())
        .required(true)
        .help("The name of the pacticipant that was deployed"))
    .arg(Arg::new("version")
        .short('e')
        .long("version")
        .value_name("VERSION")
        .value_parser(clap::builder::NonEmptyStringValueParser::new())
        .required(true)
        .help("The pacticipant version number that was deployed"))
    .arg(Arg::new("environment")
        .long("environment")
        .value_name("ENVIRONMENT")
        .value_parser(clap::builder::NonEmptyStringValueParser::new())
        .required(true)
        .help("The name of the environment that the pacticipant version was deployed to"))
    .arg(Arg::new("application-instance")
        .long("application-instance")
        .value_name("APPLICATION_INSTANCE")
        .alias("target")
        .value_parser(clap::builder::NonEmptyStringValueParser::new())
        .help("Optional. The application instance to which the deployment has occurred - a logical identifer required to differentiate deployments when there are multiple instances of the same application in an environment. This field was called 'target' in a beta release"))
    .args(add_output_arguments(
        ["json", "text", "pretty"].to_vec(),
        "text",
    ))

.args(add_broker_auth_arguments())
.args(add_verbose_arguments())
}
fn add_record_undeployment_subcommand() -> Command {
    Command::new("record-undeployment")
    .about("Record undeployment of a pacticipant version from an environment")
    .long_about("Record undeployment of a pacticipant version from an environment.\n\nNote that use of this command is only required if you are permanently removing an application instance from an environment. It is not required if you are deploying over a previous version, as record-deployment will automatically mark the previously deployed version as undeployed for you. See https://docs.pact.io/go/record-undeployment for more information.")
    .arg(Arg::new("pacticipant")
        .short('a')
        .long("pacticipant")
        .value_name("PACTICIPANT")
        .value_parser(clap::builder::NonEmptyStringValueParser::new())
        .required(true)
        .help("The name of the pacticipant that was undeployed"))
    .arg(Arg::new("environment")
        .long("environment")
       .value_name("ENVIRONMENT")
        .value_parser(clap::builder::NonEmptyStringValueParser::new())
        .required(true)
        .help("The name of the environment that the pacticipant version was undeployed from"))
    .arg(Arg::new("application-instance")
        .long("application-instance")
        .alias("target")
        .value_name("APPLICATION_INSTANCE")
        .value_parser(clap::builder::NonEmptyStringValueParser::new())
        .help("Optional. The application instance from which the application is being undeployed - a logical identifer required to differentiate deployments when there are multiple instances of the same application in an environment. This field was called 'target' in a beta release"))

    .args(add_broker_auth_arguments())
    .args(add_verbose_arguments())
    .args(add_output_arguments(
        ["json", "text", "pretty"].to_vec(),
        "text",
    ))
}

fn add_record_release_subcommand() -> Command {
    Command::new("record-release")
        .about("Record release of a pacticipant version to an environment.")
        .arg(
            Arg::new("pacticipant")
                .short('a')
                .long("pacticipant")
                .value_name("PACTICIPANT")
                .required(true)
                .help("The name of the pacticipant that was released."),
        )
        .arg(
            Arg::new("version")
                .short('e')
                .long("version")
                .value_name("VERSION")
                .required(true)
                .help("The pacticipant version number that was released."),
        )
        .arg(
            Arg::new("environment")
                .long("environment")
                .value_name("ENVIRONMENT")
                .required(true)
                .help("The name of the environment that the pacticipant version was released to."),
        )
        .args(add_output_arguments(
            ["json", "text", "pretty"].to_vec(),
            "text",
        ))
        .args(add_broker_auth_arguments())
        .args(add_verbose_arguments())
}
fn add_record_support_ended_subcommand() -> Command {
    Command::new("record-support-ended")
        .about("Record the end of support for a pacticipant version in an environment.")
        .arg(
            Arg::new("pacticipant")
                .short('a')
                .long("pacticipant")
                .value_name("PACTICIPANT")
                .required(true)
                .help("The name of the pacticipant."),
        )
        .arg(
            Arg::new("version")
                .short('e')
                .long("version")
                .value_name("VERSION")
                .required(true)
                .help("The pacticipant version number for which support is ended."),
        )
        .arg(
            Arg::new("environment")
                .long("environment")
                .value_name("ENVIRONMENT")
                .required(true)
                .help("The name of the environment in which the support is ended."),
        )
        .args(add_output_arguments(
            ["json", "text", "pretty"].to_vec(),
            "text",
        ))
        .args(add_broker_auth_arguments())
        .args(add_verbose_arguments())
}
fn add_can_i_deploy_subcommand() -> Command {
    Command::new("can-i-deploy")
    .about("Check if a pacticipant can be deployed.")
    .long_about(
    r"
    Check if a pacticipant can be deployed.

    Description:
    Returns exit code 0 or 1, indicating whether or not the specified application (pacticipant) has a successful verification result with
    each of the application versions that are already deployed to a particular environment. Prints out the relevant pact/verification
    details, indicating any missing or failed verification results.
  
    The can-i-deploy tool was originally written to support specifying versions and dependencies using tags. This usage has now been
    superseded by first class support for environments, deployments and releases. For documentation on how to use can-i-deploy with tags,
    please see https://docs.pact.io/pact_broker/client_cli/can_i_deploy_usage_with_tags/
  
    Before `can-i-deploy` can be used, the relevant environment resources must first be created in the Pact Broker using the
    `create-environment` command. The 'test' and 'production' environments will have been seeded for you. You can check the existing
    environments by running `pact-broker list-environments`. See https://docs.pact.io/pact_broker/client_cli/readme#environments for more
    information.
  
    $ pact-broker create-environment --name 'uat' --display-name 'UAT' --no-production
  
    After an application is deployed or released, its deployment must be recorded using the `ecord-deployment` or `ecord-release`
    commands. See https://docs.pact.io/pact_broker/recording_deployments_and_releases/ for more information.
  
    $ pact-broker record-deployment --pacticipant Foo --version 173153ae0 --environment uat
  
    Before an application is deployed or released to an environment, the can-i-deploy command must be run to check that the application
    version is safe to deploy with the versions of each integrated application that are already in that environment.
  
    $ pact-broker can-i-deploy --pacticipant PACTICIPANT --version VERSION --to-environment ENVIRONMENT
  
    Example: can I deploy version 173153ae0 of application Foo to the test environment?
  
    $ pact-broker can-i-deploy --pacticipant Foo --version 173153ae0 --to-environment test
  
    Can-i-deploy can also be used to check if arbitrary versions have a successful verification. When asking 'Can I deploy this
    application version with the latest version from the main branch of another application' it functions as a 'can I merge' check.
  
    $ pact-broker can-i-deploy --pacticipant Foo 173153ae0 \\ --pacticipant Bar --latest main
  
    ##### Polling
  
    If the verification process takes a long time and there are results missing when the can-i-deploy command runs in your CI/CD pipeline,
    you can configure the command to poll and wait for the missing results to arrive. The arguments to specify are `--retry-while-unknown
    TIMES` and `--retry-interval SECONDS`, set to appropriate values for your pipeline.
    "
    )
    .arg(Arg::new("pacticipant")
        .short('a')
        .long("pacticipant")
        .value_name("PACTICIPANT")
        .required(true)
        .num_args(0..=1)
        .help("The pacticipant name. Use once for each pacticipant being checked."))
    .arg(Arg::new("version")
        .short('e')
        .long("version")
        .value_name("VERSION")
        .help("The pacticipant version. Must be entered after the --pacticipant that it relates to."))
    .arg(Arg::new("ignore")
        .long("ignore")
        .num_args(0)
        .action(clap::ArgAction::SetTrue)
        .help("The pacticipant name to ignore. Use once for each pacticipant being ignored. A specific version can be ignored by also specifying a --version after the pacticipant name option. The environment variable PACT_BROKER_CAN_I_DEPLOY_IGNORE may also be used to specify a pacticipant name to ignore, with commas to separate multiple pacticipant names if necessary."))
    .arg(Arg::new("latest")
        .short('l')
        .long("latest")
        .num_args(0)
        .action(clap::ArgAction::SetTrue)
        .value_name("LATEST")
        .help("Use the latest pacticipant version. Optionally specify a TAG to use the latest version with the specified tag."))
    .arg(Arg::new("branch")
        .long("branch")
        .value_name("BRANCH")
        .help("The branch of the version for which you want to check the verification results."))
    .arg(Arg::new("main-branch")
        .long("main-branch")
        .num_args(0)
        .action(clap::ArgAction::SetTrue)
        .conflicts_with_all(&["no-main-branch", "skip-main-branch"])
        .help("Use the latest version of the configured main branch of the pacticipant as the version for which you want to check the verification results"))
    .arg(Arg::new("no-main-branch")
        .long("no-main-branch")
        .num_args(0)
        .action(clap::ArgAction::SetTrue)
        .conflicts_with_all(&["main-branch", "skip-main-branch"])
        .help("No main branch of the pacticipant as the version for which you want to check the verification results"))
    .arg(Arg::new("skip-main-branch")
        .long("skip-main-branch")
        .num_args(0)
        .action(clap::ArgAction::SetTrue)
        .conflicts_with_all(&["main-branch", "no-main-branch"])
        .help("Skip the configured main branch of the pacticipant as the version for which you want to check the verification results"))
    .arg(Arg::new("to-environment")
        .long("to-environment")
        .value_name("ENVIRONMENT")
        .help("The environment into which the pacticipant(s) are to be deployed"))
    .arg(Arg::new("to")
        .long("to")
        .value_name("TO")
        .help("The tag that represents the branch or environment of the integrated applications for which you want to check the verification result status."))
        .args(add_output_arguments(["json", "table"].to_vec(), "table"))
    .arg(Arg::new("retry-while-unknown")
        .long("retry-while-unknown")
        .value_name("TIMES")
        .help("The number of times to retry while there is an unknown verification result (ie. the provider verification is likely still running)"))
    .arg(Arg::new("retry-interval")
        .long("retry-interval")
        .value_name("SECONDS")
        .help("The time between retries in seconds. Use in conjuction with --retry-while-unknown"))
    .arg(Arg::new("dry-run")
        .long("dry-run")
        .num_args(0)
        .conflicts_with_all(&["skip-dry-run", "no-dry-run"])
        .action(clap::ArgAction::SetTrue)
        .help("When dry-run is enabled, always exit process with a success code. Can also be enabled by setting the environment variable PACT_BROKER_CAN_I_DEPLOY_DRY_RUN=true. This mode is useful when setting up your CI/CD pipeline for the first time, or in a 'break glass' situation where you need to knowingly deploy what Pact considers a breaking change. For the second scenario, it is recommended to use the environment variable and just set it for the build required to deploy that particular version, so you don't accidentally leave the dry run mode enabled."))
    .arg(Arg::new("no-dry-run")
        .long("no-dry-run")
        .num_args(0)
        .action(clap::ArgAction::SetTrue)
        .conflicts_with_all(&["skip-dry-run", "dry-run"])
        .help("When dry-run is enabled, always exit process with a success code. Can also be enabled by setting the environment variable PACT_BROKER_CAN_I_DEPLOY_DRY_RUN=true. This mode is useful when setting up your CI/CD pipeline for the first time, or in a 'break glass' situation where you need to knowingly deploy what Pact considers a breaking change. For the second scenario, it is recommended to use the environment variable and just set it for the build required to deploy that particular version, so you don't accidentally leave the dry run mode enabled."))
    .arg(Arg::new("skip-dry-run")
        .long("skip-dry-run")
        .num_args(0)
        .action(clap::ArgAction::SetTrue)
        .conflicts_with_all(&["no-dry-run", "dry-run"])
        .help("When dry-run is enabled, always exit process with a success code. Can also be enabled by setting the environment variable PACT_BROKER_CAN_I_DEPLOY_DRY_RUN=true. This mode is useful when setting up your CI/CD pipeline for the first time, or in a 'break glass' situation where you need to knowingly deploy what Pact considers a breaking change. For the second scenario, it is recommended to use the environment variable and just set it for the build required to deploy that particular version, so you don't accidentally leave the dry run mode enabled."))

.args(add_broker_auth_arguments())
.args(add_verbose_arguments())
}
fn add_can_i_merge_subcommand() -> Command {
    Command::new("can-i-merge")
    .about("Checks if the specified pacticipant version is compatible with the configured main branch of each of the pacticipants with which it is integrated.")
    .args(add_broker_auth_arguments())
    .arg(Arg::new("pacticipant")
        .short('a')
        .long("pacticipant")
        .value_name("PACTICIPANT")
        .required(true)
        .num_args(0..=1)
        .help("The pacticipant name. Use once for each pacticipant being checked."))
    .arg(Arg::new("version")
        .short('e')
        .long("version")
        .value_name("VERSION")
        .help("The pacticipant version. Must be entered after the --pacticipant that it relates to."))
        .args(add_output_arguments(["json", "table"].to_vec(), "table"))
    .arg(Arg::new("retry-while-unknown")
        .long("retry-while-unknown")
        .value_name("TIMES")
        .default_value("0")
        .help("The number of times to retry while there is an unknown verification result (ie. the provider verification is likely still running)"))
    .arg(Arg::new("retry-interval")
        .long("retry-interval")
        .value_name("SECONDS")
        .default_value("10")
        .help("The time between retries in seconds. Use in conjuction with --retry-while-unknown"))
    .arg(Arg::new("dry-run")
        .long("dry-run")
        .help("When dry-run is enabled, always exit process with a success code. Can also be enabled by setting the environment variable PACT_BROKER_CAN_I_MERGE_DRY_RUN=true. This mode is useful when setting up your CI/CD pipeline for the first time, or in a 'break glass' situation where you need to knowingly deploy what Pact considers a breaking change. For the second scenario, it is recommended to use the environment variable and just set it for the build required to deploy that particular version, so you don't accidentally leave the dry run mode enabled."))

.args(add_verbose_arguments())
}
fn add_create_or_update_pacticipant_subcommand() -> Command {
    Command::new("create-or-update-pacticipant")
        .about("Create or update pacticipant by name")
        .args(add_broker_auth_arguments())
        .arg(
            Arg::new("name")
                .long("name")
                .value_name("NAME")
                .required(true)
                .help("Pacticipant name"),
        )
        .arg(
            Arg::new("display-name")
                .long("display-name")
                .value_name("DISPLAY_NAME")
                .help("Display name"),
        )
        .arg(
            Arg::new("main-branch")
                .long("main-branch")
                .value_name("MAIN_BRANCH")
                .help("The main development branch of the pacticipant repository"),
        )
        .arg(
            Arg::new("repository-url")
                .long("repository-url")
                .value_name("REPOSITORY_URL")
                .help("The repository URL of the pacticipant"),
        )
        .args(add_output_arguments(["json", "text"].to_vec(), "text"))
        .args(add_verbose_arguments())
}
fn add_describe_pacticipant_subcommand() -> Command {
    Command::new("describe-pacticipant")
        .about("Describe a pacticipant")
        .args(add_broker_auth_arguments())
        .arg(
            Arg::new("name")
                .long("name")
                .value_name("NAME")
                .required(true)
                .help("Pacticipant name"),
        )
        .args(add_output_arguments(["json", "text"].to_vec(), "text"))
        .args(add_verbose_arguments())
}
fn add_list_pacticipants_subcommand() -> Command {
    Command::new("list-pacticipants")
        .about("List pacticipants")
        .args(add_broker_auth_arguments())
        .args(add_output_arguments(["json", "text"].to_vec(), "text"))
        .args(add_verbose_arguments())
}
fn add_create_webhook_subcommand() -> Command {
    Command::new("create-webhook")
    .about("Create a webhook")
    .arg(Arg::new("url")
        .value_name("URL")
        .required(true)
        .help("Webhook URL"))
    .arg(Arg::new("request")
        .short('X')
        .long("request")
        .value_name("METHOD")
        .help("Webhook HTTP method"))
    .arg(Arg::new("header")
        .short('H')
        .long("header")
        .value_name("one two three")
        .num_args(0..=1)
        .help("Webhook Header"))
    .arg(Arg::new("data")
        .short('d')
        .long("data")
        .value_name("DATA")
        .help("Webhook payload"))
    .arg(Arg::new("user")
        // .short('u')
        .long("user")
        .value_name("USER")
        .help("Webhook basic auth username and password eg. username:password"))
    .arg(Arg::new("consumer")
        .long("consumer")
        .value_name("CONSUMER")
        .help("Consumer name"))
    .arg(Arg::new("consumer-label")
        .long("consumer-label")
        .value_name("CONSUMER_LABEL")
        .help("Consumer label, mutually exclusive with consumer name"))
    .arg(Arg::new("provider")
        .long("provider")
        .value_name("PROVIDER")
        .help("Provider name"))
    .arg(Arg::new("provider-label")
        .long("provider-label")
        .value_name("PROVIDER_LABEL")
        .help("Provider label, mutually exclusive with provider name"))
    .arg(Arg::new("description")
        .long("description")
        .value_name("DESCRIPTION")
        .help("Webhook description"))
    .arg(Arg::new("contract-content-changed")
        .long("contract-content-changed")
        .help("Trigger this webhook when the pact content changes"))
    .arg(Arg::new("contract-published")
        .long("contract-published")
        .help("Trigger this webhook when a pact is published"))
    .arg(Arg::new("provider-verification-published")
        .long("provider-verification-published")
        .help("Trigger this webhook when a provider verification result is published"))
    .arg(Arg::new("provider-verification-failed")
        .long("provider-verification-failed")
        .help("Trigger this webhook when a failed provider verification result is published"))
    .arg(Arg::new("provider-verification-succeeded")
        .long("provider-verification-succeeded")
        .help("Trigger this webhook when a successful provider verification result is published"))
    .arg(Arg::new("contract-requiring-verification-published")
        .long("contract-requiring-verification-published")
        .help("Trigger this webhook when a contract is published that requires verification"))
    .arg(Arg::new("team-uuid")
        .long("team-uuid")
        .value_name("UUID")
        .help("UUID of the PactFlow team to which the webhook should be assigned (PactFlow only)"))

.args(add_broker_auth_arguments())
.args(add_verbose_arguments())
}
fn add_create_or_update_webhook_subcommand() -> Command {
    Command::new("create-or-update-webhook")
    .about("Create or update a webhook")
    .args(add_broker_auth_arguments())
    .arg(Arg::new("url")
        .value_name("URL")
        .required(true)
        .help("Webhook URL"))
    .arg(Arg::new("uuid")
        .long("uuid")
        .value_name("UUID")
        .required(true)
        .help("Specify the uuid for the webhook"))
    .arg(Arg::new("request")
        .short('X')
        .long("request")
        .value_name("METHOD")
        .help("Webhook HTTP method"))
    .arg(Arg::new("header")
        .short('H')
        .long("header")
        .value_name("one two three")
        .num_args(0..=1)
        .help("Webhook Header"))
    .arg(Arg::new("data")
        .short('d')
        .long("data")
        .value_name("DATA")
        .help("Webhook payload"))
    .arg(Arg::new("user")
        // .short('u')
        .long("user")
        .value_name("USER")
        .help("Webhook basic auth username and password eg. username:password"))
    .arg(Arg::new("consumer")
        .long("consumer")
        .value_name("CONSUMER")
        .help("Consumer name"))
    .arg(Arg::new("consumer-label")
        .long("consumer-label")
        .value_name("CONSUMER_LABEL")
        .help("Consumer label, mutually exclusive with consumer name"))
    .arg(Arg::new("provider")
        .long("provider")
        .value_name("PROVIDER")
        .help("Provider name"))
    .arg(Arg::new("provider-label")
        .long("provider-label")
        .value_name("PROVIDER_LABEL")
        .help("Provider label, mutually exclusive with provider name"))
    .arg(Arg::new("description")
        .long("description")
        .value_name("DESCRIPTION")
        .help("Webhook description"))
    .arg(Arg::new("contract-content-changed")
        .long("contract-content-changed")
        .help("Trigger this webhook when the pact content changes"))
    .arg(Arg::new("contract-published")
        .long("contract-published")
        .help("Trigger this webhook when a pact is published"))
    .arg(Arg::new("provider-verification-published")
        .long("provider-verification-published")
        .help("Trigger this webhook when a provider verification result is published"))
    .arg(Arg::new("provider-verification-failed")
        .long("provider-verification-failed")
        .help("Trigger this webhook when a failed provider verification result is published"))
    .arg(Arg::new("provider-verification-succeeded")
        .long("provider-verification-succeeded")
        .help("Trigger this webhook when a successful provider verification result is published"))
    .arg(Arg::new("contract-requiring-verification-published")
        .long("contract-requiring-verification-published")
        .help("Trigger this webhook when a contract is published that requires verification"))
    .arg(Arg::new("team-uuid")
        .long("team-uuid")
        .value_name("UUID")
        .help("UUID of the PactFlow team to which the webhook should be assigned (PactFlow only)"))
        .args(add_verbose_arguments())
}
fn add_test_webhook_subcommand() -> Command {
    Command::new("test-webhook")
        .about("Test a webhook")
        .arg(
            Arg::new("uuid")
                .long("uuid")
                .value_name("UUID")
                .num_args(1)
                .required(true)
                .help("Specify the uuid for the webhook"),
        )
        .args(add_broker_auth_arguments())
        .args(add_verbose_arguments())
}
fn add_delete_branch_subcommand() -> Command {
    Command::new("delete-branch")
    .about("Deletes a pacticipant branch. Does not delete the versions or pacts/verifications associated with the branch, but does make the pacts inaccessible for verification via consumer versions selectors or WIP pacts.")
    .args(add_broker_auth_arguments())
    .arg(Arg::new("branch")
        .long("branch")
        .value_name("BRANCH")
        .required(true)
        .help("The pacticipant branch name"))
    .arg(Arg::new("pacticipant")
        .short('a')
        .long("pacticipant")
        .value_name("PACTICIPANT")
        .required(true)
        .help("The name of the pacticipant that the branch belongs to"))
.args(add_verbose_arguments())
}
fn add_create_version_tag_subcommand() -> Command {
    Command::new("create-version-tag")
        .about("Add a tag to a pacticipant version")
        .arg(
            Arg::new("pacticipant")
                .short('a')
                .long("pacticipant")
                .value_name("PACTICIPANT")
                .required(true)
                .help("The pacticipant name"),
        )
        .arg(
            Arg::new("version")
                .short('e')
                .long("version")
                .value_name("VERSION")
                .required(true)
                .help("The pacticipant version"),
        )
        .arg(
            Arg::new("tag")
                .short('t')
                .long("tag")
                .value_name("TAG")
                .num_args(0..=1)
                .help("Tag name for pacticipant version. Can be specified multiple times"),
        )
        .arg(
            Arg::new("auto-create-version")
                .long("auto-create-version")
                .help("Automatically create the pacticipant version if it does not exist"),
        )
        .arg(
            Arg::new("tag-with-git-branch")
                .short('g')
                .long("tag-with-git-branch")
                .help("Tag pacticipant version with the name of the current git branch"),
        )
}
fn add_describe_version_subcommand() -> Command {
    Command::new("describe-version")
    .about("Describes a pacticipant version. If no version or tag is specified, the latest version is described.")
    .arg(Arg::new("pacticipant")
        .short('a')
        .long("pacticipant")
        .value_name("PACTICIPANT")
        .required(true)
        .help("The name of the pacticipant that the version belongs to"))
    .arg(Arg::new("version")
        .short('e')
        .long("version")
        .value_name("VERSION")
        .help("The pacticipant version number"))
    .arg(Arg::new("latest")
        .short('l')
        .long("latest")
        .value_name("TAG")
        .help("Describe the latest pacticipant version. Optionally specify a TAG to describe the latest version with the specified tag"))
        .args(add_output_arguments(["json", "table", "id"].to_vec(), "table"))
}
fn add_create_or_update_version_subcommand() -> Command {
    Command::new("create-or-update-version")
        .about("Create or update pacticipant version by version number")
        .arg(
            Arg::new("pacticipant")
                .short('a')
                .long("pacticipant")
                .value_name("PACTICIPANT")
                .required(true)
                .help("The pacticipant name"),
        )
        .arg(
            Arg::new("version")
                .short('e')
                .long("version")
                .value_name("VERSION")
                .required(true)
                .help("The pacticipant version number"),
        )
        .arg(
            Arg::new("branch")
                .long("branch")
                .value_name("BRANCH")
                .help("The repository branch name"),
        )
        .arg(
            Arg::new("tag")
                .short('t')
                .long("tag")
                .value_name("TAG")
                .num_args(0..=1)
                .help("Tag name for pacticipant version. Can be specified multiple times"),
        )
        .args(add_output_arguments(["json", "text"].to_vec(), "text"))
}
fn add_generate_uuid_subcommand() -> Command {
    Command::new("generate-uuid")
        .about("Generate a UUID for use when calling create-or-update-webhook")
}

fn add_publish_provider_contract_subcommand() -> Command {
    Command::new("publish-provider-contract")
    .about("Publish provider contract to PactFlow")
    .args(add_broker_auth_arguments())
    .arg(Arg::new("contract-file")
        .num_args(1)
        .value_name("CONTRACT_FILE")
        .required(true)
        .help("The contract file(s)"))
    .arg(Arg::new("provider")
        .long("provider")
        .value_name("PROVIDER")
        .help("The provider name"))
    .arg(Arg::new("provider-app-version")
        .short('a')
        .long("provider-app-version")
        .value_name("PROVIDER_APP_VERSION")
        .required(true)
        .help("The provider application version"))
    .arg(Arg::new("branch")
        // .short('h')
        .long("branch")
        .value_name("BRANCH")
        .help("Repository branch of the provider version"))
    .arg(Arg::new("tag")
        .short('t')
        .long("tag")
        .value_name("TAG")
        .num_args(0..=1)
        .help("Tag name for provider version. Can be specified multiple times."))
    .arg(Arg::new("specification")
        .long("specification")
        .value_name("SPECIFICATION")
        .default_value("oas")
        .help("The contract specification"))
    .arg(Arg::new("content-type")
        .long("content-type")
        .value_name("CONTENT_TYPE")
        .help("The content type. eg. application/yml"))
    .arg(Arg::new("verification-success")
        .long("verification-success")
        .help("Whether or not the self verification passed successfully."))
    .arg(Arg::new("verification-exit-code")
        .long("verification-exit-code")
        .value_name("N")
        .help("The exit code of the verification process. Can be used instead of --verification-success|--no-verification-success for a simpler build script."))
    .arg(Arg::new("verification-results")
        .long("verification-results")
        .value_name("VERIFICATION_RESULTS")
        .help("The path to the file containing the output from the verification process"))
    .arg(Arg::new("verification-results-content-type")
        .long("verification-results-content-type")
        .value_name("VERIFICATION_RESULTS_CONTENT_TYPE")
        .help("The content type of the verification output eg. text/plain, application/yaml"))
    .arg(Arg::new("verification-results-format")
        .long("verification-results-format")
        .value_name("VERIFICATION_RESULTS_FORMAT")
        .help("The format of the verification output eg. junit, text"))
    .arg(Arg::new("verifier")
        .long("verifier")
        .value_name("VERIFIER")
        .help("The tool used to verify the provider contract"))
    .arg(Arg::new("verifier-version")
        .long("verifier-version")
        .value_name("VERIFIER_VERSION")
        .help("The version of the tool used to verify the provider contract"))
    .arg(Arg::new("build-url")
        .long("build-url")
        .value_name("BUILD_URL")
        .help("The build URL that created the provider contract"))
        .args(add_output_arguments(["json", "text"].to_vec(), "text"))
.args(add_verbose_arguments())
}
