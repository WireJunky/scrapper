use tokio::fs;
use std::path::PathBuf;
use std::io::{ErrorKind, Error};

static DATADIRNAME: &str = "Data";

async fn create_data_dir_if_not_exists() -> Result<PathBuf, std::io::Error> {
    let mut p = std::env::current_exe()?;
    p.pop();
    p.push(DATADIRNAME);
    let body_path = p.as_path();
    let attr = fs::metadata(body_path).await;
    match attr {
        Ok(_) => Ok(body_path.to_path_buf()),
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                fs::create_dir(body_path).await?
            }
            Err(e)
        }
    }
}

pub async fn write_file(file_name: &str, content: &str) -> std::io::Result<()> {
    let base_dir_res = create_data_dir_if_not_exists().await;

    if let Ok(mut base_dir) = base_dir_res {
        base_dir.push(file_name);
        fs::write(base_dir.as_path(), content).await
    } else {
        Err(Error::new(
            ErrorKind::Other,
            "error writing content to file",
        ))
    }
}

pub async fn read_file(file_name: &str) -> std::io::Result<String> {
    let base_dir_res = create_data_dir_if_not_exists().await;

    if let Ok(mut base_dir) = base_dir_res {
        base_dir.push(file_name);
        let contents = fs::read(base_dir.as_path()).await?;
        Ok(String::from_utf8(contents).unwrap())
    } else {
        Err(Error::new(
            ErrorKind::Other,
            "error writing content to file",
        ))
    }
}


