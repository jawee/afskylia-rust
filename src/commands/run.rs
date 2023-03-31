use std::collections::HashMap;

use crate::server;

pub fn run() {

    let image_byte_vec: Vec<u8> = Vec::from([137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 1, 0, 0, 0, 1, 8, 6, 0, 0, 0, 31, 21, 196, 137, 0, 0, 0, 1, 115, 82, 71, 66, 0, 174, 206, 28, 233, 0, 0, 0, 13, 73, 68, 65, 84, 24, 87, 99, 248, 195, 118, 249, 63, 0, 6, 172, 2, 213, 240, 132, 102, 115, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130]);
    let mut hash_map: HashMap<String, Vec<u8>> = HashMap::new();

    hash_map.insert(String::from("/"), HTML.to_string().as_bytes().to_vec());
    hash_map.insert(String::from("/style.css"), CSS.to_string().as_bytes().to_vec());
    hash_map.insert(String::from("404"), NOT_FOUND.to_string().as_bytes().to_vec());
    hash_map.insert(String::from("/1x1.png"), image_byte_vec.as_slice().to_vec());
    hash_map.insert(String::from("/script.js"), JS.to_string().as_bytes().to_vec());

    server::start(&hash_map);
}

// const IMAGE = [137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 1, 0, 0, 0, 1, 8, 6, 0, 0, 0, 31, 21, 196, 137, 0, 0, 0, 1, 115, 82, 71, 66, 0, 174, 206, 28, 233, 0, 0, 0, 13, 73, 68, 65, 84, 24, 87, 99, 248, 195, 118, 249, 63, 0, 6, 172, 2, 213, 240, 132, 102, 115, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130];

const CSS: &str = r#"
body {
    background-color: #ccc;
}
"#;

const JS: &str = r#"
    console.log("hello");
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
    <script src="script.js"></script>
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
