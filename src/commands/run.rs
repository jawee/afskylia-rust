use std::{collections::HashMap, env::current_dir, path::PathBuf, fs::{self, File}, io::{BufReader, Read}};

use crate::server;
use crate::commands::build;

pub fn run(args: &Vec<String>) {
    build(args);
    let curr_dir_path = current_dir().unwrap();

    let public_folder_path = curr_dir_path.join(PUBLIC_DIR_PATH);

    let map = get_folder_contents(public_folder_path);

    server::start(&map);
}

static PUBLIC_DIR_PATH: &str = "public";

fn get_folder_contents(public_folder_path: PathBuf) -> HashMap<String, Vec<u8>> {
    let mut map = HashMap::new();
    let dir = fs::read_dir(&public_folder_path).expect("ERROR: Can't read directory");
    for path in dir {
        let path = path.expect("ERROR: Something went wrong with path").path();

        let file = File::open(&path).expect("ERROR: Couldn't open file");
        let mut buf_reader = BufReader::new(file);
                                                                                                                                                     
        let mut content = String::new();
        buf_reader.read_to_string(&mut content).expect("ERROR: Couldn't read file to string");

        let relative_file_path = get_relative_file_path(&path, &public_folder_path).into_os_string().into_string().expect("ERROR: COuldn't convert path to string");
        let relative_file_path_string = format!("/{}", relative_file_path);
        map.insert(relative_file_path_string, content.as_bytes().to_vec());
    }
    return map;
}

fn get_relative_file_path(file_path: &PathBuf, base_path: &PathBuf) -> PathBuf {
    let file_path = file_path.strip_prefix(base_path).expect("ERROR: Couldn't strip prefix");
    return file_path.to_path_buf();
}
