# Pact Broker Client (Rust)

A client for the [Pact Broker](https://docs.pact.io/pact_broker/) and [PactFlow](https://pactflow.io/?utm_source=ossdocs&utm_campaign=pact_broker_client_readme). Publishes and retrieves pacts, pacticipants, pacticipant versions, environments, deployments and releases. Supports publishing provider contracts for PactFlow. The functionality is available via a CLI

![Build status](https://github.com/pact-foundation/pact_broker-client/workflows/Test/badge.svg)

## Installation

### Standalone executable

Download the latest [pact-ruby-standalone][pact-ruby-standalone] package. You do not need Ruby to run the CLI, as the Ruby runtime is packaged with the executable using Travelling Ruby.

## Connecting to a Pact Broker with a self signed certificate

To connect to a Pact Broker that uses custom SSL cerificates, set the environment variable `$SSL_CERT_FILE` or `$SSL_CERT_DIR` to a path that contains the appropriate certificate. Read more at https://docs.pact.io/pact_broker/advanced_topics/using-tls#for-non-jvm

## Pact Broker CLI

All commands prefixed with `pact-broker` can be used with the OSS Pact Broker and PactFlow. Commands prefixed with `pactflow` can only be used with PactFlow.

The Pact Broker base URL can be specified either using the environment variable `$PACT_BROKER_BASE_URL` or the `-b` or `--broker-base-url` parameters.

Pact Broker authentication can be performed either using basic auth or a bearer token.

Basic auth parameters can be specified using the `$PACT_BROKER_USERNAME` and `$PACT_BROKER_PASSWORD` environment variables, or the `-u` or `--broker-username` and `-p` or `--broker-password` parameters.

Authentication using a bearer token can be specified using the environment variable `$PACT_BROKER_TOKEN` or the `-k` or `--broker-token` parameters. This bearer token authentication is used by [PactFlow](https://pactflow.io) and is not available in the [OSS Pact Broker](https://docs.pact.io/pact_broker/), which only supports basic auth.

### Pacts

#### publish

```console
$ pact_cli pact-broker publish --help
Publishes pacts to the Pact Broker

Usage: pact_cli pact-broker publish [OPTIONS] --broker-base-url <PACT_BROKER_BASE_URL> [PACT_DIRS_OR_FILES]

Arguments:
  [PACT_DIRS_OR_FILES]  Pact directories or files

Options:
  -b, --broker-base-url <PACT_BROKER_BASE_URL>
          The base URL of the Pact Broker [env: PACT_BROKER_BASE_URL=]
  -u, --broker-username <PACT_BROKER_USERNAME>
          Pact Broker basic auth username [env: PACT_BROKER_USERNAME=]
  -p, --broker-password <PACT_BROKER_PASSWORD>
          Pact Broker basic auth password [env: PACT_BROKER_PASSWORD=]
  -k, --broker-token <PACT_BROKER_TOKEN>
          Pact Broker bearer token [env: PACT_BROKER_TOKEN=]
  -a, --consumer-app-version <consumer-app-version>
          The consumer application version
      --branch <branch>
          Repository branch of the consumer version
  -r, --auto-detect-version-properties
          Automatically detect the repository commit, branch and build URL from known CI environment variables or git CLI. Supports Buildkite, Circle CI, Travis CI, GitHub Actions, Jenkins, Hudson, AppVeyor, GitLab, CodeShip, Bitbucket and Azure DevOps.
  -t, --tag [<tag>]
          Tag name for consumer version. Can be specified multiple times.
  -g, --tag-with-git-branch
          Tag consumer version with the name of the current git branch. Supports Buildkite, Circle CI, Travis CI, GitHub Actions, Jenkins, Hudson, AppVeyor, GitLab, CodeShip, Bitbucket and Azure DevOps.
      --build-url <build-url>
          The build URL that created the pact
      --merge
          If a pact already exists for this consumer version and provider, merge the contents. Useful when running Pact tests concurrently on different build nodes.
  -o, --output <OUTPUT>
          Value must be one of ["json", "text"] [default: text] [possible values: json, text]
  -v, --verbose
          Verbose output.
  -h, --help
          Print help

```

Publish pacts to a Pact Broker.

#### list-latest-pact-versions

```console
$ pact_cli pact-broker list-latest-pact-versions --help
List the latest pact for each integration

Usage: pact_cli pact-broker list-latest-pact-versions [OPTIONS] --broker-base-url <PACT_BROKER_BASE_URL>

Options:
  -b, --broker-base-url <PACT_BROKER_BASE_URL>
          The base URL of the Pact Broker [env: PACT_BROKER_BASE_URL=]
  -u, --broker-username <PACT_BROKER_USERNAME>
          Pact Broker basic auth username [env: PACT_BROKER_USERNAME=]
  -p, --broker-password <PACT_BROKER_PASSWORD>
          Pact Broker basic auth password [env: PACT_BROKER_PASSWORD=]
  -k, --broker-token <PACT_BROKER_TOKEN>
          Pact Broker bearer token [env: PACT_BROKER_TOKEN=]
  -v, --verbose
          Verbose output.
  -o, --output <OUTPUT>
          Value must be one of ["json", "table"] [default: table] [possible values: json, table]
  -h, --help
          Print help

```

List the latest pact for each integration

### Environments

#### create-environment

```console
$ pact_cli pact-broker create-environment --help
Create an environment resource in the Pact Broker to represent a real world deployment or release environment

Usage: pact_cli pact-broker create-environment [OPTIONS] --name <NAME> --broker-base-url <PACT_BROKER_BASE_URL>

Options:
      --name <NAME>
          The uniquely identifying name of the environment as used in deployment code
      --display-name <DISPLAY_NAME>
          The display name of the environment
      --production
          Whether or not this environment is a production environment. This is currently informational only.
      --contact-name <CONTACT_NAME>
          The name of the team/person responsible for this environment
      --contact-email-address <CONTACT_EMAIL_ADDRESS>
          The email address of the team/person responsible for this environment
  -o, --output <OUTPUT>
          Value must be one of ["json", "text", "id"] [default: text] [possible values: json, text, id]
  -b, --broker-base-url <PACT_BROKER_BASE_URL>
          The base URL of the Pact Broker [env: PACT_BROKER_BASE_URL=]
  -u, --broker-username <PACT_BROKER_USERNAME>
          Pact Broker basic auth username [env: PACT_BROKER_USERNAME=]
  -p, --broker-password <PACT_BROKER_PASSWORD>
          Pact Broker basic auth password [env: PACT_BROKER_PASSWORD=]
  -k, --broker-token <PACT_BROKER_TOKEN>
          Pact Broker bearer token [env: PACT_BROKER_TOKEN=]
  -v, --verbose
          Verbose output.
  -h, --help
          Print help

```

Create an environment resource in the Pact Broker to represent a real world deployment or release environment.

#### update-environment

```console
$ pact_cli pact-broker update-environment --help
Update an environment resource in the Pact Broker

Usage: pact_cli pact-broker update-environment [OPTIONS] --uuid <UUID> --broker-base-url <PACT_BROKER_BASE_URL>

Options:
      --uuid <UUID>
          The UUID of the environment to update
      --name <NAME>
          The uniquely identifying name of the environment as used in deployment code
      --display-name <DISPLAY_NAME>
          The display name of the environment
      --production
          Whether or not this environment is a production environment. This is currently informational only.
      --contact-name <CONTACT_NAME>
          The name of the team/person responsible for this environment
      --contact-email-address <CONTACT_EMAIL_ADDRESS>
          The email address of the team/person responsible for this environment
  -o, --output <OUTPUT>
          Value must be one of ["json", "text", "id"] [default: text] [possible values: json, text, id]
  -b, --broker-base-url <PACT_BROKER_BASE_URL>
          The base URL of the Pact Broker [env: PACT_BROKER_BASE_URL=]
  -u, --broker-username <PACT_BROKER_USERNAME>
          Pact Broker basic auth username [env: PACT_BROKER_USERNAME=]
  -p, --broker-password <PACT_BROKER_PASSWORD>
          Pact Broker basic auth password [env: PACT_BROKER_PASSWORD=]
  -k, --broker-token <PACT_BROKER_TOKEN>
          Pact Broker bearer token [env: PACT_BROKER_TOKEN=]
  -v, --verbose
          Verbose output.
  -h, --help
          Print help

```

Update an environment resource in the Pact Broker.

#### describe-environment

```console
$ pact_cli pact-broker describe-environment --help
Describe an environment

Usage: pact_cli pact-broker describe-environment [OPTIONS] --uuid <UUID> --broker-base-url <PACT_BROKER_BASE_URL>

Options:
      --uuid <UUID>
          The UUID of the environment to describe
  -o, --output <OUTPUT>
          Value must be one of ["json", "text"] [default: text] [possible values: json, text]
  -b, --broker-base-url <PACT_BROKER_BASE_URL>
          The base URL of the Pact Broker [env: PACT_BROKER_BASE_URL=]
  -u, --broker-username <PACT_BROKER_USERNAME>
          Pact Broker basic auth username [env: PACT_BROKER_USERNAME=]
  -p, --broker-password <PACT_BROKER_PASSWORD>
          Pact Broker basic auth password [env: PACT_BROKER_PASSWORD=]
  -k, --broker-token <PACT_BROKER_TOKEN>
          Pact Broker bearer token [env: PACT_BROKER_TOKEN=]
  -v, --verbose
          Verbose output.
  -h, --help
          Print help

```

Describe an environment

#### delete-environment

```console
$ pact_cli pact-broker delete-environment --help
Delete an environment

Usage: pact_cli pact-broker delete-environment [OPTIONS] --uuid <UUID> --broker-base-url <PACT_BROKER_BASE_URL>

Options:
      --uuid <UUID>
          The UUID of the environment to delete
  -b, --broker-base-url <PACT_BROKER_BASE_URL>
          The base URL of the Pact Broker [env: PACT_BROKER_BASE_URL=]
  -u, --broker-username <PACT_BROKER_USERNAME>
          Pact Broker basic auth username [env: PACT_BROKER_USERNAME=]
  -p, --broker-password <PACT_BROKER_PASSWORD>
          Pact Broker basic auth password [env: PACT_BROKER_PASSWORD=]
  -k, --broker-token <PACT_BROKER_TOKEN>
          Pact Broker bearer token [env: PACT_BROKER_TOKEN=]
  -v, --verbose
          Verbose output.
  -h, --help
          Print help

```

Delete an environment

#### list-environments

```console
$ pact_cli pact-broker list-environments --help
List environments

Usage: pact_cli pact-broker list-environments [OPTIONS] --broker-base-url <PACT_BROKER_BASE_URL>

Options:
  -o, --output <OUTPUT>
          Value must be one of ["json", "text", "pretty"] [default: text] [possible values: json, text, pretty]
  -b, --broker-base-url <PACT_BROKER_BASE_URL>
          The base URL of the Pact Broker [env: PACT_BROKER_BASE_URL=]
  -u, --broker-username <PACT_BROKER_USERNAME>
          Pact Broker basic auth username [env: PACT_BROKER_USERNAME=]
  -p, --broker-password <PACT_BROKER_PASSWORD>
          Pact Broker basic auth password [env: PACT_BROKER_PASSWORD=]
  -k, --broker-token <PACT_BROKER_TOKEN>
          Pact Broker bearer token [env: PACT_BROKER_TOKEN=]
  -v, --verbose
          Verbose output.
  -h, --help
          Print help

```

List environments

### Deployments

#### record-deployment

```console
$ pact_cli pact-broker record-deployment --help
Record deployment of a pacticipant version to an environment

Usage: pact_cli pact-broker record-deployment [OPTIONS] --pacticipant <PACTICIPANT> --version <VERSION> --environment <ENVIRONMENT> --broker-base-url <PACT_BROKER_BASE_URL>

Options:
  -a, --pacticipant <PACTICIPANT>
          The name of the pacticipant that was deployed
  -e, --version <VERSION>
          The pacticipant version number that was deployed
      --environment <ENVIRONMENT>
          The name of the environment that the pacticipant version was deployed to
      --application-instance <APPLICATION_INSTANCE>
          Optional. The application instance to which the deployment has occurred - a logical identifer required to differentiate deployments when there are multiple instances of the same application in an environment. This field was called 'target' in a beta release
  -o, --output <OUTPUT>
          Value must be one of ["json", "text", "pretty"] [default: text] [possible values: json, text, pretty]
  -b, --broker-base-url <PACT_BROKER_BASE_URL>
          The base URL of the Pact Broker [env: PACT_BROKER_BASE_URL=]
  -u, --broker-username <PACT_BROKER_USERNAME>
          Pact Broker basic auth username [env: PACT_BROKER_USERNAME=]
  -p, --broker-password <PACT_BROKER_PASSWORD>
          Pact Broker basic auth password [env: PACT_BROKER_PASSWORD=]
  -k, --broker-token <PACT_BROKER_TOKEN>
          Pact Broker bearer token [env: PACT_BROKER_TOKEN=]
  -v, --verbose
          Verbose output.
  -h, --help
          Print help

```

Record deployment of a pacticipant version to an environment. See https://docs.pact.io/record-deployment for more information.

#### record-undeployment

```console
$ pact_cli pact-broker record-undeployment --help
Record undeployment of a pacticipant version from an environment.

Note that use of this command is only required if you are permanently removing an application instance from an environment. It is not required if you are deploying over a previous version, as record-deployment will automatically mark the previously deployed version as undeployed for you. See https://docs.pact.io/go/record-undeployment for more information.

Usage: pact_cli pact-broker record-undeployment [OPTIONS] --pacticipant <PACTICIPANT> --environment <ENVIRONMENT> --broker-base-url <PACT_BROKER_BASE_URL>

Options:
  -a, --pacticipant <PACTICIPANT>
          The name of the pacticipant that was undeployed

      --environment <ENVIRONMENT>
          The name of the environment that the pacticipant version was undeployed from

      --application-instance <APPLICATION_INSTANCE>
          Optional. The application instance from which the application is being undeployed - a logical identifer required to differentiate deployments when there are multiple instances of the same application in an environment. This field was called 'target' in a beta release

  -b, --broker-base-url <PACT_BROKER_BASE_URL>
          The base URL of the Pact Broker
          
          [env: PACT_BROKER_BASE_URL=]

  -u, --broker-username <PACT_BROKER_USERNAME>
          Pact Broker basic auth username
          
          [env: PACT_BROKER_USERNAME=]

  -p, --broker-password <PACT_BROKER_PASSWORD>
          Pact Broker basic auth password
          
          [env: PACT_BROKER_PASSWORD=]

  -k, --broker-token <PACT_BROKER_TOKEN>
          Pact Broker bearer token
          
          [env: PACT_BROKER_TOKEN=]

  -v, --verbose
          Verbose output.

  -o, --output <OUTPUT>
          Value must be one of ["json", "text", "pretty"]
          
          [default: text]
          [possible values: json, text, pretty]

  -h, --help
          Print help (see a summary with '-h')

```

Description:
  Note that use of this command is only required if you are permanently removing an application instance from an environment. It is not required if you are
  deploying over a previous version, as record-deployment will automatically mark the previously deployed version as undeployed for you. See
  https://docs.pact.io/record-undeployment for more information.

### Releases

#### record-release

```console
$ pact_cli pact-broker record-release --help
Record release of a pacticipant version to an environment.

Usage: pact_cli pact-broker record-release [OPTIONS] --pacticipant <PACTICIPANT> --version <VERSION> --environment <ENVIRONMENT> --broker-base-url <PACT_BROKER_BASE_URL>

Options:
  -a, --pacticipant <PACTICIPANT>
          The name of the pacticipant that was released.
  -e, --version <VERSION>
          The pacticipant version number that was released.
      --environment <ENVIRONMENT>
          The name of the environment that the pacticipant version was released to.
  -o, --output <OUTPUT>
          Value must be one of ["json", "text", "pretty"] [default: text] [possible values: json, text, pretty]
  -b, --broker-base-url <PACT_BROKER_BASE_URL>
          The base URL of the Pact Broker [env: PACT_BROKER_BASE_URL=]
  -u, --broker-username <PACT_BROKER_USERNAME>
          Pact Broker basic auth username [env: PACT_BROKER_USERNAME=]
  -p, --broker-password <PACT_BROKER_PASSWORD>
          Pact Broker basic auth password [env: PACT_BROKER_PASSWORD=]
  -k, --broker-token <PACT_BROKER_TOKEN>
          Pact Broker bearer token [env: PACT_BROKER_TOKEN=]
  -v, --verbose
          Verbose output.
  -h, --help
          Print help

```

Record release of a pacticipant version to an environment. See See https://docs.pact.io/record-release for more information.

#### record-support-ended

```console
$ pact_cli pact-broker record-support-ended --help
Record the end of support for a pacticipant version in an environment.

Usage: pact_cli pact-broker record-support-ended [OPTIONS] --pacticipant <PACTICIPANT> --version <VERSION> --environment <ENVIRONMENT> --broker-base-url <PACT_BROKER_BASE_URL>

Options:
  -a, --pacticipant <PACTICIPANT>
          The name of the pacticipant.
  -e, --version <VERSION>
          The pacticipant version number for which support is ended.
      --environment <ENVIRONMENT>
          The name of the environment in which the support is ended.
  -o, --output <OUTPUT>
          Value must be one of ["json", "text", "pretty"] [default: text] [possible values: json, text, pretty]
  -b, --broker-base-url <PACT_BROKER_BASE_URL>
          The base URL of the Pact Broker [env: PACT_BROKER_BASE_URL=]
  -u, --broker-username <PACT_BROKER_USERNAME>
          Pact Broker basic auth username [env: PACT_BROKER_USERNAME=]
  -p, --broker-password <PACT_BROKER_PASSWORD>
          Pact Broker basic auth password [env: PACT_BROKER_PASSWORD=]
  -k, --broker-token <PACT_BROKER_TOKEN>
          Pact Broker bearer token [env: PACT_BROKER_TOKEN=]
  -v, --verbose
          Verbose output.
  -h, --help
          Print help

```

Record the end of support for a pacticipant version in an environment. See https://docs.pact.io/record-support-ended for more information.

### Matrix

#### can-i-deploy

```console
$ pact_cli pact-broker can-i-deploy --help
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
  
    $ pact-broker can-i-deploy --pacticipant Foo 173153ae0 // --pacticipant Bar --latest main
  
    ##### Polling
  
    If the verification process takes a long time and there are results missing when the can-i-deploy command runs in your CI/CD pipeline,
    you can configure the command to poll and wait for the missing results to arrive. The arguments to specify are `--retry-while-unknown
    TIMES` and `--retry-interval SECONDS`, set to appropriate values for your pipeline.
    

Usage: pact_cli pact-broker can-i-deploy [OPTIONS] --pacticipant [<PACTICIPANT>] --broker-base-url <PACT_BROKER_BASE_URL>

Options:
  -a, --pacticipant [<PACTICIPANT>]
          The pacticipant name. Use once for each pacticipant being checked.

  -e, --version <VERSION>
          The pacticipant version. Must be entered after the --pacticipant that it relates to.

      --ignore
          The pacticipant name to ignore. Use once for each pacticipant being ignored. A specific version can be ignored by also specifying a --version after the pacticipant name option. The environment variable PACT_BROKER_CAN_I_DEPLOY_IGNORE may also be used to specify a pacticipant name to ignore, with commas to separate multiple pacticipant names if necessary.

  -l, --latest
          Use the latest pacticipant version. Optionally specify a TAG to use the latest version with the specified tag.

      --branch <BRANCH>
          The branch of the version for which you want to check the verification results.

      --main-branch
          Use the latest version of the configured main branch of the pacticipant as the version for which you want to check the verification results

      --no-main-branch
          No main branch of the pacticipant as the version for which you want to check the verification results

      --skip-main-branch
          Skip the configured main branch of the pacticipant as the version for which you want to check the verification results

      --to-environment <ENVIRONMENT>
          The environment into which the pacticipant(s) are to be deployed

      --to <TO>
          The tag that represents the branch or environment of the integrated applications for which you want to check the verification result status.

  -o, --output <OUTPUT>
          Value must be one of ["json", "table"]
          
          [default: table]
          [possible values: json, table]

      --retry-while-unknown <TIMES>
          The number of times to retry while there is an unknown verification result (ie. the provider verification is likely still running)

      --retry-interval <SECONDS>
          The time between retries in seconds. Use in conjuction with --retry-while-unknown

      --dry-run
          When dry-run is enabled, always exit process with a success code. Can also be enabled by setting the environment variable PACT_BROKER_CAN_I_DEPLOY_DRY_RUN=true. This mode is useful when setting up your CI/CD pipeline for the first time, or in a 'break glass' situation where you need to knowingly deploy what Pact considers a breaking change. For the second scenario, it is recommended to use the environment variable and just set it for the build required to deploy that particular version, so you don't accidentally leave the dry run mode enabled.

      --no-dry-run
          When dry-run is enabled, always exit process with a success code. Can also be enabled by setting the environment variable PACT_BROKER_CAN_I_DEPLOY_DRY_RUN=true. This mode is useful when setting up your CI/CD pipeline for the first time, or in a 'break glass' situation where you need to knowingly deploy what Pact considers a breaking change. For the second scenario, it is recommended to use the environment variable and just set it for the build required to deploy that particular version, so you don't accidentally leave the dry run mode enabled.

      --skip-dry-run
          When dry-run is enabled, always exit process with a success code. Can also be enabled by setting the environment variable PACT_BROKER_CAN_I_DEPLOY_DRY_RUN=true. This mode is useful when setting up your CI/CD pipeline for the first time, or in a 'break glass' situation where you need to knowingly deploy what Pact considers a breaking change. For the second scenario, it is recommended to use the environment variable and just set it for the build required to deploy that particular version, so you don't accidentally leave the dry run mode enabled.

  -b, --broker-base-url <PACT_BROKER_BASE_URL>
          The base URL of the Pact Broker
          
          [env: PACT_BROKER_BASE_URL=]

  -u, --broker-username <PACT_BROKER_USERNAME>
          Pact Broker basic auth username
          
          [env: PACT_BROKER_USERNAME=]

  -p, --broker-password <PACT_BROKER_PASSWORD>
          Pact Broker basic auth password
          
          [env: PACT_BROKER_PASSWORD=]

  -k, --broker-token <PACT_BROKER_TOKEN>
          Pact Broker bearer token
          
          [env: PACT_BROKER_TOKEN=]

  -v, --verbose
          Verbose output.

  -h, --help
          Print help (see a summary with '-h')

```

Description:
  Returns exit code 0 or 1, indicating whether or not the specified application (pacticipant) has a successful verification result with each of the application
  versions that are already deployed to a particular environment. Prints out the relevant pact/verification details, indicating any missing or failed
  verification results.

  The can-i-deploy tool was originally written to support specifying versions and dependencies using tags. This usage has now been superseded by first class
  support for environments, deployments and releases. For documentation on how to use can-i-deploy with tags, please see
  https://docs.pact.io/pact_broker/client_cli/can_i_deploy_usage_with_tags/

  Before `can-i-deploy` can be used, the relevant environment resources must first be created in the Pact Broker using the `create-environment` command. The
  "test" and "production" environments will have been seeded for you. You can check the existing environments by running `pact-broker list-environments`. See
  https://docs.pact.io/pact_broker/client_cli/readme#environments for more information.

`$ pact_cli pact-broker create-environment --name "uat" --display-name "UAT" --no-production`

  After an application is deployed or released, its deployment must be recorded using the `record-deployment` or `record-release` commands. See
  https://docs.pact.io/pact_broker/recording_deployments_and_releases/ for more information.

`$ pact_cli pact-broker record-deployment --pacticipant Foo --version 173153ae0 --environment uat`

  Before an application is deployed or released to an environment, the can-i-deploy command must be run to check that the application version is safe to deploy
  with the versions of each integrated application that are already in that environment.

`$ pact_cli pact-broker can-i-deploy --pacticipant PACTICIPANT --version VERSION --to-environment ENVIRONMENT`

  Example: can I deploy version 173153ae0 of application Foo to the test environment?

`$ pact_cli pact-broker can-i-deploy --pacticipant Foo --version 173153ae0 --to-environment test`

  Can-i-deploy can also be used to check if arbitrary versions have a successful verification. When asking "Can I deploy this application version with the
  latest version from the main branch of another application" it functions as a "can I merge" check.

`$ pact_cli pact-broker can-i-deploy --pacticipant Foo 173153ae0 \ --pacticipant Bar --latest main`

##### Polling

If the verification process takes a long time and there are results missing when the can-i-deploy command runs in your CI/CD pipeline, you can configure the
command to poll and wait for the missing results to arrive. The arguments to specify are `--retry-while-unknown TIMES` and `--retry-interval SECONDS`, set to
appropriate values for your pipeline.

#### can-i-merge

```console
$ pact_cli pact-broker can-i-merge --help
Checks if the specified pacticipant version is compatible with the configured main branch of each of the pacticipants with which it is integrated.

Usage: pact_cli pact-broker can-i-merge [OPTIONS] --broker-base-url <PACT_BROKER_BASE_URL> --pacticipant [<PACTICIPANT>]

Options:
  -b, --broker-base-url <PACT_BROKER_BASE_URL>
          The base URL of the Pact Broker [env: PACT_BROKER_BASE_URL=]
  -u, --broker-username <PACT_BROKER_USERNAME>
          Pact Broker basic auth username [env: PACT_BROKER_USERNAME=]
  -p, --broker-password <PACT_BROKER_PASSWORD>
          Pact Broker basic auth password [env: PACT_BROKER_PASSWORD=]
  -k, --broker-token <PACT_BROKER_TOKEN>
          Pact Broker bearer token [env: PACT_BROKER_TOKEN=]
  -a, --pacticipant [<PACTICIPANT>]
          The pacticipant name. Use once for each pacticipant being checked.
  -e, --version <VERSION>
          The pacticipant version. Must be entered after the --pacticipant that it relates to.
  -o, --output <OUTPUT>
          Value must be one of ["json", "table"] [default: table] [possible values: json, table]
      --retry-while-unknown <TIMES>
          The number of times to retry while there is an unknown verification result (ie. the provider verification is likely still running) [default: 0]
      --retry-interval <SECONDS>
          The time between retries in seconds. Use in conjuction with --retry-while-unknown [default: 10]
      --dry-run <dry-run>
          When dry-run is enabled, always exit process with a success code. Can also be enabled by setting the environment variable PACT_BROKER_CAN_I_MERGE_DRY_RUN=true. This mode is useful when setting up your CI/CD pipeline for the first time, or in a 'break glass' situation where you need to knowingly deploy what Pact considers a breaking change. For the second scenario, it is recommended to use the environment variable and just set it for the build required to deploy that particular version, so you don't accidentally leave the dry run mode enabled.
  -v, --verbose
          Verbose output.
  -h, --help
          Print help

```

Description:
  Checks if the specified pacticipant version is compatible with the configured main branch of each of the pacticipants with which it is integrated.

### Pacticipants

#### create-or-update-pacticipant

```console
$ pact_cli pact-broker create-or-update-pacticipant --help
Create or update pacticipant by name

Usage: pact_cli pact-broker create-or-update-pacticipant [OPTIONS] --broker-base-url <PACT_BROKER_BASE_URL> --name <NAME>

Options:
  -b, --broker-base-url <PACT_BROKER_BASE_URL>
          The base URL of the Pact Broker [env: PACT_BROKER_BASE_URL=]
  -u, --broker-username <PACT_BROKER_USERNAME>
          Pact Broker basic auth username [env: PACT_BROKER_USERNAME=]
  -p, --broker-password <PACT_BROKER_PASSWORD>
          Pact Broker basic auth password [env: PACT_BROKER_PASSWORD=]
  -k, --broker-token <PACT_BROKER_TOKEN>
          Pact Broker bearer token [env: PACT_BROKER_TOKEN=]
      --name <NAME>
          Pacticipant name
      --display-name <DISPLAY_NAME>
          Display name
      --main-branch <MAIN_BRANCH>
          The main development branch of the pacticipant repository
      --repository-url <REPOSITORY_URL>
          The repository URL of the pacticipant
  -o, --output <OUTPUT>
          Value must be one of ["json", "text"] [default: text] [possible values: json, text]
  -v, --verbose
          Verbose output.
  -h, --help
          Print help

```

Create or update pacticipant by name

#### describe-pacticipant

```console
$ pact_cli pact-broker describe-pacticipant --help
Describe a pacticipant

Usage: pact_cli pact-broker describe-pacticipant [OPTIONS] --broker-base-url <PACT_BROKER_BASE_URL> --name <NAME>

Options:
  -b, --broker-base-url <PACT_BROKER_BASE_URL>
          The base URL of the Pact Broker [env: PACT_BROKER_BASE_URL=]
  -u, --broker-username <PACT_BROKER_USERNAME>
          Pact Broker basic auth username [env: PACT_BROKER_USERNAME=]
  -p, --broker-password <PACT_BROKER_PASSWORD>
          Pact Broker basic auth password [env: PACT_BROKER_PASSWORD=]
  -k, --broker-token <PACT_BROKER_TOKEN>
          Pact Broker bearer token [env: PACT_BROKER_TOKEN=]
      --name <NAME>
          Pacticipant name
  -o, --output <OUTPUT>
          Value must be one of ["json", "text"] [default: text] [possible values: json, text]
  -v, --verbose
          Verbose output.
  -h, --help
          Print help

```

Describe a pacticipant

#### list-pacticipants

```console
$ pact_cli pact-broker list-pacticipants --help
List pacticipants

Usage: pact_cli pact-broker list-pacticipants [OPTIONS] --broker-base-url <PACT_BROKER_BASE_URL>

Options:
  -b, --broker-base-url <PACT_BROKER_BASE_URL>
          The base URL of the Pact Broker [env: PACT_BROKER_BASE_URL=]
  -u, --broker-username <PACT_BROKER_USERNAME>
          Pact Broker basic auth username [env: PACT_BROKER_USERNAME=]
  -p, --broker-password <PACT_BROKER_PASSWORD>
          Pact Broker basic auth password [env: PACT_BROKER_PASSWORD=]
  -k, --broker-token <PACT_BROKER_TOKEN>
          Pact Broker bearer token [env: PACT_BROKER_TOKEN=]
  -o, --output <OUTPUT>
          Value must be one of ["json", "text"] [default: text] [possible values: json, text]
  -v, --verbose
          Verbose output.
  -h, --help
          Print help

```

List pacticipants

### Webhooks

#### create-webhook

```console
$ pact_cli pact-broker create-webhook --help
Create a webhook

Usage: pact_cli pact-broker create-webhook [OPTIONS] --broker-base-url <PACT_BROKER_BASE_URL> <URL>

Arguments:
  <URL>  Webhook URL

Options:
  -X, --request <METHOD>
          Webhook HTTP method
  -H, --header [<one two three>]
          Webhook Header
  -d, --data <DATA>
          Webhook payload
      --user <USER>
          Webhook basic auth username and password eg. username:password
      --consumer <CONSUMER>
          Consumer name
      --consumer-label <CONSUMER_LABEL>
          Consumer label, mutually exclusive with consumer name
      --provider <PROVIDER>
          Provider name
      --provider-label <PROVIDER_LABEL>
          Provider label, mutually exclusive with provider name
      --description <DESCRIPTION>
          Webhook description
      --contract-content-changed <contract-content-changed>
          Trigger this webhook when the pact content changes
      --contract-published <contract-published>
          Trigger this webhook when a pact is published
      --provider-verification-published <provider-verification-published>
          Trigger this webhook when a provider verification result is published
      --provider-verification-failed <provider-verification-failed>
          Trigger this webhook when a failed provider verification result is published
      --provider-verification-succeeded <provider-verification-succeeded>
          Trigger this webhook when a successful provider verification result is published
      --contract-requiring-verification-published <contract-requiring-verification-published>
          Trigger this webhook when a contract is published that requires verification
      --team-uuid <UUID>
          UUID of the PactFlow team to which the webhook should be assigned (PactFlow only)
  -b, --broker-base-url <PACT_BROKER_BASE_URL>
          The base URL of the Pact Broker [env: PACT_BROKER_BASE_URL=]
  -u, --broker-username <PACT_BROKER_USERNAME>
          Pact Broker basic auth username [env: PACT_BROKER_USERNAME=]
  -p, --broker-password <PACT_BROKER_PASSWORD>
          Pact Broker basic auth password [env: PACT_BROKER_PASSWORD=]
  -k, --broker-token <PACT_BROKER_TOKEN>
          Pact Broker bearer token [env: PACT_BROKER_TOKEN=]
  -v, --verbose
          Verbose output.
  -h, --help
          Print help

```

Description:
  Create a curl command that executes the request that you want your webhook to execute, then replace "curl" with "pact-broker create-webhook" and add the
  consumer, provider, event types and broker details. Note that the URL must be the first parameter when executing create-webhook.

  Note that the -u option from the curl command clashes with the -u option from the pact-broker CLI. When used in this command, the -u will be used as a curl
  option. Please use the --broker-username or environment variable for the Pact Broker username.

#### create-or-update-webhook

```console
$ pact_cli pact-broker create-or-update-webhook --help
Create or update a webhook

Usage: pact_cli pact-broker create-or-update-webhook [OPTIONS] --broker-base-url <PACT_BROKER_BASE_URL> --uuid <UUID> <URL>

Arguments:
  <URL>  Webhook URL

Options:
  -b, --broker-base-url <PACT_BROKER_BASE_URL>
          The base URL of the Pact Broker [env: PACT_BROKER_BASE_URL=]
  -u, --broker-username <PACT_BROKER_USERNAME>
          Pact Broker basic auth username [env: PACT_BROKER_USERNAME=]
  -p, --broker-password <PACT_BROKER_PASSWORD>
          Pact Broker basic auth password [env: PACT_BROKER_PASSWORD=]
  -k, --broker-token <PACT_BROKER_TOKEN>
          Pact Broker bearer token [env: PACT_BROKER_TOKEN=]
      --uuid <UUID>
          Specify the uuid for the webhook
  -X, --request <METHOD>
          Webhook HTTP method
  -H, --header [<one two three>]
          Webhook Header
  -d, --data <DATA>
          Webhook payload
      --user <USER>
          Webhook basic auth username and password eg. username:password
      --consumer <CONSUMER>
          Consumer name
      --consumer-label <CONSUMER_LABEL>
          Consumer label, mutually exclusive with consumer name
      --provider <PROVIDER>
          Provider name
      --provider-label <PROVIDER_LABEL>
          Provider label, mutually exclusive with provider name
      --description <DESCRIPTION>
          Webhook description
      --contract-content-changed <contract-content-changed>
          Trigger this webhook when the pact content changes
      --contract-published <contract-published>
          Trigger this webhook when a pact is published
      --provider-verification-published <provider-verification-published>
          Trigger this webhook when a provider verification result is published
      --provider-verification-failed <provider-verification-failed>
          Trigger this webhook when a failed provider verification result is published
      --provider-verification-succeeded <provider-verification-succeeded>
          Trigger this webhook when a successful provider verification result is published
      --contract-requiring-verification-published <contract-requiring-verification-published>
          Trigger this webhook when a contract is published that requires verification
      --team-uuid <UUID>
          UUID of the PactFlow team to which the webhook should be assigned (PactFlow only)
  -v, --verbose
          Verbose output.
  -h, --help
          Print help

```

Description:
  Create a curl command that executes the request that you want your webhook to execute, then replace "curl" with "pact-broker create-or-update-webhook" and
  add the consumer, provider, event types and broker details. Note that the URL must be the first parameter when executing create-or-update-webhook and a uuid
  must also be provided. You can generate a valid UUID by using the `generate-uuid` command.

  Note that the -u option from the curl command clashes with the -u option from the pact-broker CLI. When used in this command, the -u will be used as a curl
  option. Please use the --broker-username or environment variable for the Pact Broker username.

#### test-webhook

```console
$ pact_cli pact-broker test-webhook --help
Test a webhook

Usage: pact_cli pact-broker test-webhook [OPTIONS] --uuid <UUID> --broker-base-url <PACT_BROKER_BASE_URL>

Options:
      --uuid <UUID>
          Specify the uuid for the webhook
  -b, --broker-base-url <PACT_BROKER_BASE_URL>
          The base URL of the Pact Broker [env: PACT_BROKER_BASE_URL=]
  -u, --broker-username <PACT_BROKER_USERNAME>
          Pact Broker basic auth username [env: PACT_BROKER_USERNAME=]
  -p, --broker-password <PACT_BROKER_PASSWORD>
          Pact Broker basic auth password [env: PACT_BROKER_PASSWORD=]
  -k, --broker-token <PACT_BROKER_TOKEN>
          Pact Broker bearer token [env: PACT_BROKER_TOKEN=]
  -v, --verbose
          Verbose output.
  -h, --help
          Print help

```

Test the execution of a webhook

### Branches

#### delete-branch

```console
$ pact_cli pact-broker delete-branch --help
Deletes a pacticipant branch. Does not delete the versions or pacts/verifications associated with the branch, but does make the pacts inaccessible for verification via consumer versions selectors or WIP pacts.

Usage: pact_cli pact-broker delete-branch [OPTIONS] --broker-base-url <PACT_BROKER_BASE_URL> --branch <BRANCH> --pacticipant <PACTICIPANT>

Options:
  -b, --broker-base-url <PACT_BROKER_BASE_URL>
          The base URL of the Pact Broker [env: PACT_BROKER_BASE_URL=]
  -u, --broker-username <PACT_BROKER_USERNAME>
          Pact Broker basic auth username [env: PACT_BROKER_USERNAME=]
  -p, --broker-password <PACT_BROKER_PASSWORD>
          Pact Broker basic auth password [env: PACT_BROKER_PASSWORD=]
  -k, --broker-token <PACT_BROKER_TOKEN>
          Pact Broker bearer token [env: PACT_BROKER_TOKEN=]
      --branch <BRANCH>
          The pacticipant branch name
  -a, --pacticipant <PACTICIPANT>
          The name of the pacticipant that the branch belongs to
  -v, --verbose
          Verbose output.
  -h, --help
          Print help

```

Deletes a pacticipant branch. Does not delete the versions or pacts/verifications associated with the branch, but does make the pacts inaccessible for verification via consumer versions selectors or WIP pacts.

### Tags

#### create-version-tag

```console
$ pact_cli pact-broker create-version-tag --help
Add a tag to a pacticipant version

Usage: pact_cli pact-broker create-version-tag [OPTIONS] --pacticipant <PACTICIPANT> --version <VERSION>

Options:
  -a, --pacticipant <PACTICIPANT>
          The pacticipant name
  -e, --version <VERSION>
          The pacticipant version
  -t, --tag [<TAG>]
          Tag name for pacticipant version. Can be specified multiple times
      --auto-create-version <auto-create-version>
          Automatically create the pacticipant version if it does not exist
  -g, --tag-with-git-branch <tag-with-git-branch>
          Tag pacticipant version with the name of the current git branch
  -h, --help
          Print help

```

Add a tag to a pacticipant version

### Versions

#### describe-version

```console
$ pact_cli pact-broker describe-version --help
Describes a pacticipant version. If no version or tag is specified, the latest version is described.

Usage: pact_cli pact-broker describe-version [OPTIONS] --pacticipant <PACTICIPANT>

Options:
  -a, --pacticipant <PACTICIPANT>  The name of the pacticipant that the version belongs to
  -e, --version <VERSION>          The pacticipant version number
  -l, --latest <TAG>               Describe the latest pacticipant version. Optionally specify a TAG to describe the latest version with the specified tag
  -o, --output <OUTPUT>            Value must be one of ["json", "table", "id"] [default: table] [possible values: json, table, id]
  -h, --help                       Print help

```

Describes a pacticipant version. If no version or tag is specified, the latest version is described.

#### create-or-update-version

```console
$ pact_cli pact-broker create-or-update-version --help
Create or update pacticipant version by version number

Usage: pact_cli pact-broker create-or-update-version [OPTIONS] --pacticipant <PACTICIPANT> --version <VERSION>

Options:
  -a, --pacticipant <PACTICIPANT>  The pacticipant name
  -e, --version <VERSION>          The pacticipant version number
      --branch <BRANCH>            The repository branch name
  -t, --tag [<TAG>]                Tag name for pacticipant version. Can be specified multiple times
  -o, --output <OUTPUT>            Value must be one of ["json", "text"] [default: text] [possible values: json, text]
  -h, --help                       Print help

```

Create or update pacticipant version by version number

### Miscellaneous

#### generate-uuid

```console
$ pact_cli pact-broker generate-uuid --help
Generate a UUID for use when calling create-or-update-webhook

Usage: pact_cli pact-broker generate-uuid

Options:
  -h, --help  Print help

```

Generate a UUID for use when calling create-or-update-webhook

### Provider contracts (PactFlow only)

#### publish-provider-contract

```console
$ pact_cli pactflow publish-provider-contract --help
Publish provider contract to PactFlow

Usage: pact_cli pactflow publish-provider-contract [OPTIONS] --broker-base-url <PACT_BROKER_BASE_URL> --provider-app-version <PROVIDER_APP_VERSION> <CONTRACT_FILE>

Arguments:
  <CONTRACT_FILE>  The contract file(s)

Options:
  -b, --broker-base-url <PACT_BROKER_BASE_URL>
          The base URL of the Pact Broker [env: PACT_BROKER_BASE_URL=]
  -u, --broker-username <PACT_BROKER_USERNAME>
          Pact Broker basic auth username [env: PACT_BROKER_USERNAME=]
  -p, --broker-password <PACT_BROKER_PASSWORD>
          Pact Broker basic auth password [env: PACT_BROKER_PASSWORD=]
  -k, --broker-token <PACT_BROKER_TOKEN>
          Pact Broker bearer token [env: PACT_BROKER_TOKEN=]
      --provider <PROVIDER>
          The provider name
  -a, --provider-app-version <PROVIDER_APP_VERSION>
          The provider application version
      --branch <BRANCH>
          Repository branch of the provider version
  -t, --tag [<TAG>]
          Tag name for provider version. Can be specified multiple times.
      --specification <SPECIFICATION>
          The contract specification [default: oas]
      --content-type <CONTENT_TYPE>
          The content type. eg. application/yml
      --verification-success <verification-success>
          Whether or not the self verification passed successfully.
      --verification-exit-code <N>
          The exit code of the verification process. Can be used instead of --verification-success|--no-verification-success for a simpler build script.
      --verification-results <VERIFICATION_RESULTS>
          The path to the file containing the output from the verification process
      --verification-results-content-type <VERIFICATION_RESULTS_CONTENT_TYPE>
          The content type of the verification output eg. text/plain, application/yaml
      --verification-results-format <VERIFICATION_RESULTS_FORMAT>
          The format of the verification output eg. junit, text
      --verifier <VERIFIER>
          The tool used to verify the provider contract
      --verifier-version <VERIFIER_VERSION>
          The version of the tool used to verify the provider contract
      --build-url <BUILD_URL>
          The build URL that created the provider contract
  -o, --output <OUTPUT>
          Value must be one of ["json", "text"] [default: text] [possible values: json, text]
  -v, --verbose
          Verbose output.
  -h, --help
          Print help

```

Publish provider contract to PactFlow

## Pact Plugin CLI

```console
$ pact_cli --help
A pact cli tool

Usage: pact_cli [COMMAND]

Commands:
  pact-broker  
  pactflow     
  completions  Generates completion scripts for your shell
  docker       Run the Pact Broker as a Docker container
  examples     download example projects
  project      Pact project actions for setting up and managing pact projects
  standalone   Install & Run the Pact Broker in $HOME/traveling-broker
  plugin       CLI utility for Pact plugins
  mock         Standalone Pact mock server
  stub         Pact Stub Server 0.0.4
  verifier     
  help         Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help

```

```console
$ pact_cli plugin list --help
List the installed plugins

Usage: pact_cli plugin list [COMMAND]

Commands:
  installed  List installed plugins
  known      List known plugins
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help

```

```console
$ pact_cli plugin list installed --help
List installed plugins

Usage: pact_cli plugin list installed

Options:
  -h, --help  Print help

```

```console
$ pact_cli plugin list known --help
List known plugins

Usage: pact_cli plugin list known [OPTIONS]

Options:
  -a, --show-all-versions  Display all versions of the known plugins
  -h, --help               Print help

```

```console
$ pact_cli plugin env --help
Print out the Pact plugin environment config

Usage: pact_cli plugin env

Options:
  -h, --help  Print help

```

```console
$ pact_cli plugin install --help
Install a plugin 

A plugin can be either installed from a URL, or for a known plugin, by name (and optionally version)

Usage: pact_cli plugin install [OPTIONS] <SOURCE>

Arguments:
  <SOURCE>  Where to fetch the plugin files from. This should be a URL or the name of a known plugin.

Options:
  -t, --source-type <SOURCE_TYPE>  The type of source to fetch the plugin files from. Will default to Github releases. [possible values: github]
  -y, --yes                        Automatically answer Yes for all prompts
  -s, --skip-if-installed          Skip installing the plugin if the same version is already installed
  -v, --version <VERSION>          The version to install. This is only used for known plugins.
  -h, --help                       Print help

```

```console
$ pact_cli plugin remove --help
Remove a plugin

Usage: pact_cli plugin remove [OPTIONS] <NAME> [VERSION]

Arguments:
  <NAME>     Plugin name
  [VERSION]  Plugin version. Not required if there is only one plugin version.

Options:
  -y, --yes   Automatically answer Yes for all prompts
  -h, --help  Print help

```

```console
$ pact_cli plugin enable --help
Enable a plugin version

Usage: pact_cli plugin enable <name> [VERSION]

Arguments:
  <name>     Plugin name
  [VERSION]  Plugin version. Not required if there is only one plugin version.

Options:
  -h, --help  Print help

```

```console
$ pact_cli plugin disable --help
Disable a plugin version

Usage: pact_cli plugin disable <name> [VERSION]

Arguments:
  <name>     Plugin name
  [VERSION]  Plugin version. Not required if there is only one plugin version.

Options:
  -h, --help  Print help

```

```console
$ pact_cli plugin repository --help
Sub-commands for dealing with a plugin repository

Usage: pact_cli plugin repository [COMMAND]

Commands:
  validate                 Check the consistency of the repository index file
  new                      Create a new blank repository index file
  add-plugin-version       Add a plugin version to the index file (will update existing entry)
  add-all-plugin-versions  Add all versions of a plugin to the index file (will update existing entries)
  yank-version             Remove a plugin version from the index file
  list                     List all plugins found in the index file
  list-versions            List all plugin versions found in the index file
  help                     Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help

```

```console
$ pact_cli plugin repository validate --help
Check the consistency of the repository index file

Usage: pact_cli plugin repository validate <FILENAME>

Arguments:
  <FILENAME>  Filename to validate

Options:
  -h, --help  Print help

```

```console
$ pact_cli plugin repository new --help
Create a new blank repository index file

Usage: pact_cli plugin repository new [OPTIONS] [FILENAME]

Arguments:
  [FILENAME]  Filename to use for the new file. By default will use repository.index

Options:
  -o, --overwrite   Overwrite any existing file?
  -h, --help       Print help

```

```console
$ pact_cli plugin repository add-plugin-version --help
Add a plugin version to the index file (will update existing entry)

Usage: pact_cli plugin repository add-plugin-version <COMMAND>

Commands:
  file     Add an entry for a local plugin manifest file to the repository file
  git-hub  Add an entry for a GitHub Release to the repository file
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help

```

```console
$ pact_cli plugin repository add-plugin-version git-hub --help
Add an entry for a GitHub Release to the repository file

Usage: pact_cli plugin repository add-plugin-version git-hub <REPOSITORY_FILE> <URL>

Arguments:
  <REPOSITORY_FILE>  Repository index file to update
  <URL>              Base URL for GitHub APIs, will default to https://api.github.com/repos/

Options:
  -h, --help  Print help

```

```console
$ pact_cli plugin repository add-plugin-version file --help
Add an entry for a local plugin manifest file to the repository file

Usage: pact_cli plugin repository add-plugin-version file <REPOSITORY_FILE> <FILE>

Arguments:
  <REPOSITORY_FILE>  Repository index file to update
  <FILE>             Path to the local plugin manifest file

Options:
  -h, --help  Print help

```

```console
$ pact_cli plugin repository add-all-plugin-versions --help
Add all versions of a plugin to the index file (will update existing entries)

Usage: pact_cli plugin repository add-all-plugin-versions <REPOSITORY_FILE> <OWNER> <REPOSITORY> [BASE_URL]

Arguments:
  <REPOSITORY_FILE>  Repository index file to update
  <OWNER>            Repository owner to load versions from
  <REPOSITORY>       Repository to load versions from
  [BASE_URL]         Base URL for GitHub APIs, will default to https://api.github.com/repos/

Options:
  -h, --help  Print help

```

```console
$ pact_cli plugin repository yank-version --help
Remove a plugin version from the index file

Usage: pact_cli plugin repository yank-version

Options:
  -h, --help  Print help

```

```console
$ pact_cli plugin list --help
List the installed plugins

Usage: pact_cli plugin list [COMMAND]

Commands:
  installed  List installed plugins
  known      List known plugins
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help

```

```console
$ pact_cli plugin repository list-versions --help
List all plugin versions found in the index file

Usage: pact_cli plugin repository list-versions <FILENAME> <NAME>

Arguments:
  <FILENAME>  Filename to list versions from
  <NAME>      Plugin entry to list versions for

Options:
  -h, --help  Print help

```

## Pact Mock Server CLI

```console
$ pact_cli mock --help
Standalone Pact mock server

Usage: pact_cli mock [OPTIONS] [COMMAND]

Commands:
  start            Starts the master mock server
  list             Lists all the running mock servers
  create           Creates a new mock server from a pact file
  verify           Verify the mock server by id or port number, and generate a pact file if all ok
  shutdown         Shutdown the mock server by id or port number, releasing all its resources
  shutdown-master  Performs a graceful shutdown of the master server (displayed when it started)
  help             Print this message or the help of the given subcommand(s)

Options:
      --help                 Print help and exit
  -v, --version              Print version information and exit
  -p, --port <port>          port the master mock server runs on (defaults to 8080)
  -h, --host <host>          hostname the master mock server runs on (defaults to localhost)
  -l, --loglevel <loglevel>  Log level for mock servers to write to the log file (defaults to info) [possible values: error, warn, info, debug, trace, none]
      --no-term-log          Turns off using terminal ANSI escape codes
      --no-file-log          Do not log to an output file

```

```console
$ pact_cli mock start --help
Starts the master mock server

Usage: pact_cli mock start [OPTIONS]

Options:
      --help                     Print help and exit
  -o, --output <output>          the directory where to write files to (defaults to current directory)
      --base-port <base-port>    the base port number that mock server ports will be allocated from. If not specified, ports will be randomly assigned by the OS.
  -v, --version                  Print version information and exit
  -p, --port <port>              port the master mock server runs on (defaults to 8080)
      --server-key <server-key>  the server key to use to authenticate shutdown requests (defaults to a random generated one)
  -h, --host <host>              hostname the master mock server runs on (defaults to localhost)
  -l, --loglevel <loglevel>      Log level for mock servers to write to the log file (defaults to info) [possible values: error, warn, info, debug, trace, none]
      --no-term-log              Turns off using terminal ANSI escape codes
      --no-file-log              Do not log to an output file

```

```console
$ pact_cli mock list --help
Lists all the running mock servers

Usage: pact_cli mock list [OPTIONS]

Options:
      --help                 Print help and exit
  -v, --version              Print version information and exit
  -p, --port <port>          port the master mock server runs on (defaults to 8080)
  -h, --host <host>          hostname the master mock server runs on (defaults to localhost)
  -l, --loglevel <loglevel>  Log level for mock servers to write to the log file (defaults to info) [possible values: error, warn, info, debug, trace, none]
      --no-term-log          Turns off using terminal ANSI escape codes
      --no-file-log          Do not log to an output file

```

```console
$ pact_cli mock create --help
Creates a new mock server from a pact file

Usage: pact_cli mock create [OPTIONS] --file <file>

Options:
  -f, --file <file>          the pact file to define the mock server
      --help                 Print help and exit
  -c, --cors-preflight       Handle CORS pre-flight requests
  -v, --version              Print version information and exit
  -p, --port <port>          port the master mock server runs on (defaults to 8080)
  -h, --host <host>          hostname the master mock server runs on (defaults to localhost)
  -l, --loglevel <loglevel>  Log level for mock servers to write to the log file (defaults to info) [possible values: error, warn, info, debug, trace, none]
      --no-term-log          Turns off using terminal ANSI escape codes
      --no-file-log          Do not log to an output file

```


```console
$ pact_cli mock verify --help
Verify the mock server by id or port number, and generate a pact file if all ok

Usage: pact_cli mock verify [OPTIONS]

Options:
      --help
          Print help and exit
  -i, --mock-server-id <mock-server-id>
          the ID of the mock server
  -m, --mock-server-port <mock-server-port>
          the port number of the mock server
  -v, --version
          Print version information and exit
  -p, --port <port>
          port the master mock server runs on (defaults to 8080)
  -h, --host <host>
          hostname the master mock server runs on (defaults to localhost)
  -l, --loglevel <loglevel>
          Log level for mock servers to write to the log file (defaults to info) [possible values: error, warn, info, debug, trace, none]
      --no-term-log
          Turns off using terminal ANSI escape codes
      --no-file-log
          Do not log to an output file

```

```console
$ pact_cli mock shutdown --help
Shutdown the mock server by id or port number, releasing all its resources

Usage: pact_cli mock shutdown [OPTIONS]

Options:
      --help
          Print help and exit
  -i, --mock-server-id <mock-server-id>
          the ID of the mock server
  -m, --mock-server-port <mock-server-port>
          the port number of the mock server
  -v, --version
          Print version information and exit
  -p, --port <port>
          port the master mock server runs on (defaults to 8080)
  -h, --host <host>
          hostname the master mock server runs on (defaults to localhost)
  -l, --loglevel <loglevel>
          Log level for mock servers to write to the log file (defaults to info) [possible values: error, warn, info, debug, trace, none]
      --no-term-log
          Turns off using terminal ANSI escape codes
      --no-file-log
          Do not log to an output file

```


```console
$ pact_cli mock shutdown-master --help
Performs a graceful shutdown of the master server (displayed when it started)

Usage: pact_cli mock shutdown-master [OPTIONS] --server-key <server-key>

Options:
      --help                     Print help and exit
  -k, --server-key <server-key>  the server key of the master server
      --period <period>          the period of time in milliseconds to allow the server to shutdown (defaults to 100ms)
  -v, --version                  Print version information and exit
  -p, --port <port>              port the master mock server runs on (defaults to 8080)
  -h, --host <host>              hostname the master mock server runs on (defaults to localhost)
  -l, --loglevel <loglevel>      Log level for mock servers to write to the log file (defaults to info) [possible values: error, warn, info, debug, trace, none]
      --no-term-log              Turns off using terminal ANSI escape codes
      --no-file-log              Do not log to an output file

```

## Pact Stub Server CLI

```console
$ pact_cli stub --help
Pact Stub Server 0.0.4

Usage: pact_cli stub [OPTIONS]

Options:
  -l, --loglevel <loglevel>
          Log level (defaults to info) [default: info] [possible values: error, warn, info, debug, trace, none]
  -f, --file <file>
          Pact file to load (can be repeated)
  -d, --dir <dir>
          Directory of pact files to load (can be repeated)
  -e, --extension <ext>
          File extension to use when loading from a directory (default is json)
  -u, --url <url>
          URL of pact file to fetch (can be repeated)
  -b, --broker-url <broker-url>
          URL of the pact broker to fetch pacts from [env: PACT_BROKER_BASE_URL=]
      --user <user>
          User and password to use when fetching pacts from URLS or Pact Broker in user:password form
  -t, --token <token>
          Bearer token to use when fetching pacts from URLS or Pact Broker
  -p, --port <port>
          Port to run on (defaults to random port assigned by the OS)
  -o, --cors
          Automatically respond to OPTIONS requests and return default CORS headers
      --cors-referer
          Set the CORS Access-Control-Allow-Origin header to the Referer
      --insecure-tls
          Disables TLS certificate validation
  -s, --provider-state <provider-state>
          Provider state regular expression to filter the responses by
      --provider-state-header-name <provider-state-header-name>
          Name of the header parameter containing the provider state to be used in case multiple matching interactions are found
      --empty-provider-state
          Include empty provider states when filtering with --provider-state
      --consumer-name <consumer-name>
          Consumer name or regex to use to filter the Pacts fetched from the Pact broker (can be repeated)
      --provider-name <provider-name>
          Provider name or regex to use to filter the Pacts fetched from the Pact broker (can be repeated)
  -v, --version
          Print version information
  -h, --help
          Print help

```

## Pact Verifier CLI

```console
$ pact_cli verifier --help
Usage: pact_cli verifier [OPTIONS]

Options:
      --help     Print help and exit
  -v, --version  Print version information and exit

Logging options:
  -l, --loglevel <loglevel>  Log level to emit log events at (defaults to warn) [possible values: error, warn, info, debug, trace, none]
      --pretty-log           Emits excessively pretty, multi-line logs, optimized for human readability.
      --full-log             This emits human-readable, single-line logs for each event that occurs, with the current span context displayed before the formatted representation of the event.
      --compact-log          Emit logs optimized for short line lengths.
  -j, --json <json-file>     Generate a JSON report of the verification
  -x, --junit <junit-file>   Generate a JUnit XML report of the verification (requires the junit feature)
      --no-colour            Disables ANSI escape codes in the output [aliases: no-color]

Loading pacts options:
  -f, --file <file>
          Pact file to verify (can be repeated)
  -d, --dir <dir>
          Directory of pact files to verify (can be repeated)
  -u, --url <url>
          URL of pact file to verify (can be repeated)
  -b, --broker-url <broker-url>
          URL of the pact broker to fetch pacts from to verify (requires the provider name parameter) [env: PACT_BROKER_BASE_URL=]
      --webhook-callback-url <webhook-callback-url>
          URL of a Pact to verify via a webhook callback. Requires the broker-url to be set.
      --ignore-no-pacts-error
          Do not fail if no pacts are found to verify

Authentication options:
      --user <user>          Username to use when fetching pacts from URLS [env: PACT_BROKER_USERNAME=]
      --password <password>  Password to use when fetching pacts from URLS [env: PACT_BROKER_PASSWORD=]
  -t, --token <token>        Bearer token to use when fetching pacts from URLS [env: PACT_BROKER_TOKEN=]

Provider options:
  -h, --hostname <hostname>
          Provider hostname (defaults to localhost)
  -p, --port <port>
          Provider port (defaults to protocol default 80/443)
      --transport <transport>
          Provider protocol transport to use (http, https, grpc, etc.) [default: http]
      --transports <transports>
          Allows multiple protocol transports to be configured (http, https, grpc, etc.) with their associated port numbers separated by a colon. For example, use --transports http:8080 grpc:5555 to configure both.
  -n, --provider-name <provider-name>
          Provider name (defaults to provider)
      --base-path <base-path>
          Base path to add to all requests
      --request-timeout <request-timeout>
          Sets the HTTP request timeout in milliseconds for requests to the target API and for state change requests.
  -H, --header <custom-header>
          Add a custom header to be included in the calls to the provider. Values must be in the form KEY=VALUE, where KEY and VALUE contain ASCII characters (32-127) only. Can be repeated.
      --disable-ssl-verification
          Disables validation of SSL certificates

Provider state options:
  -s, --state-change-url <state-change-url>
          URL to post state change requests to
      --state-change-as-query
          State change request data will be sent as query parameters instead of in the request body
      --state-change-teardown
          State change teardown requests are to be made after each interaction

Filtering interactions:
      --filter-description <filter-description>
          Only validate interactions whose descriptions match this filter (regex format) [env: PACT_DESCRIPTION=]
      --filter-state <filter-state>
          Only validate interactions whose provider states match this filter (regex format) [env: PACT_PROVIDER_STATE=]
      --filter-no-state
          Only validate interactions that have no defined provider state [env: PACT_PROVIDER_NO_STATE=]
  -c, --filter-consumer <filter-consumer>
          Consumer name to filter the pacts to be verified (can be repeated)

Publishing options:
      --publish
          Enables publishing of verification results back to the Pact Broker. Requires the broker-url and provider-version parameters.
      --provider-version <provider-version>
          Provider version that is being verified. This is required when publishing results.
      --build-url <build-url>
          URL of the build to associate with the published verification results.
      --provider-tags <provider-tags>
          Provider tags to use when publishing results. Accepts comma-separated values.
      --provider-branch <provider-branch>
          Provider branch to use when publishing results

Pact Broker options:
      --consumer-version-tags <consumer-version-tags>
          Consumer tags to use when fetching pacts from the Broker. Accepts comma-separated values.
      --consumer-version-selectors <consumer-version-selectors>
          Consumer version selectors to use when fetching pacts from the Broker. Accepts a JSON string as per https://docs.pact.io/pact_broker/advanced_topics/consumer_version_selectors/. Can be repeated.
      --enable-pending
          Enables Pending Pacts
      --include-wip-pacts-since <include-wip-pacts-since>
          Allow pacts that don't match given consumer selectors (or tags) to  be verified, without causing the overall task to fail. For more information, see https://pact.io/wip

```

## Pact Broker Docker

```console
$ pact_cli docker --help
Run the Pact Broker as a Docker container

Usage: pact_cli docker [COMMAND]

Commands:
  start   Start the Pact Broker as a Docker container
  stop    Stop the Pact Broker Docker container
  remove  Remove the Pact Broker Docker container
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help

```
