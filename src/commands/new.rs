use std::{path::Path, env::current_dir, fs, io::Error};

pub fn new(args: &Vec<String>) {
    let command = &args[2];
    // let command = args.iter().nth(2);
    match command.as_str() {
        "site" => new_site(),
        "page" => new_page(&args[3]),
        _ => println!("Unknown parameter '{}'", command.as_str()),
    }
}

fn new_site() {
    let dir = current_dir().unwrap();
    new_site_internal(dir.as_path()).unwrap();
}

fn new_site_internal(base_dir: &Path) -> Result<(), Error> {
    println!("create new site at {}", base_dir.display());
    
    let dirs = get_site_dirs();
    for dir in dirs {
        fs::create_dir(base_dir.join(dir).as_path())?;
    }

    return Ok(());
}

fn get_site_dirs() -> Vec<String> {
    let dirs = vec!["content", "layouts", "resources"];
    return dirs.iter().map(|s| s.to_string()).collect();
}

fn new_page(_page_name: &str) {
    println!("create page {}", _page_name);
}

#[cfg(test)]
mod tests {

    use std::{env::temp_dir, fs::{File, self}, io::Error};

    use uuid::Uuid;

    use super::{new_site_internal, get_site_dirs};

    #[test]
    fn test_new_site_internal() -> Result<(), Error> {
        let uuid = Uuid::new_v4().to_string();
        let dir = temp_dir().join(uuid);
        fs::create_dir(dir.as_path())?;

        new_site_internal(dir.as_path())?;

        for d in get_site_dirs() {
            let dir_meta = fs::metadata(dir.join(&d))?;
            assert_eq!(dir_meta.is_dir(), true, "{} is not a directory", d);
        }

        fs::remove_dir_all(dir.as_path())?;
        return Ok(());
    }

    #[test]
    fn test_tmp_dir() -> Result<(), Error> {
        let uuid = Uuid::new_v4().to_string();
        let dir = temp_dir().join(uuid);
        fs::create_dir(dir.as_path())?;

        let file_path = dir.as_path().join("tmpfile.txt");
        File::create(&file_path)?;
        let dir_path = dir.as_path().join("templates");
        fs::create_dir(&dir_path)?;

        let file_meta = fs::metadata(file_path)?;
        let dir_meta = fs::metadata(dir_path)?;
        assert_eq!(file_meta.is_file(), true);
        assert_eq!(file_meta.is_dir(), false);
        assert_eq!(dir_meta.is_dir(), true);
        assert_eq!(dir_meta.is_file(), false);
        fs::remove_dir_all(dir.as_path())?;
        return Ok(());
    }
}
