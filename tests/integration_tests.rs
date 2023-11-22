use url_parser_rust::{parse_url, create_user_info};

#[test]
fn test_parse_scheme_and_host() {
    let url = "https://www.ukma.edu.ua";
    let parsed_url = parse_url(url).expect("URL is not correct");

    assert_eq!(parsed_url.scheme, "https");
    assert_eq!(parsed_url.host, "www.ukma.edu.ua");
    assert_eq!(parsed_url.port, None);
    assert_eq!(parsed_url.path, "");
    assert_eq!(parsed_url.query, None);
    assert_eq!(parsed_url.fragment, None);
    assert_eq!(parsed_url.userinfo, None);
}

#[test]
fn test_parse_all_elements() {
    let url = "http://localhost:8088/api/forms?id=1&name=sofiia";
    let parsed_url = parse_url(url).expect("URL is not correct");

    assert_eq!(parsed_url.scheme, "http");
    assert_eq!(parsed_url.host, "localhost");
    assert_eq!(parsed_url.port, Some(8088));
    assert_eq!(parsed_url.path, "api/forms");
    assert_eq!(parsed_url.query, Some("id=1&name=sofiia".to_string()));
    assert_eq!(parsed_url.userinfo, None);
}

#[test]
fn parse_github_io() {
    let url = "https://microsoft.github.io/Data-Science-For-Beginners?id=getting-started";
    let parsed_url = parse_url(url).expect("URL not parsed");

    assert_eq!(parsed_url.scheme, "https");
    assert_eq!(parsed_url.host, "microsoft.github.io");
    assert_eq!(parsed_url.port, None);
    assert_eq!(parsed_url.path, "Data-Science-For-Beginners");
    assert_eq!(parsed_url.query, Some("id=getting-started".to_string()));
    assert_eq!(parsed_url.fragment, None);
    assert_eq!(parsed_url.userinfo, None);
}

#[test]
fn test_parse_host_and_port() {
    let url = "http://www.ukma.edu.ua:8081";
    let parsed_url = parse_url(url).expect("URL not parsed");

    assert_eq!(parsed_url.scheme, "http");
    assert_eq!(parsed_url.host, "www.ukma.edu.ua");
    assert_eq!(parsed_url.port, Some(8081));
    assert_eq!(parsed_url.path, "");
    assert_eq!(parsed_url.query, None);
    assert_eq!(parsed_url.fragment, None);
    assert_eq!(parsed_url.userinfo, None);
}

#[test]
fn test_userinfo() {
    let url = "https://user:password@example.com:8080/resource?param=value";
    let parsed_url = parse_url(url).expect("URL not parsed");

    let user_info = create_user_info("user", "password");

    assert_eq!(parsed_url.scheme, "https");
    assert_eq!(parsed_url.host, "example.com");
    assert_eq!(parsed_url.port, Some(8080));
    assert_eq!(parsed_url.path, "resource");
    assert_eq!(parsed_url.query, Some("param=value".to_string()));
    assert_eq!(parsed_url.fragment, None);
    assert_eq!(parsed_url.userinfo, Some(user_info));
}