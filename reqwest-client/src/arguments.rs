use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, default_value_t = String::from("command"))]
    pub route: String,

    // #[arg(short, long, default_value_t = String::from("request.json"))]
    // pub payload: String,

    // #[arg(short, long, default_value_t = 1)]
    // pub tail: i32,

    #[command(subcommand)]
    pub verb: Verb,
}

#[derive(Debug, Subcommand)]
pub enum Verb {
    GET,
    POST,
    DELETE,
    PUT,
    PATCH,
}

pub fn get_args() -> (String, Verb) {
    let cli = Cli::parse();

    // let payload: String = fs::read_to_string(cli.payload.to_owned())
    //     .expect(format!("Can't read from file {}", cli.payload).as_str());

    (cli.route, cli.verb)
}