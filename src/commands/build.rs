use std::{collections::HashMap, env::current_dir, fs::{self, File}, io::{Read, BufReader}, path::{PathBuf, Path}};

pub fn build(_args: &Vec<String>) {
    let _curr_dir_path = current_dir().unwrap();
    todo!("Build is not implemented yet.");
}

fn build_internal(base_dir: PathBuf) {
    let layouts_map = get_layouts(&base_dir);
}

static LAYOUT_DIR_PATH: &str = "layouts";

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
        println!("Name: {}", path.display())
    }

    return map;
}

fn get_relative_file_path(file_path: &PathBuf, base_path: &PathBuf) -> PathBuf {
    let file_path = file_path.strip_prefix(base_path).expect("ERROR: Couldn't strip prefix");
    return file_path.to_path_buf();
}



#[cfg(test)]
mod tests {
    use std::{path::{Path, PathBuf}, env::temp_dir, fs::{self, File}, io::{Write, BufWriter}};

    use claim::{assert_ok, assert_some};
    use uuid::Uuid;

    use crate::commands::build::{get_relative_file_path, get_layouts};

    #[test]
    fn test_get_layouts_index() {
        let base_dir = create_test_site();
        let layout_map = get_layouts(&base_dir);

        // println!("{:?}", layout_map);
        assert_ok!(fs::remove_dir_all(base_dir.as_path()));
        assert_some!(layout_map.get("index.html"));
    }

    #[test]
    fn test_build() {
        let base_dir = create_test_site();
        assert_ok!(fs::remove_dir_all(base_dir.as_path()));
    }
    fn create_test_site() -> PathBuf{
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

        let index_content_file = File::create(base_dir.as_path().join("content").as_path().join("index.md").as_path()).expect("ERROR: couldn't create index content file");
        buf_writer = BufWriter::new(index_content_file);
        buf_writer.write(INDEX_CONTENT.as_ref()).expect("ERROR: couldn't write to content file");

        return base_dir;
    }


    #[test]
    fn test_strip_base_path_with_subdirectory() {
        let base_path = PathBuf::from("/home/user/website/");
        let file_path = PathBuf::from("/home/user/website/subdir/index.txt");

        let res = get_relative_file_path(&file_path, &base_path);

        assert_eq!(res.as_path(), Path::new("subdir/index.txt"));
        // assert_eq!(path.strip_prefix("/test/haha/foo.txt"), Ok(Path::new("")));
        // assert_eq!(path.strip_prefix("/test/haha/foo.txt/"), Ok(Path::new("")));
    }
    #[test]
    fn test_strip_base_path() {
        let base_path = PathBuf::from("/home/user/website/");
        let file_path = PathBuf::from("/home/user/website/index.txt");

        let res = get_relative_file_path(&file_path, &base_path);

        assert_eq!(res.as_path(), Path::new("index.txt"));
        // assert_eq!(path.strip_prefix("/test/haha/foo.txt"), Ok(Path::new("")));
        // assert_eq!(path.strip_prefix("/test/haha/foo.txt/"), Ok(Path::new("")));
    }



    static INDEX_CONTENT: &str = r#"
        # Index

        Some content
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
