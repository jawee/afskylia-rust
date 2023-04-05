use std::collections::HashMap;
use std::env::current_dir;
use std::fs::{self, File};
use std::io::{Read, BufReader, BufWriter, Write};
use std::path::PathBuf;

use crate::generators::{MergePage, HtmlGenerator};
use crate::parsers::markdown::Lexer;

pub fn build(_args: &Vec<String>) {
    let curr_dir_path = current_dir().unwrap();
    build_internal(&curr_dir_path);
}

fn build_internal(base_dir: &PathBuf) {
    let layouts_map = get_layouts(&base_dir);
    let content_map = get_content(&base_dir);

    let public_dir_path = base_dir.join(PUBLIC_DIR_PATH);

    if !public_dir_path.is_dir() {
        fs::create_dir(&public_dir_path).expect("ERROR: Couldn't create public dir");
    }

    for (key, value) in layouts_map.iter() {
        let file_name = &key[..=key.len()-6];
        let content_key = &format!("{file_name}.md");
        let markdown_content = content_map.get(content_key).expect(&format!("ERROR: couldn't get content for {content_key}"));

        let lexer = Lexer::new(markdown_content).unwrap();
        let mut html_generator = HtmlGenerator::new(lexer);
        let html_content = html_generator.get_html().unwrap();

        let page = MergePage::parse(value, &html_content).expect("ERROR: Couldn't merge page");
        let public_file = File::create(&public_dir_path.join(key)).expect(&format!("ERROR: Couldn't create page {key}"));
        let mut buf_writer = BufWriter::new(public_file);
        buf_writer.write(page.to_string().as_ref()).expect("ERROR: couldn't write content to file");
    }
}

static PUBLIC_DIR_PATH: &str = "public";
static LAYOUT_DIR_PATH: &str = "layouts";
static CONTENT_DIR_PATH: &str = "content";

fn get_content(base_dir: &PathBuf) -> HashMap<String, String> {
    let content_dir_path = base_dir.join(CONTENT_DIR_PATH);

    let map = get_content_rec(&content_dir_path, &content_dir_path);
    return map;
}

fn get_content_rec(dir: &PathBuf, base_dir: &PathBuf) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for path in fs::read_dir(&dir).expect("ERROR: Couldn't read content_dir_path") {
        let path = path.unwrap().path();
        if path.is_dir(){
            let new_map = get_content_rec(&path.to_path_buf(), base_dir);
            map.extend(new_map);
            continue;
        }

        let file = File::open(&path).expect("ERROR: Couldn't open file");
        let mut buf_reader = BufReader::new(file);
                                                                                                                                                     
        let mut content = String::new();
        buf_reader.read_to_string(&mut content).expect("ERROR: Couldn't read file to string");

        let relative_file_path = get_relative_file_path(&path, &base_dir);
        map.insert(relative_file_path.into_os_string().into_string().expect("ERROR: Couldn't convert path to string"), content);
    }
    return map.clone();
}

fn get_layouts(base_dir: &PathBuf) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let layouts_dir_path = base_dir.join(LAYOUT_DIR_PATH);

    let paths = fs::read_dir(&layouts_dir_path).unwrap();

    for path in paths {
        let path = path.unwrap().path();
        let file = File::open(&path).expect("ERROR: Couldn't open file");
        let mut buf_reader = BufReader::new(file);
                                                                                                                                                     
        let mut content = String::new();
        buf_reader.read_to_string(&mut content).expect("ERROR: Couldn't read file to string");

        let relative_file_path = get_relative_file_path(&path, &layouts_dir_path);
        map.insert(relative_file_path.into_os_string().into_string().expect("ERROR: Couldn't convert path to string"), content);
    }

    return map;
}

fn get_relative_file_path(file_path: &PathBuf, base_path: &PathBuf) -> PathBuf {
    let file_path = file_path.strip_prefix(base_path).expect("ERROR: Couldn't strip prefix");
    return file_path.to_path_buf();
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};
    use std::env::temp_dir;
    use std::fs::{self, File};
    use std::io::{Write, BufWriter};

    use claim::{assert_ok, assert_some};
    use uuid::Uuid;

    use crate::commands::build::*;

    #[test]
    fn test_build_internal() {
        let base_dir_path = create_test_site();
        println!("{}", base_dir_path.clone().into_os_string().into_string().unwrap());
        build_internal(&base_dir_path);

        let public_dir_path = base_dir_path.join(PUBLIC_DIR_PATH);
        assert_ok!(fs::read_to_string(public_dir_path.join("index.html")));
        assert_ok!(fs::remove_dir_all(base_dir_path.as_path()));
    }

    #[test]
    fn test_get_content() {
        let base_dir = create_test_site();
        let content_map = get_content(&base_dir);

        assert_ok!(fs::remove_dir_all(base_dir.as_path()));
        assert_some!(content_map.get("index.md"), "Couldn't get index.md");
        assert_some!(content_map.get("posts/post-1.md"), "Couldn't get posts/post-1.md");
        assert_some!(content_map.get("posts/post-2.md"), "Couldn't get posts/post-2.md");
    }
    #[test]
    fn test_get_layouts() {
        let base_dir = create_test_site();
        let layout_map = get_layouts(&base_dir);

        assert_ok!(fs::remove_dir_all(base_dir.as_path()));
        assert_some!(layout_map.get("index.html"));
        assert_some!(layout_map.get("posts.html"));
    }

    #[test]
    fn test_build() {
        let base_dir = create_test_site();
        assert_ok!(fs::remove_dir_all(base_dir.as_path()));
    }

    fn create_test_site() -> PathBuf {
        let uuid = Uuid::new_v4().to_string();
        let base_dir = temp_dir().join(uuid);
        fs::create_dir(base_dir.as_path()).unwrap();

        let dirs = vec!["content", "layouts", "resources"];
        for dir in dirs {
            fs::create_dir(base_dir.join(dir).as_path()).unwrap();
        }

        let index_layout_file = File::create(base_dir.as_path().join("layouts").as_path().join("index.html").as_path()).expect("ERROR: couldn't create index layout file");
        let mut buf_writer = BufWriter::new(index_layout_file);
        buf_writer.write(INDEX_LAYOUT.as_ref()).expect("ERROR: couldn't write to layout file");

        let posts_layout_file = File::create(base_dir.as_path().join("layouts/posts.html").as_path()).expect("ERROR: couldn't create posts layout file");
        buf_writer = BufWriter::new(posts_layout_file);
        buf_writer.write(INDEX_LAYOUT.as_ref()).expect("ERROR: couldn't write to posts file");

        let index_content_file = File::create(base_dir.as_path().join("content").as_path().join("index.md").as_path()).expect("ERROR: couldn't create index content file");
        buf_writer = BufWriter::new(index_content_file);
        buf_writer.write(INDEX_CONTENT.as_ref()).expect("ERROR: couldn't write to content file");

        fs::create_dir(base_dir.join("content").join("posts")).expect("ERROR: Couldn't create posts dir in contents");

        let posts_file_1 = File::create(base_dir.join("content").join("posts").join("post-1.md")).expect("Error: couldn't create post-1.md");
        buf_writer = BufWriter::new(posts_file_1);
        buf_writer.write(POST_1_CONTENT.as_ref()).expect("ERROR: couldn't write to content file");

        let posts_file_2 = File::create(base_dir.join("content").join("posts").join("post-2.md")).expect("Error: couldn't create post-2.md");
        buf_writer = BufWriter::new(posts_file_2);
        buf_writer.write(POST_2_CONTENT.as_ref()).expect("ERROR: couldn't write to content file");

        return base_dir;
    }


    #[test]
    fn test_strip_base_path_with_subdirectory() {
        let base_path = PathBuf::from("/home/user/website/");
        let file_path = PathBuf::from("/home/user/website/subdir/index.txt");

        let res = get_relative_file_path(&file_path, &base_path);

        assert_eq!(res.as_path(), Path::new("subdir/index.txt"));
    }
    #[test]
    fn test_strip_base_path() {
        let base_path = PathBuf::from("/home/user/website/");
        let file_path = PathBuf::from("/home/user/website/index.txt");

        let res = get_relative_file_path(&file_path, &base_path);

        assert_eq!(res.as_path(), Path::new("index.txt"));
    }



    static INDEX_CONTENT: &str = r#"
        # Index

        Some content
        "#;
    static POST_1_CONTENT: &str = r#"
        # Post 1

        Post content
        "#;
    static POST_2_CONTENT: &str = r#"
        # Post 2

        Post content
        "#;
    static INDEX_LAYOUT: &str = r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
        <meta charset="utf-8">
        <title>404 - Not Found</title>
        </head>
        <body>
        {content}
        </body>
        </html>
        "#;

}
