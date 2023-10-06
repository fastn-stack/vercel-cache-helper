use std::io::{Seek, Write};

pub async fn download(
    remote_client: vercel_cache_helper::vercel::remote_client::RemoteClient,
    path: &Option<std::path::PathBuf>,
) -> vercel_cache_helper::Result<()> {
    let project_dir = if let Some(path) = path {
        path.clone()
    } else {
        std::env::current_dir()?
    };
    let cache_dir = if let Some(cache_dir) = vercel_cache_helper::utils::get_cache_dir() {
        println!("Cache dir found: {:?}", cache_dir);
        cache_dir
    } else {
        println!("Cache dir not found");
        return Ok(());
    };
    let output_dir = tempfile::tempdir()?;

    println!("Looking for artifacts...");

    let mut output_exists_req = remote_client.exists(
        vercel_cache_helper::vercel::constants::FASTN_VERCEL_REMOTE_CACHE_HASH.to_string(),
        None,
    )?;
    let output_artifact_exists = output_exists_req.send().await?;

    if !output_artifact_exists {
        println!("Build artifacts not found.");
        return Ok(());
    }

    println!("Build artifacts found. Downloading build artifacts...");

    let mut output_dir_archive = tempfile::tempfile()?;
    let mut output_get_req = remote_client.get(
        vercel_cache_helper::vercel::constants::FASTN_VERCEL_REMOTE_CACHE_HASH.to_string(),
        None,
    )?;

    let output_get_res = output_get_req.get().await?;

    assert!(output_get_res.status().is_success(), "Build artifacts could not be downloaded.");

    println!("Build artifacts downloaded");

    let buf = &output_get_res.bytes().await?.to_vec();

    assert!(vercel_cache_helper::utils::is_zstd_compressed(&buf), "Downloaded archive (Size: {}) is not zstd compressed.", &buf.len());

    output_dir_archive.write_all(buf)?;

    output_dir_archive
        .seek(std::io::SeekFrom::Start(0))
        .unwrap();

    vercel_cache_helper::utils::extract_tar_zst(
        output_dir_archive,
        &output_dir.path().to_path_buf(),
    )?;

    let temp_build_dir = output_dir.path().join(".build");
    let temp_cache_dir = output_dir.path().join("cache");

    vercel_cache_helper::utils::copy_recursively(temp_build_dir, project_dir.join(".build"))?;
    vercel_cache_helper::utils::copy_recursively(temp_cache_dir, cache_dir)?;

    println!("Remote cache retrieval completed successfully.");

    Ok(())
}
