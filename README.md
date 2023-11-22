# URL Parser

This Rust library provides functionality for parsing URLs and extracting relevant components. It includes a command-line interface for parsing URLs from a string.

## Features

- **Parse URLs** from strings.
- Support for **basic URL components** such as scheme, host, port, path, query, fragment, and user information.
- **Command-line interface** (CLI) for parsing URLs.
- **Error Handling**: Utilizes the thiserror crate for robust error handling.

## Getting Started

### Dependencies

Add the following dependencies to your `Cargo.toml` file:

```toml
[dependencies]
clap = "2.33.0"
thiserror = "1.0.44"
peg = "0.8.0"
```
### Command-Line Interface
The library includes a command-line interface for parsing URLs. To use it, build the project at first by :

`cargo build`

and then parse your URL:

`cargo run -- parse <URL>`

#### Additional subcommands

- **Help**: for more information about the project

`cargo run -- help`

- **Credits**: Print credit information

`cargo run -- credits`

### Testing

The library includes several tests to ensure proper functionality. To run the tests, use the following command:

`cargo test`

## Structure 
Project has a simple structure with 
- **lib.rs** as library source code, 
- **main.rs** as main application source code
- **tests folder** for tests files
- **Cargo.toml** for Rust package configuration file
```
url_parser_rust/
│
├── src/
│   ├── lib.rs      
│   ├── main.rs           
│
├── tests/
│   ├── integration_tests.rs   
│
├── Cargo.toml       
├── README.md           
├── LICENSE               
└── .gitignore
```
## Examples

You can parse your URL inside the **main** like this:
```
use url_parser_rust::{parse_url, create_user_info, ParseUrlException};

fn main() {
// Example: Parse a simple URL
    let simple_url = "https://www.example.com";
    match parse_url(simple_url) {
    Ok(parsed_url) => {
    println!("Parsed URL: {:#?}", parsed_url);
    }
    Err(err) => {
    eprintln!("Error parsing URL: {}", err);
    }
}
```
### Command-Line Interface

#### First
We can you CLI to parse your URL fast. For example for this URL https://www.ukma.edu.ua we can write command:

`cargo run -- parse https://www.ukma.edu.ua`

And the output:
```
ParsedUrl {
scheme: "https",
host: "www.ukma.edu.ua",
port: None,
path: "",
query: None,
fragment: None,
userinfo: None,
}
```
#### Second
You can try URL with the port:

`cargo run -- parse http://localhost:8888/tree/Desktop/uni`

With the result:

```
ParsedUrl {
scheme: "http",
host: "localhost",
port: Some(
8888,
),
path: "tree/Desktop/uni",
query: None,
fragment: None,
userinfo: None,
}
```
#### Third

You can parse URL with the login and password information:

`cargo run -- parse https://Sofiia:password1234@example.com:8080/resource?param=value`

With output:
```
ParsedUrl {
scheme: "https",
host: "example.com",
port: Some(
8080,
),
path: "resource",
query: Some(
"param=value",
),
fragment: None,
userinfo: Some(
UserInfo {
login: "Sofiia",
password: "password1234",
},
),
}
```
### Testing
You can create your own tests in this way:
```
    fn test_parse_simple_url() {
        let url = "https://www.example.com";
        let parsed_url = parse_url(url).expect("URL is not correct");

        assert_eq!(parsed_url.scheme, "https");
        assert_eq!(parsed_url.host, "www.example.com");
        assert_eq!(parsed_url.port, None);
        assert_eq!(parsed_url.path, "");
        assert_eq!(parsed_url.query, None);
        assert_eq!(parsed_url.fragment, None);
        assert_eq!(parsed_url.userinfo, None);
    }
```
See more examples in tests directory.

## Author
This URL parser library in Rust was created by Sofiia Yeremeieva.