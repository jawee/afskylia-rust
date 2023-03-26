use std::{collections::HashMap, net::{TcpListener, TcpStream}, io::{BufReader, BufRead, Write}};

pub fn start(content_map: HashMap<String, String>) {
    let listener = TcpListener::bind("127.0.0.1:1313").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream, content_map.clone());
    }
}

fn handle_connection(mut stream: TcpStream, content_map: HashMap<String, String>) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    println!("{}", request_line);

    // GET /asdf HTTP/1.1
    let path = get_request_path(&request_line);

    let (status_line, html) = match get_content_for_path(path, content_map) {
        None => {
            ("HTTP/1.1 404 NOT FOUND", get_not_found_content())
        },
        Some(t) => {
            ("HTTP/1.1 200 OK", t)
        }
    };

    let length = html.len();

    let response = 
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{html}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn get_not_found_content() -> String {
    return NOT_FOUND.to_string();
}

fn get_content_for_path(path: String, content_map: HashMap<String, String>) -> Option<String> {
    let maybe_content = content_map.get(&path).cloned();

    let maybe_content = match maybe_content {
        Some(_) => maybe_content,
        None => get_content_for_path("404".to_string(), content_map)
    };

    let content = match maybe_content {
        Some(t) => t,
        None => NOT_FOUND.to_string()
    };
    
    return Some(content);
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

const NOT_FOUND: &str = r#"
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>404 - Not Found</title>
  </head>
  <body>
    <h1>404 - Not Found!</h1>
    <p>Sorry, I don't know what you're asking for.</p>
  </body>
</html>"#;

#[cfg(test)]
mod tests {

    use std::{collections::HashMap, io::Error};

    use crate::server::server::NOT_FOUND;

    use super::{get_request_path, get_request_path_string, get_content_for_path};

    #[test]
    fn test_get_content_for_path() {
        let path = "/".to_string();
        let mut content_map: HashMap<String, String> = HashMap::new();
        content_map.insert("/".to_string(), "content".to_string());
        let content = get_content_for_path(path, content_map);

        assert_eq!(content, Some("content".to_string()));
    }

    #[test]
    fn test_get_content_for_path_custom_not_found() {
        let path = "/".to_string();
        let mut content_map: HashMap<String, String> = HashMap::new();
        content_map.insert("404".to_string(), "content".to_string());
        let content = get_content_for_path(path, content_map);

        assert_eq!(content, Some("content".to_string()));
    }

    #[test]
    fn test_get_content_for_path_default_not_found() {
        let path = "/".to_string();
        let content_map: HashMap<String, String> = HashMap::new();
        let content = get_content_for_path(path, content_map);

        assert_eq!(content, Some(NOT_FOUND.to_string()));
    }

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
