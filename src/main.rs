use chrono::offset::Local;
use chrono::Datelike;
use std::env;
use std::vec::Vec;

use birthday::ldap::client as ldap_client;

#[tokio::main]
async fn main() {
    let LDAP_DN = env::var("LDAP_BIND_DN").unwrap();
    let LDAP_PW = env::var("LDAP_BIND_PW").unwrap();
    let mut ldap_client = ldap_client::LdapClient::new(
        &LDAP_DN,
        &LDAP_PW,
    ).await;
    println!("LDAP client initialized");
    let d = Local::today().naive_local();
    let date_string = format!("{:02}{:02}", &d.month(), &d.day());
    println!("{}", date_string);
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
    println!("{}", output);
}

