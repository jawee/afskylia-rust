use std::{net::{TcpListener, TcpStream}, io::{BufRead, BufReader, Write}, collections::HashMap};

use crate::server;

pub fn run() {

    let mut hash_map: HashMap<String, String> = HashMap::new();

    hash_map.insert(String::from("/"), HTML.to_string());
    hash_map.insert(String::from("/style.css"), CSS.to_string());
    hash_map.insert(String::from("404"), NOT_FOUND.to_string());

    server::start(hash_map);
}

const CSS: &str = r#"
body {
    background-color: #ccc;
}
"#;


const HTML: &str = r#"
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Hello!</title>
    <link rel="stylesheet" href="style.css">
  </head>
  <body>
    <h1>Hello!</h1>
    <p>Hi from Rust</p>
  </body>
</html>"#;

const NOT_FOUND: &str = r#"
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>404 - Not Found</title>
  </head>
  <body>
    <h1>Custom 404 - Not Found!</h1>
    <p>Sorry, I don't know what you're asking for.</p>
  </body>
</html>"#;
