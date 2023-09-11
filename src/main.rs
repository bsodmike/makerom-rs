use error::Error;
use std::{fs::File, path::Path};
use tokio::io::AsyncWriteExt;

pub mod error;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let bytes: Vec<u8> = vec![];

    let current = std::env::current_dir().expect("Current path");
    let output_path = current.join(Path::new("output"));

    Ok(())
}

async fn save_bytes_to_disk<P, D>(path: P, data: D) -> Result<(), Error>
where
    P: AsRef<std::path::Path>,
    D: AsRef<[u8]>,
{
    let mut file = tokio::fs::File::create(path).await?;
    file.write_all(data.as_ref()).await?;

    Ok(())
}
