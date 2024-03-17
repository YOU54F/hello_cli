use clap::{App, Arg};

fn main() {
    let matches = App::new("pact")
    .subcommand(Command::new("examples").about("download example projects")
        .arg(
            Arg::with_name("type")
                .short("t")
                .long("type")
                .takes_value(true)
                .possible_values(&["bdct", "cdct", "workshops"])
                .required(true)
                .help("Specify the project type (bdct, cdct, workshops)"),
        )
        .arg(
            Arg::with_name("project")
                .short("p")
                .long("project")
                .takes_value(true)
                .help("Specify the project to download"),
        )
        .arg(
            Arg::with_name("all")
                .short("a")
                .long("all")
                .help("Download all projects"),
        ))
    .subcommand(Command::new("setup").about("setup pact")
            .subcommand(Command::new("install").about("install pact")
                .arg(
                    Arg::with_name("language")
                        .short("l")
                        .long("language")
                        .takes_value(true)
                        .possible_values(&["js", "golang", "ruby", "python", "java", ".net", "rust", "php"])
                        .required(true)
                        .help("Specify the language to install pact for"),
                ))
            .subcommand(Command::new("new").about("create new pact project")
                .arg(
                    Arg::with_name("language")
                        .short("l")
                        .long("language")
                        .takes_value(true)
                        .possible_values(&["js", "golang", "ruby", "python", "java", ".net", "rust", "php"])
                        .required(true)
                        .help("Specify the language for the new pact project"),
                ))
            .subcommand(Command::new("link").about("link pact project")
                .arg(
                    Arg::with_name("language")
                        .short("l")
                        .long("language")
                        .takes_value(true)
                        .possible_values(&["js", "golang", "ruby", "python", "java", ".net", "rust", "php"])
                        .required(true)
                        .help("Specify the language of the pact project to link"),
                ))
            .subcommand(Command::new("create").about("create pact contract")
                .arg(
                    Arg::with_name("language")
                        .short("l")
                        .long("language")
                        .takes_value(true)
                        .possible_values(&["js", "golang", "ruby", "python", "java", ".net", "rust", "php"])
                        .required(true)
                        .help("Specify the language for creating the pact contract"),
                ))
            .subcommand(Command::new("issue").about("create pact issue")
                .arg(
                    Arg::with_name("language")
                        .short("l")
                        .long("language")
                        .takes_value(true)
                        .possible_values(&["js", "golang", "ruby", "python", "java", ".net", "rust", "php"])
                        .required(true)
                        .help("Specify the language for creating the pact issue"),
                ))
            .subcommand(Command::new("docs").about("open pact documentation")
                .arg(
                    Arg::with_name("language")
                        .short("l")
                        .long("language")
                        .takes_value(true)
                        .possible_values(&["js", "golang", "ruby", "python", "java", ".net", "rust", "php"])
                        .required(true)
                        .help("Specify the language for opening the pact documentation"),
                ))
    
)

        .get_matches();
