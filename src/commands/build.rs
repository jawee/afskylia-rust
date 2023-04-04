use std::{collections::HashMap, env::current_dir, fs::{self, File}, io::{Read, BufReader}, path::PathBuf};

pub fn build(_args: &Vec<String>) {
    let layouts_map = get_layouts();
    todo!("Build is not implemented yet.");
}

static LAYOUT_DIR_PATH: &str = "layouts";

fn get_layouts() -> HashMap<String, String> {
    let mut map = HashMap::new();
    let curr_dir_path = current_dir().unwrap();
    let layouts_dir_path = curr_dir_path.join(LAYOUT_DIR_PATH);

    let paths = fs::read_dir(layouts_dir_path).unwrap();

    for path in paths {
        let path = path.unwrap().path();
        let file = File::open(&path).expect("ERROR: Couldn't open file");
        let mut buf_reader = BufReader::new(file);
                                                                                                                                                     
        let mut content = String::new();
        buf_reader.read_to_string(&mut content).expect("ERROR: Couldn't read file to string");

        // map.insert(path.file_name().map_, content);
        println!("Name: {}", path.display())
    }

    return map;
}

fn get_relative_file_path(file_path: PathBuf, base_path: PathBuf) -> PathBuf {
    let file_path = file_path.strip_prefix(base_path).expect("ERROR: Couldn't strip prefix");
    return file_path.to_path_buf();
}



#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use crate::commands::build::get_relative_file_path;


    #[test]
    fn test_strip_base_path_with_subdirectory() {
        let base_path = PathBuf::from("/home/user/website/");
        let file_path = PathBuf::from("/home/user/website/subdir/index.txt");

        let res = get_relative_file_path(file_path, base_path);

        assert_eq!(res.as_path(), Path::new("subdir/index.txt"));
        // assert_eq!(path.strip_prefix("/test/haha/foo.txt"), Ok(Path::new("")));
        // assert_eq!(path.strip_prefix("/test/haha/foo.txt/"), Ok(Path::new("")));
    }
    #[test]
    fn test_strip_base_path() {
        let base_path = PathBuf::from("/home/user/website/");
        let file_path = PathBuf::from("/home/user/website/index.txt");

        let res = get_relative_file_path(file_path, base_path);

        assert_eq!(res.as_path(), Path::new("index.txt"));
        // assert_eq!(path.strip_prefix("/test/haha/foo.txt"), Ok(Path::new("")));
        // assert_eq!(path.strip_prefix("/test/haha/foo.txt/"), Ok(Path::new("")));
    }

}
