use super::constants::TABLE_ROW_LIMIT;
use graphql_client::Error;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct TableMetadata {
    pub total_records: String,
    pub displayed_records: i64,
}

impl Default for TableMetadata {
    fn default() -> Self {
        TableMetadata {
            total_records: "all".to_string(),
            displayed_records: TABLE_ROW_LIMIT,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MyError {
    NetworkError(String),
    ParseError(String), // other error variants
    GraphQLError(Vec<Error>),
    GraphQLEmpty(String),
    UrlParseError(String),
}

impl From<url::ParseError> for MyError {
    fn from(err: url::ParseError) -> MyError {
        MyError::UrlParseError(err.to_string())
    }
}

impl From<reqwest::Error> for MyError {
    fn from(err: reqwest::Error) -> Self {
        MyError::NetworkError(err.to_string())
    }
}

pub enum Status {
    Pending,
    Complete,
    Unknown,
}

pub enum ColorVariant {
    Green,
    Blue,
    Grey,
    Transparent,
    DarkBlue,
}

#[derive(Clone)]
pub struct NavEntry {
    pub href: String,
    pub text: String,
    pub icon: NavIcon,
    pub sub_entries: Option<Vec<NavEntry>>,
    pub disabled: bool,
    pub number_bubble: Option<usize>,
}

impl Default for NavEntry {
    fn default() -> Self {
        NavEntry {
            sub_entries: None,
            disabled: false,
            number_bubble: None,
            href: String::new(),
            text: String::new(),
            icon: NavIcon::Accounts,
        }
    }
}

#[derive(Clone)]
pub struct UrlParamSelectOptions {
    pub is_boolean_option: bool,
    pub cases: Vec<String>,
}

#[derive(Clone, PartialEq)]
pub enum NavIcon {
    Blocks,
    Transactions,
    Accounts,
    SNARKs,
    Staking,
    Send,
    ZKApps,
    Tokens,
    Addresses,
    FeeTransfers,
    Analytics,
    More,
}
