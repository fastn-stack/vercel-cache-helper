use std::io::{Seek, Write};

use crate::vercel::constants::FASTN_VERCEL_REMOTE_BUILD_HASH;

pub async fn download(
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
        std::fs::create_dir(&output_dir).expect("Failed to create .output dir.");
    }

    println!("Looking for artifacts...");

    let mut output_exists_req =
        remote_client.exists(FASTN_VERCEL_REMOTE_BUILD_HASH.to_string(), None)?;
    let output_artifact_exists = output_exists_req.send().await?;

    if output_artifact_exists {
        println!(".output artifact found");
    } else {
        println!(".output artifact not found");
        return Ok(());
    }

    println!("Downloading .output artifact");

    let mut output_dir_archive = tempfile::tempfile()?;
    let mut output_get_req = remote_client.get(FASTN_VERCEL_REMOTE_BUILD_HASH.to_string(), None)?;
    let output_get_res = output_get_req.get().await?;

    println!("Downloaded .output artifact");

    output_dir_archive.write_all(&output_get_res.bytes().await?.to_vec())?;

    output_dir_archive
        .seek(std::io::SeekFrom::Start(0))
        .unwrap();

    vercel_cache_helper::utils::extract_tar_gz(output_dir_archive, &output_dir)?;

    println!("done!");

    Ok(())
}
