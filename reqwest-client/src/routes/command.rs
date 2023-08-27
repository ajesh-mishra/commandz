use std::io;
use std::process;

use chrono::Utc;
use dotenv_codegen::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use dialoguer::MultiSelect;
use utils::get_last_commands;

use crate::arguments::Verb;
use crate::utils;

pub async fn get_response(verb: Verb) {
    let client = Client::builder()
        .build()
        .expect("Cannot create reqwest builder");

    match verb {
        Verb::GET => get(client).await,
        Verb::POST => post(client).await,
        _ => {}
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Command {
    id: i32,
    command_name: String,
    description: String,
    command_type: String,
    created_on: chrono::NaiveDateTime,
}

async fn get(client: Client) {
    let res = client
        .get(dotenv!("remote_host"))
        .send()
        .await
        .expect("Get request failed!");

    let res = res
        .json::<Vec<Command>>()
        .await
        .expect("Response Conversion Failed");

    println!("{:#?}", res);
}

async fn post(client: Client) {
    let items = get_last_commands(2);

    let selections = MultiSelect::new()
        .items(&items)
        .interact()
        .expect("Can't get selection!");

    if selections.len() < 1 {
        process::exit(0);
    }

    println!("\n\n Commands Selected:");
    println!(" -----------------\n");

    let mut command_name = String::from("");

    for (index, &selection) in selections.iter().enumerate() {
        println!("{}. {}", index, items[selection]);
        command_name.push_str(items[selection].as_str());
        command_name.push_str(";");
    }

    let mut command_type = String::new();

    println!("\n\n Command Type:");
    println!(" ------------\n");

    io::stdin().read_line(&mut command_type)
        .expect("error: unable to read command type");

    let mut description = String::new();

    println!("\n\n Description:");
    println!(" -----------\n");

    io::stdin().read_line(&mut description)
        .expect("error: unable to read user description");

    // let command_payload = Command {
    //     id: 0,
    //     command_name: "docker 63qa exec -it sh".to_owned(),
    //     description: "Dummy Description".to_owned(),
    //     command_type: "dummy".to_owned(),
    //     created_on: Utc::now().naive_utc(),
    // };

    let command_payload = Command {
        id: 0,
        command_name,
        description,
        command_type,
        created_on: Utc::now().naive_utc(),
    };

    let res = client
        .post(dotenv!("remote_host"))
        .json(&command_payload)
        .send()
        .await
        .expect("Post Request Failed: Make sure you have 'remote_host' defined in .evn file!");

    let res = res
        .text()
        .await
        .expect("Server Response Conversion Failed!");

    println!("Response: {}", res);
}

