//! This crate lets you write bytes directly to disk, to flash to your
#![warn(rust_2018_idioms)]
#![cfg_attr(docsrs, feature(doc_cfg))]

use error::Result;
use std::path::Path;
use tokio::io::AsyncWriteExt;

pub mod error;

#[tokio::main]
async fn main() -> Result<()> {
    let current = std::env::current_dir().expect("Current path");
    let output_path = current.join(Path::new("rom.bin"));

    let mut rom: Vec<u8> = vec![
        0xa9, 0xff, // lda ff
        0x8d, 0x02, 0x60, // sta 6002
        //
        0xa9, 0x55, // lda 55
        0x8d, 0x00, 0x60, // sta 6000
        //
        0xa9, 0xaa, // lda aa
        0x8d, 0x02, 0x60, // sta 6002
        //
        0x4c, 0x05, 0x80, // jmp 8005
    ];

    let remaining = 32768 - rom.len();
    for _ in 0..remaining {
        rom.push(0xea);
    }

    rom[0x7ffc] = 0x00;
    rom[0x7ffd] = 0x80; // little endian for 6502

    save_bytes_to_disk(output_path, rom).await?;

    Ok(())
}

async fn save_bytes_to_disk<P, D>(path: P, data: D) -> Result<()>
where
    P: AsRef<std::path::Path>,
    D: AsRef<[u8]>,
{
    let mut file = tokio::fs::File::create(path).await?;
    file.write_all(data.as_ref()).await?;

    Ok(())
}
