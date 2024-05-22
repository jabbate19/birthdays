use ldap3::{drive, Ldap, LdapConnAsync, SearchEntry};
use rand::prelude::SliceRandom;
use rand::SeedableRng;
use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    AsyncResolver,
};

use super::user::LdapUser;

const SEARCH_ATTRS: [&str; 12] = [
    "cn",
    "dn",
    "uid",
    "memberOf",
    "krbPrincipalName",
    "mail",
    "mobile",
    "ibutton",
    "drinkBalance",
    "birthday",
    "slackuid",
    "nsaccountlock",
];

#[derive(Clone)]
pub struct LdapClient {
    ldap: Ldap,
}

impl LdapClient {
    pub async fn new(bind_dn: &str, bind_pw: &str) -> Self {
        let servers = get_ldap_servers().await;
        let (conn, mut ldap) = LdapConnAsync::new(
            servers
                .choose(&mut rand::rngs::StdRng::from_entropy())
                .unwrap(),
        )
        .await
        .unwrap();
        drive!(conn);

        ldap.simple_bind(bind_dn, bind_pw).await.unwrap();

        LdapClient { ldap }
    }

    pub async fn search_birthday(&mut self, query: &str) -> Vec<LdapUser> {
        self.ldap.with_timeout(std::time::Duration::from_secs(5));
        let (results, _result) = self
            .ldap
            .search(
                "cn=users,cn=accounts,dc=csh,dc=rit,dc=edu",
                ldap3::Scope::Subtree,
                &format!("(&(birthday=*{query}*)(!(nsaccountlock=TRUE)))"),
                SEARCH_ATTRS,
            )
            .await
            .unwrap()
            .success()
            .unwrap();

        results
            .iter()
            .map(|result| {
                let user = SearchEntry::construct(result.to_owned());
                LdapUser::from_entry(&user)
            })
            .collect()
    }
}

async fn get_ldap_servers() -> Vec<String> {
    let resolver =
        AsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default()).unwrap();
    let response = resolver.srv_lookup("_ldap._tcp.csh.rit.edu").await.unwrap();

    // TODO: Make sure servers are working
    response
        .iter()
        .map(|record| {
            format!(
                "ldaps://{}",
                record.target().to_string().trim_end_matches('.')
            )
        })
        .collect()
}
