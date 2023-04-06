use std::{path::Path, env::current_dir, fs::{self, File}, io::{Error, BufWriter, Write}};

pub fn new(args: &Vec<String>) {
    let command = &args[2];
    // let command = args.iter().nth(2);
    match command.as_str() {
        "help" => println!("{}", HELP),
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

    for dir in get_site_dirs() {
        fs::create_dir(base_dir.join(dir).as_path())?;
    }

    let base_layout_file = File::create(base_dir.join("layouts").join("_base.html")).expect("ERROR: couldn't create base layout file");
    let mut buf_writer = BufWriter::new(base_layout_file);
    buf_writer.write(BASE.as_ref()).expect("ERROR: couldn't write to layout file");


    let index_layout_file = File::create(base_dir.join("layouts").join("index.html")).expect("ERROR: couldn't create index layout file");
    buf_writer = BufWriter::new(index_layout_file);
    buf_writer.write(INDEX_LAYOUT.as_ref()).expect("ERROR: couldn't write to layout file");


    let index_content_file = File::create(base_dir.join("content").join("index.md")).expect("ERROR: couldn't create index content file");
    buf_writer = BufWriter::new(index_content_file);
    buf_writer.write(INDEX_CONTENT.as_ref()).expect("ERROR: couldn't write to content file");
    return Ok(());
}

fn get_site_dirs() -> Vec<String> {
    let dirs = vec!["content", "layouts", "resources"];
    return dirs.iter().map(|s| s.to_string()).collect();
}

fn new_page(_page_name: &str) {
    println!("create page {}", _page_name);
}

static BASE: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>My Page</title>
</head>
<body>
{layout}
</body>
</html>
"#;
static INDEX_CONTENT: &str = r#"
# Index

Some content

1. First list item
2. Second list item


"#;

static INDEX_LAYOUT: &str = r#"
{content}
"#;

static HELP: &str = r#"
Create a new site or create a new content file.

Usage:
afskylia new [command]

Available Commands:
site        Create a new site 
page        Create a new page

"#;

#[cfg(test)]
mod tests {

    use std::{env::temp_dir, fs::{File, self}, io::Error};

    use claim::assert_ok;
    use uuid::Uuid;

    use super::{new_site_internal, get_site_dirs};

    #[test]
    fn test_new_site_internal() -> Result<(), Error> {
        let uuid = Uuid::new_v4().to_string();
        let base_dir = temp_dir().join("rust").join(uuid);
        fs::create_dir_all(base_dir.as_path()).expect("ERROR: couldn't create base_dir");

        new_site_internal(base_dir.as_path())?;

        for d in get_site_dirs() {
            let dir_meta = fs::metadata(base_dir.join(&d))?;
            assert_eq!(dir_meta.is_dir(), true, "{} is not a directory", d);
        }

        let mut base_metadata = fs::metadata(base_dir.join("layouts/_base.html"));
        assert_ok!(base_metadata);
        base_metadata = fs::metadata(base_dir.join("layouts/index.html"));
        assert_ok!(base_metadata);
        base_metadata = fs::metadata(base_dir.join("content/index.md"));
        assert_ok!(base_metadata);
        assert!(std::panic::catch_unwind(|| {}).is_ok());

        fs::remove_dir_all(base_dir.as_path())?;
        return Ok(());
    }

    #[test]
    fn test_tmp_dir() -> Result<(), Error> {
        let uuid = Uuid::new_v4().to_string();
        let base_dir = temp_dir().join("rust").join(uuid);
        fs::create_dir_all(base_dir.as_path()).expect("ERROR: couldn't create base_dir");

        let file_path = base_dir.as_path().join("tmpfile.txt");
        File::create(&file_path)?;
        let dir_path = base_dir.as_path().join("templates");
        fs::create_dir(&dir_path)?;

        let file_meta = fs::metadata(file_path)?;
        let dir_meta = fs::metadata(dir_path)?;
        assert_eq!(file_meta.is_file(), true);
        assert_eq!(file_meta.is_dir(), false);
        assert_eq!(dir_meta.is_dir(), true);
        assert_eq!(dir_meta.is_file(), false);
        fs::remove_dir_all(base_dir.as_path())?;
        return Ok(());
    }
}
