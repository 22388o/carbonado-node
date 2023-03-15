use std::fs;

use anyhow::Result;
use carbonado_node::{
    backend::fs::{delete_file, write_file},
    config::SYS_CFG,
    prelude::{CATALOG_DIR, SEGMENT_DIR},
    structs::Secp256k1PubKey,
};
use log::{debug, info};
use rand::thread_rng;
use secp256k1::generate_keypair;

const RUST_LOG: &str = "carbonad_node=trace,carbonado=trace,file=trace";

#[tokio::test]
async fn write_read() -> Result<()> {
    carbonado::utils::init_logging(RUST_LOG);

    let (_sk, pk) = generate_keypair(&mut thread_rng());

    info!("Reading file bytes");
    let file_bytes = fs::read("tests/samples/cat.gif")?;
    debug!("{} bytes read", file_bytes.len());

    info!("Writing file");
    let blake3_hash = write_file(Secp256k1PubKey(pk), &file_bytes).await?;
    debug!("File hash: {blake3_hash}");

    // info!("Reading file by hash");
    // let new_file_bytes = read_file(&blake3_hash).await?;
    // debug!("{} new bytes read", new_file_bytes.len());

    // assert_eq!(
    //     file_bytes, new_file_bytes,
    //     "Written and read file matches bytes"
    // );

    // info!("File write/read test finished successfully!");

    for volume in SYS_CFG.volumes.iter() {
        let segment_dir = volume.path.join(SEGMENT_DIR);
        let segment_count = fs::read_dir(segment_dir)?.count();
        assert_eq!(segment_count, 6);

        let catalog_dir = volume.path.join(CATALOG_DIR);
        let catalog_count = fs::read_dir(catalog_dir)?.count();
        assert_eq!(catalog_count, 1);
    }

    delete_file(&blake3_hash)?;

    for volume in SYS_CFG.volumes.iter() {
        let segment_dir = volume.path.join(SEGMENT_DIR);
        let segment_count = fs::read_dir(segment_dir)?.count();
        assert_eq!(segment_count, 0);

        let catalog_dir = volume.path.join(CATALOG_DIR);
        let catalog_count = fs::read_dir(catalog_dir)?.count();
        assert_eq!(catalog_count, 0);
    }

    Ok(())
}
