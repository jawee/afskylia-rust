use std::{collections::HashMap, net::{TcpListener, TcpStream}, io::{BufReader, BufRead, Write}};

pub fn start(content_map: &HashMap<String, Vec<u8>>) {
    let listener = TcpListener::bind("127.0.0.1:1313").unwrap();

    println!("Listening on port 1313");
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_stream(stream, &content_map);
    }
}

fn handle_stream(mut stream: TcpStream, content_map: &HashMap<String, Vec<u8>>) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    handle_connection(stream, request_line, &content_map);
}

fn handle_connection(mut stream: impl Write, request_line: String, content_map: &HashMap<String, Vec<u8>>) {
    println!("{request_line}");
    let mut path = get_request_path(&request_line);
    if path == "/" {
        path = "/index.html".to_string();
    }

    let (status_line, content) = match get_content_for_path(path.clone(), &content_map.clone()) {
        None => {
            ("HTTP/1.1 404 NOT FOUND", get_not_found_content(&content_map))
        },
        Some(t) => {
            ("HTTP/1.1 200 OK", t)
        }
    };

    let content_length = content.len();

    let content_type = match path {
        s if s.ends_with(".png") => "image/png",
        s if s.ends_with(".css") => "text/css",
        s if s.ends_with(".jpg") || s.ends_with(".jpeg") => "image/jpeg",
        s if s.ends_with(".js") => "text/javascript",
        _ => "text/html",
    };

    let response = 
        format!("{status_line}\r\ncontent-length: {content_length}\r\ncontent-type: {content_type}\r\n\r\n");

    let mut respvec = Vec::from(response.as_bytes());
    respvec.extend_from_slice(&mut Vec::from(content));

    stream.write_all(&respvec).unwrap();
}

fn get_not_found_content(content_map: &HashMap<String, Vec<u8>>) -> Vec<u8> {
    let content = match content_map.get("404") {
        Some(t) => t.to_vec(),
        None => NOT_FOUND.to_string().as_bytes().to_vec()
    };
    
    return content.to_vec();
}

fn get_content_for_path(path: String, content_map: &HashMap<String, Vec<u8>>) -> Option<Vec<u8>> {
    let maybe_content = content_map.get(&path).cloned();
    return maybe_content;
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

    use std::{collections::HashMap, io::{Error, Write}};

    use crate::server::server::{NOT_FOUND, get_not_found_content};

    use super::{get_request_path, get_request_path_string, get_content_for_path, handle_connection};

    struct MockWriter {
        content: Vec<u8>
    }
    
    impl MockWriter {
        pub fn get_content(&self) -> Vec<u8> {
            return self.content.clone();
        }
    }

    impl Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.content.extend_from_slice(buf);
            return Ok(buf.len());
        }

        fn flush(&mut self) -> std::io::Result<()> {
            todo!()
        }
    }
    fn get_respvec(status_line: &str, content: &str, content_type: &str) -> Vec<u8> {
        let content_length = content.len();
        let response = 
            format!("{status_line}\r\ncontent-length: {content_length}\r\ncontent-type: {content_type}\r\n\r\n");
        let mut respvec = Vec::from(response.as_bytes());
        respvec.extend_from_slice(&mut Vec::from(content));
        return respvec;
    }

    #[test]
    fn test_handle_connection_get_javascript() -> Result<(), Error> {
        let status_line = "HTTP/1.1 200 OK";
        let content = r#"console.log(hello)"#;
        let content_type = "text/javascript";
        let respvec = get_respvec(status_line, content, content_type);

        let mut stream = MockWriter{ content: Vec::new() };
        let request_line = "GET /script.js HTTP/1.1".to_string();
        let mut content_map: HashMap<String, Vec<u8>> = HashMap::new();
        content_map.insert("/script.js".to_string(), content.to_string().as_bytes().to_vec());
        handle_connection(&mut stream, request_line, &content_map);

        let content = stream.get_content();
        let resp_str = std::str::from_utf8(&content);
        let expected_str = std::str::from_utf8(&respvec);
        assert_eq!(resp_str, expected_str);
        assert_eq!(content.len(), respvec.len());
        return Ok(());
    }

    #[test]
    fn test_handle_connection() -> Result<(), Error> {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let content = NOT_FOUND;
        let content_type = "text/html";
        let respvec = get_respvec(status_line, content, content_type);

        let mut stream = MockWriter{ content: Vec::new() };
        let request_line = "".to_string();
        let content_map: HashMap<String, Vec<u8>> = HashMap::new();
        handle_connection(&mut stream, request_line, &content_map);

        assert_eq!(stream.get_content().len(), respvec.len());
        return Ok(());
    }

    #[test]
    fn test_get_content_for_path() {
        let path = "/".to_string();
        let mut content_map: HashMap<String, Vec<u8>> = HashMap::new();
        content_map.insert("/".to_string(), "content".as_bytes().to_vec());
        let content = get_content_for_path(path, &content_map);

        assert_eq!(content, Some("content".as_bytes().to_vec()));
    }

    #[test]
    fn test_get_content_for_path_not_found_none() {
        let path = "/asdf".to_string();
        let mut content_map: HashMap<String, Vec<u8>> = HashMap::new();
        content_map.insert("/".to_string(), "content".as_bytes().to_vec());
        let content = get_content_for_path(path, &content_map);

        assert_eq!(content, None);
    }

    #[test]
    fn test_get_not_found_custom_not_found() {
        let mut content_map: HashMap<String, Vec<u8>> = HashMap::new();
        content_map.insert("404".to_string(), "content".as_bytes().to_vec());
        let content = get_not_found_content(&content_map);

        assert_eq!(content, "content".as_bytes().to_vec());
    }

    #[test]
    fn test_get_content_for_path_default_not_found() {
        let content_map: HashMap<String, Vec<u8>> = HashMap::new();
        let content = get_not_found_content(&content_map);

        assert_eq!(content, NOT_FOUND.as_bytes().to_vec());
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
