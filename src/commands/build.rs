use std::collections::HashMap;
use std::env::current_dir;
use std::fs::{self, File};
use std::io::{Read, BufReader, BufWriter, Write};
use std::path::{PathBuf, Path};

use crate::generators::{MergePage, HtmlGenerator};
use crate::parsers::markdown::Lexer;

pub fn build(_args: &Vec<String>) {
    let curr_dir_path = current_dir().unwrap();
    build_internal(&curr_dir_path);
}

fn build_internal(base_dir: &PathBuf) -> HashMap<PathBuf, String> {
    let mut layouts_map = get_layouts(&base_dir);
    let content_map = get_content(&base_dir);

    let public_dir_path = base_dir.join(PUBLIC_DIR_PATH);

    if !public_dir_path.is_dir() {
        fs::create_dir(&public_dir_path).expect("ERROR: Couldn't create public dir");
    }

    copy_resources_to_public(&base_dir.join("resources"), &public_dir_path);

    let maybe_base_template = layouts_map.remove("_base.html");
    let keys = layouts_map.clone().into_keys().collect::<Vec<String>>();
    let menu_html = build_menu_html(keys);

    for (key, value) in layouts_map.iter() {
        let file_name = &key[..=key.len()-6];
        let content_key = &format!("{file_name}.md");
        let markdown_content = match content_map.get(content_key) {
            Some(c) => c.to_string(),
            None => {
                //TODO: need to handle this better. But for now just merge all posts into a
                //long markdown string and generate html
                let keys = content_map.keys().filter(|x| x.starts_with(&format!("{file_name}/"))).collect::<Vec<&String>>();
                let mut str = String::new();
                for key in keys {
                    let cont = match content_map.get(key) {
                        Some(c) => c,
                        None => ""
                    };

                    str.push_str(cont);
                    str.push_str("\n\n");
                }
                str
            }
        };

        let lexer = Lexer::new(&markdown_content).unwrap();
        let mut html_generator = HtmlGenerator::new(lexer);
        let html_content = html_generator.get_html().unwrap();

        let mut layout = merge_base_with_layout(&maybe_base_template, &value);
        layout = layout.replace("{menu}", &menu_html);
        let page = MergePage::parse(&layout, &html_content).expect("ERROR: Couldn't merge page");
        let public_file = File::create(&public_dir_path.join(key)).expect(&format!("ERROR: Couldn't create page {key}"));
        let mut buf_writer = BufWriter::new(public_file);
        buf_writer.write(page.to_string().as_ref()).expect("ERROR: couldn't write content to file");
    }

    return HashMap::new();
}

fn copy_resources_to_public(resources_dir: &PathBuf, public_dir: &PathBuf) {
    if !Path::new(public_dir).exists() {
        fs::create_dir_all(public_dir).expect("ERROR: Couldn't create public dir");
    }
    for entry in fs::read_dir(resources_dir).expect("ERROR: Couldn't read resources dir") {
        match entry {
            Ok(entry) => {
                let orig_path = entry.path();
                let new_path = public_dir.join(&orig_path.file_name().expect("ERROR: Couldn't get file_name"));
                fs::copy(orig_path, new_path).expect("ERROR: couldn't copy file");
            },
            Err(e) => {
                eprintln!("ERROR: Couldn't read resource: {e}")
            }
        }
    }
}

fn merge_base_with_layout(base_tpl: &Option<String>, content_layout: &String) -> String {
    let layout = match base_tpl {
        Some(base) => {
            let res = base.replace("{layout}", content_layout);
            res.to_string()
        },
        None => {
            content_layout.to_string()
        }
    };

    return layout;
}

static PUBLIC_DIR_PATH: &str = "public";
static LAYOUT_DIR_PATH: &str = "layouts";
static CONTENT_DIR_PATH: &str = "content";

fn get_content(base_dir: &PathBuf) -> HashMap<String, String> {
    let content_dir_path = base_dir.join(CONTENT_DIR_PATH);

    let map = get_content_rec(&content_dir_path, &content_dir_path);
    return map;
}

//TODO: Should probably have a struct instead of string, to support ordering and custom naming
fn build_menu_html(pages: Vec<String>) -> String {
    let mut menu_html = String::from("<ul>");
    for page in pages {
        let page_name = page.replace(".html", "");
        let menu_item = format!("<li><a href=\"/{page}\">{page_name}</a></li>");
        menu_html.push_str(&menu_item);
    }
    menu_html.push_str("</ul>");
    return menu_html;
}

fn get_content_rec(dir: &PathBuf, base_dir: &PathBuf) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for path in fs::read_dir(&dir).expect("ERROR: Couldn't read content_dir_path") {
        let path = path.expect("ERROR: couldn't get path").path();
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

    use claim::{assert_ok, assert_some};

    use crate::commands::test_utils::*;
    use crate::commands::build::*;

    #[test]
    fn test_copy_resources() {
        let site_builder = SiteBuilder::new()
            .with_resource("style", "css", "")
            .with_resource("script", "js", "");

        let base_dir_path = site_builder.get_path();

        let public_dir_path = base_dir_path.join("public");

        copy_resources_to_public(&base_dir_path.join("resources"), &public_dir_path);

        assert_eq!(Path::new(&public_dir_path.join("style.css")).exists(), true, "style.css doesn't exist");
        assert_eq!(Path::new(&public_dir_path.join("script.js")).exists(), true, "script.js doesn't exist");

        assert_ok!(fs::remove_dir_all(base_dir_path.as_path()));
    }

    #[test]
    fn test_build_menu() {
        let menu_items: Vec<String> = vec!["index.html", "second-page.html", "posts.html"].iter().map(|x| x.to_string()).collect();
        let expected = "<ul><li><a href=\"/index.html\">index</a></li><li><a href=\"/second-page.html\">second-page</a></li><li><a href=\"/posts.html\">posts</a></li></ul>";

        let result = build_menu_html(menu_items);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_build_internal() {
        let site_builder = SiteBuilder::new()
            .with_base_layout("base", BASE)
            .with_page_with_content("index", INDEX_LAYOUT, INDEX_CONTENT);
        let base_dir_path = site_builder.get_path();

        build_internal(&base_dir_path);

        let public_dir_path = base_dir_path.join(PUBLIC_DIR_PATH);
        let index_file_str = fs::read_to_string(public_dir_path.join("index.html")).expect("ERROR: Couldn't read index.html");

        assert_ok!(fs::remove_dir_all(base_dir_path.as_path()));
        assert_eq!(index_file_str.contains("<h1"), true, "Does not contain h1");
        assert_eq!(index_file_str.contains("<html"), true, "Does not contain html-tag");
        assert_eq!(index_file_str.contains("<head>"), true, "Does not contain head-tag");
        assert_eq!(index_file_str.contains("<body"), true, "Does not contain body-tag");
    }

    #[test]
    fn test_get_content_nested() {
        let site_builder = SiteBuilder::new()
            .with_base_layout("base", BASE)
            .with_page_with_content("index", INDEX_LAYOUT, INDEX_CONTENT)
            .with_page_with_nested_content("posts", INDEX_LAYOUT, PathBuf::from("posts"), "post-1", POST_1_CONTENT)
            .with_nested_content(PathBuf::from("posts"), "post-2", POST_2_CONTENT);
        let base_dir_path = site_builder.get_path();

        let content_map = get_content(&base_dir_path);

        assert_ok!(fs::remove_dir_all(base_dir_path.as_path()));
        assert_some!(content_map.get("index.md"), "Couldn't get index.md");
        assert_some!(content_map.get("posts/post-1.md"), "Couldn't get posts/post-1.md");
        assert_some!(content_map.get("posts/post-2.md"), "Couldn't get posts/post-2.md");
    }

    #[test]
    fn test_get_content() {
        let site_builder = SiteBuilder::new()
            .with_base_layout("base", BASE)
            .with_page_with_content("index", INDEX_LAYOUT, INDEX_CONTENT);
        let base_dir_path = site_builder.get_path();

        let content_map = get_content(&base_dir_path);

        assert_ok!(fs::remove_dir_all(base_dir_path.as_path()));
        assert_some!(content_map.get("index.md"), "Couldn't get index.md");
    }

    #[test]
    fn test_get_layouts() {
        let site_builder = SiteBuilder::new()
            .with_base_layout("base", BASE)
            .with_page_with_content("index", INDEX_LAYOUT, INDEX_CONTENT)
            .with_page_with_nested_content("posts", INDEX_LAYOUT, PathBuf::from("posts"), "post-1", POST_1_CONTENT);

        let base_dir_path = site_builder.get_path();
        let layout_map = get_layouts(&base_dir_path);

        assert_ok!(fs::remove_dir_all(base_dir_path.as_path()));
        assert_some!(layout_map.get("index.html"));
        assert_some!(layout_map.get("posts.html"));
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
}

#[cfg(test)]
pub mod test_utils {
    use std::io::{BufWriter, Write};
    use std::fs::{self, File};
    use std::env::temp_dir;
    use std::path::PathBuf;

    use uuid::Uuid;
    pub struct SiteBuilder {
        base_dir_path: PathBuf
    }

    impl SiteBuilder {
        pub fn new() -> Self {
            let uuid = Uuid::new_v4().to_string();
            let base_dir_path = temp_dir().join("rust").join(uuid);
            fs::create_dir_all(base_dir_path.as_path()).expect("ERROR: couldn't create base_dir");
            let dirs = vec!["content", "layouts", "resources"];
            for dir in dirs {
                fs::create_dir_all(base_dir_path.join(dir)).expect("ERROR: Couldn't create dir");
            }
            return SiteBuilder {base_dir_path};
        }

        pub fn get_path(self) -> PathBuf {
            return self.base_dir_path.clone();
        }

        pub fn with_base_layout(self, layout_name: &str, layout_content: &str) -> Self {
            let layout_file_name = format!("_{layout_name}.html");
            let base_layout_file = File::create(self.base_dir_path.join("layouts").join(layout_file_name)).expect("ERROR: couldn't create base layout file");
            let mut buf_writer = BufWriter::new(base_layout_file);
            buf_writer.write(layout_content.as_ref()).expect("ERROR: couldn't write to content file");

            return self;
        }

        pub fn with_resource(self, resource_name: &str, resource_file_extension: &str,  resource_content: &str) -> Self {
            let resource_file_name = format!("{}.{}", resource_name, resource_file_extension);
            let resource_file = File::create(self.base_dir_path.join("resources").join(resource_file_name)).expect("ERROR: couldn't create resources file");
            let mut buf_writer = BufWriter::new(resource_file);
            buf_writer.write(resource_content.as_ref()).expect("ERROR: couldn't write to resource file");
            return self;
        }

        pub fn with_page_with_content(self, page_name: &str, page_layout: &str, page_content: &str) -> Self {
            //layout
            let layout_file_name = format!("{}.html", page_name);
            let layout_file = File::create(self.base_dir_path.join("layouts").join(layout_file_name)).expect("ERROR: couldn't create layout file");
            let mut buf_writer = BufWriter::new(layout_file);
            buf_writer.write(page_layout.as_ref()).expect("ERROR: couldn't write to layout file");

            //content
            let content_file_name = format!("{}.md", page_name);
            let content_file = File::create(self.base_dir_path.join("content").join(content_file_name)).expect("Error: couldn't content file");
            buf_writer = BufWriter::new(content_file);
            buf_writer.write(page_content.as_ref()).expect("ERROR: couldn't write to content file");
            return self;
        }

        pub fn with_page_with_nested_content(self, page_name: &str, page_layout: &str, page_content_path: PathBuf, page_content_name: &str, page_content: &str) -> Self {
            fs::create_dir_all(self.base_dir_path.join("content").join(&page_content_path)).expect("ERROR: Couldn't create page_content_path");

            //layout
            let layout_file_name = format!("{}.html", page_name);
            let layout_file = File::create(self.base_dir_path.join("layouts").join(layout_file_name)).expect("ERROR: couldn't create layout file");
            let mut buf_writer = BufWriter::new(layout_file);
            buf_writer.write(page_layout.as_ref()).expect("ERROR: couldn't write to layout file");

            //content
            let content_file_name = format!("{}.md", page_content_name);
            let content_file = File::create(self.base_dir_path.join("content").join(page_content_path).join(content_file_name)).expect("Error: couldn't create content file");
            buf_writer = BufWriter::new(content_file);
            buf_writer.write(page_content.as_ref()).expect("ERROR: couldn't write to content file");
            return self;
        }

        pub fn with_nested_content(self, content_path: PathBuf, content_name: &str, content: &str) -> Self {
            let content_file_name = format!("{}.md", content_name);
            let content_file = File::create(self.base_dir_path.join("content").join(&content_path).join(content_file_name)).expect("Error: couldn't create content file");
            let mut buf_writer = BufWriter::new(content_file);
            buf_writer.write(content.as_ref()).expect("ERROR: couldn't write to content file");
            return self;
        }
    }

    pub static INDEX_CONTENT: &str = "\
        # Index\n\
        \n\
        Some content\n\
        ";
    pub static POST_1_CONTENT: &str = "\
        # Post 1\n\
        \n\
        Post content\n\
        ";
    pub static POST_2_CONTENT: &str = "\
        # Post 2\n\
        \n\
        Post content\n\
        ";
    pub static BASE: &str = r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
        <meta charset="utf-8">
        <title>404 - Not Found</title>
        </head>
        <body>
        {menu}
        {layout}
        </body>
        </html>
        "#;
    pub static INDEX_LAYOUT: &str = r#"
        {content}
        "#;
}
