use std::io::{Read, Seek};

use crate::vercel::constants::FASTN_VERCEL_REMOTE_BUILD_HASH;

pub async fn upload(
    remote_client: vercel_cache_helper::vercel::remote_client::RemoteClient,
    path: &Option<std::path::PathBuf>,
) -> vercel_cache_helper::Result<()> {
    let project_dir = if let Some(path) = path {
        path.clone()
    } else {
        std::env::current_dir()?
    };

    let output_dir = project_dir.join(".output");

    if !output_dir.exists() {
        println!("Output dir does not exist: {:?}", output_dir);
        return Ok(());
    }

    let mut output_dir_archive = tempfile::tempfile()?;

    vercel_cache_helper::utils::create_tar_gz_archive(&output_dir, &output_dir_archive)?;

    output_dir_archive.seek(std::io::SeekFrom::Start(0)).unwrap();

    let mut output_archive_buf: Vec<u8> = Vec::new();

    let output_archive_size = output_dir_archive.read_to_end(&mut output_archive_buf)?;

    println!("Output archive bytes read: {} bytes", output_archive_size);

    let mut output_put_req = remote_client.put(FASTN_VERCEL_REMOTE_BUILD_HASH.to_string(), None)?;

    println!("Uploading .output archive");

    output_put_req
        .buffer(&mut output_archive_buf, output_archive_size)
        .await?;

    println!("Uploaded .output archive");

    println!("done!");

    Ok(())
}
