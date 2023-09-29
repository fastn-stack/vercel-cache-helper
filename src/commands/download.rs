use std::io::{Seek, Write};

pub async fn download(
    remote_client: vercel_cache_helper::vercel::remote_client::RemoteClient,
    path: &Option<std::path::PathBuf>,
) -> vercel_cache_helper::Result<()> {
    let cache_dir = if let Some(cache_dir) = vercel_cache_helper::utils::get_cache_dir() {
        println!("Cache dir found: {:?}", cache_dir);
        cache_dir
    } else {
        return Ok(());
    };

    let project_dir = if let Some(path) = path {
        path.clone()
    } else {
        std::env::current_dir()?
    };
    let build_dir = project_dir.join(".build");

    if !build_dir.exists() {
        std::fs::create_dir(&build_dir).expect("Failed to create .build dir.");
    }

    let cache_dir_hash = std::env::var(
        vercel_cache_helper::vercel::constants::FASTN_VERCEL_REMOTE_BUILD_CACHE_HASH.to_string(),
    )?;
    let build_archive_hash = std::env::var(
        vercel_cache_helper::vercel::constants::FASTN_VERCEL_REMOTE_BUILD_HASH.to_string(),
    )?;

    println!("Build archive hash: {:?}", build_archive_hash);
    println!("Cache dir hash: {:?}", cache_dir_hash);

    println!("Looking for artifacts...");

    let mut build_exists_req = remote_client.exists(build_archive_hash.to_string(), None)?;

    let build_artifact_exists = build_exists_req.send().await?;

    if build_artifact_exists {
        println!(".build artifact found");
    } else {
        println!(".build artifact not found");
        return Ok(());
    }

    let mut cache_exists_req = remote_client.exists(cache_dir_hash.to_string(), None)?;

    let cache_artifact_exists = cache_exists_req.send().await?;

    if cache_artifact_exists {
        println!("cache artifact found");
    } else {
        println!("cache artifact not found");
        return Ok(());
    }

    println!("Downloading .build artifact");

    let mut build_dir_archive = tempfile::tempfile()?;

    let mut build_get_req = remote_client.get(build_archive_hash.to_string(), None)?;

    let build_get_res = build_get_req.get().await?;

    println!("Downloaded .build artifact");

    build_dir_archive.write_all(&build_get_res.bytes().await?.to_vec())?;

    build_dir_archive.seek(std::io::SeekFrom::Start(0))?;

    vercel_cache_helper::utils::extract_tar_gz(build_dir_archive, &build_dir)?;

    println!("Downloading cache artifact");

    let mut cache_dir_archive = tempfile::tempfile()?;

    let mut cache_get_req = remote_client.get(build_archive_hash.to_string(), None)?;

    let cache_get_res = cache_get_req.get().await?;

    println!("Downloaded cache artifact");

    cache_dir_archive.write_all(&cache_get_res.bytes().await?.to_vec())?;

    cache_dir_archive.seek(std::io::SeekFrom::Start(0))?;

    vercel_cache_helper::utils::extract_tar_gz(cache_dir_archive, &cache_dir)?;

    println!("done!");

    Ok(())
}
