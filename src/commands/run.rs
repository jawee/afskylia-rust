use std::{net::{TcpListener, TcpStream}, io::{BufRead, BufReader, Write}};

pub fn run() {
    let listener = TcpListener::bind("127.0.0.1:1313").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    println!("{}", request_line);

    // GET /asdf HTTP/1.1
    let mut found = false;
    let mut char_vec: Vec<char> = vec![];
    for c in request_line.chars() {
        if c == ' ' {
            if found {
                break;
            }
            found = true;
            continue;
        }

        if found {
            char_vec.push(c);
        }
    }
    // let path = String::from(char_vec);
    let path = char_vec.iter().collect::<String>();
    println!("{}", path);

    let (status_line, html) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", HTML)
    } else {
        ("HTTP/1.1 404 NOT FOUND", NOT_FOUND)
    };

    let length = html.len();

    let response = 
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{html}");

    stream.write_all(response.as_bytes()).unwrap();
}

const HTML: &str = r#"
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Hello!</title>
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
    <title>Hello!</title>
  </head>
  <body>
    <h1>Oops!</h1>
    <p>Sorry, I don't know what you're asking for.</p>
  </body>
</html>"#;
