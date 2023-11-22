use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseUrlException {
    #[error("Error parsing URL: {0}")]
    ParsingException(String),
}

#[derive(Debug)]
pub struct ParsedUrl {
    pub scheme: String,
    pub host: String,
    pub port: Option<u16>,
    pub path: String,
    pub query: Option<String>,
    pub fragment: Option<String>,
    pub userinfo: Option<UserInfo>,
}

#[derive(Debug)]
pub struct UserInfo {
    pub login: String,
    pub password: String,
}

impl UserInfo {
    fn new() -> Self {
        UserInfo {
            login: String::new(),
            password: String::new(),
        }
    }
}

impl PartialEq for UserInfo {
    fn eq(&self, other: &Self) -> bool {
        self.login == other.login && self.password == other.password
    }
}

impl ParsedUrl {
    fn new() -> Self {
        ParsedUrl {
            scheme: String::new(),
            host: String::new(),
            port: None,
            path: String::new(),
            query: None,
            fragment: None,
            userinfo: None,
        }
    }
}

pub fn create_user_info(login: &str, password: &str) -> UserInfo {
    let mut user_info = UserInfo::new();
    user_info.password = password.parse().unwrap();
    user_info.login = login.parse().unwrap();
    return user_info;
}

fn parse_port(port_str: &str) -> Option<u16> {
    port_str.parse().ok()
}

pub fn parse_url(url_string: &str) -> Result<ParsedUrl, ParseUrlException> {
    let parts: Vec<&str> = url_string.split("://").collect();

    if parts.len() != 2 {
        return Err(ParseUrlException::ParsingException("Invalid URL format".to_string()));
    }

    let scheme = parts[0].to_string();
    let rest = parts[1];

    let mut parsed_url = ParsedUrl::new();
    parsed_url.scheme = scheme;

    let rest_parts: Vec<&str> = rest.splitn(2, "/").collect();

    let (authority, path_query_fragment) = match rest_parts.len() {
        1 => (rest_parts[0], ""),
        2 => (rest_parts[0], rest_parts[1]),
        _ => return Err(ParseUrlException::ParsingException("Invalid URL format".to_string())),
    };

    let mut authority_new = "";

    if authority.contains("@") {
        let user_info_and_host: Vec<&str> = authority.split("@").collect();
        let user_info: Vec<&str> = user_info_and_host[0].split(":").collect();

        let mut user_info_obj = UserInfo::new();

        user_info_obj.login = user_info[0].parse().unwrap();
        user_info_obj.password = user_info[1].parse().unwrap();

        parsed_url.userinfo = Some(user_info_obj);
        authority_new = user_info_and_host[1];
    }

    if !authority_new.is_empty() {
        let authority_parts: Vec<&str> = authority_new.splitn(2, ":").collect();
        parsed_url.host = authority_parts[0].to_string();
        parsed_url.port = authority_parts.get(1).and_then(|port_str| parse_port(port_str));
    } else {
        let authority_parts: Vec<&str> = authority.splitn(2, ":").collect();
        parsed_url.host = authority_parts[0].to_string();
        parsed_url.port = authority_parts.get(1).and_then(|port_str| parse_port(port_str));
    }

    let path_query_fragment_parts: Vec<&str> = path_query_fragment.splitn(3, "?").collect();
    parsed_url.path = path_query_fragment_parts[0].to_string();
    parsed_url.query = path_query_fragment_parts.get(1).map(|q| q.to_string());
    parsed_url.fragment = path_query_fragment_parts.get(2).map(|f| f.to_string());

    Ok(parsed_url)
}

