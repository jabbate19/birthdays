use chrono::offset::Local;
use chrono::Datelike;
use reqwest::Client;
use std::collections::HashMap;
use std::env;
use std::vec::Vec;

use birthdays::ldap::client as ldap_client;

#[tokio::main]
async fn main() {
    let LDAP_DN = env::var("LDAP_BIND_DN").unwrap();
    let LDAP_PW = env::var("LDAP_BIND_PW").unwrap();
    let SLACK_TOKEN = env::var("SLACK_TOKEN").unwrap();
    let mut ldap_client = ldap_client::LdapClient::new(
        &LDAP_DN,
        &LDAP_PW,
    ).await;
    println!("LDAP client initialized");
    println!("{}", SLACK_TOKEN);
    let d = Local::today().naive_local();
    let date_string = format!("{:02}{:02}", &d.month(), &d.day());
    let members = ldap_client.search_birthday(&date_string).await;
    let mut users: Vec<String> = Vec::new();
    for member in members {
        if &member.birthday[4..8] == &date_string {
            users.push(match member.slackuid {
                Some(uid) => format!("<@{}>", uid),
                None => member.cn,
            });
        }
    }
    let mut output = String::from("Happy Birthday to ");
    if users.len() == 1 {
        output.push_str(&format!("{}!", users[0]));
    } else if users.len() == 2 {
        output.push_str(&format!("{} and {}!", users[0], users[1]));
    } else {
        for i in [..users.len()-1] {
            let s = &users[i];
            output.push_str(&format!("{:?}, ", s));
        }
        output.push_str(&format!("and {}!", users[users.len()-1]));
    }
    let mut map = HashMap::new();
    map.insert("channel", "CBBK03MQ9");
    map.insert("text", &output);
    let client = Client::new();
    let res = client.post("https://slack.com/api/chat.postMessage")
                    .header("Content-type", "application/json")
                    .header("Authorization", &format!("Bearer {}", SLACK_TOKEN))
                    .json(&map)
                    .send()
                    .await;
}

