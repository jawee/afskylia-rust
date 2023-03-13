use std::{path::Path, env::current_dir};

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
    new_site_internal(dir.as_path());
}

fn new_site_internal(_dir: &Path) {
    println!("create new site at {}", _dir.display());
}

fn new_page(_page_name: &str) {
    println!("create page {}", _page_name);
}

#[cfg(test)]
mod tests {

    use std::{env::temp_dir, fs::{File, self}, io::Error};

    use super::new_site_internal;

    #[test]
    fn test_new_site_internal() -> Result<(), Error> {
        let dir = temp_dir().join("test_tmp_dir2");
        println!("{}", dir.display());
        fs::create_dir(dir.as_path())?;

        new_site_internal(dir.as_path());
        fs::remove_dir_all(dir.as_path())?;
        return Ok(());
    }

    #[test]
    fn test_tmp_dir() -> Result<(), Error> {
        // let uuid = random
        let dir = temp_dir().join("test_tmp_dir");
        println!("{}", dir.display());
        fs::create_dir(dir.as_path())?;
        let file_path = dir.as_path().join("tmpfile.txt");
        let _file = File::create(&file_path)?;
        let dir_path = dir.as_path().join("templates");
        let _newdir = fs::create_dir(&dir_path)?;

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
