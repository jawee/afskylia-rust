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
    let path = get_request_path(&request_line);
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

fn get_request_path(request_line: &str) -> String {
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

    let path = get_request_path_string(char_vec);
    return path;
}

fn get_request_path_string(char_vec: Vec<char>) -> String {
    let path = char_vec.iter().collect::<String>();
    return path;
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


#[cfg(test)]
mod tests {
    use std::io::Error;

    use super::{get_request_path, get_request_path_string};


    #[test]
    fn test_get_request_path() {
        let request_line = "GET /path/to/file.html HTTP/1.1";

        let path = get_request_path(request_line);

        assert_eq!(path, "/path/to/file.html".to_string());
    }

    #[test]
    fn test_get_request_path_string() -> Result<(), Error> {
        let path_str = "/path/to/file.html";
        let char_vec = path_str.chars().collect();

        let path = get_request_path_string(char_vec);

        assert_eq!(path_str, path);

        return Ok(());
    }

}
