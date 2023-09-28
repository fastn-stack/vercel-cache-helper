pub async fn download(
    remote_client: vercel_cache_helper::vercel::remote_cache_client::RemoteClient,
) -> vercel_cache_helper::Result<()> {
    let cache_dir = if let Some(cache_dir) = vercel_cache_helper::utils::get_cache_dir() {
        println!("Cache dir found: {:?}", cache_dir);
        cache_dir
    } else {
        return Ok(());
    };

    let current_dir = std::env::current_dir()?;
    let cache_key_path = current_dir.join(".cache_key");
    let build_dir = current_dir.join(".build");

    if !build_dir.exists() {
        std::fs::create_dir(&build_dir).expect("Failed to create .build dir.");
    }

    let cache_key_file =
        std::fs::read_to_string(cache_key_path).expect(".cache_key file not found");

    let hash_keys = cache_key_file.split_once("\n").unwrap();

    let (cache_dir_hash, build_archive_hash) = hash_keys;

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

    let mut build_get_req = remote_client.get(build_archive_hash.to_string(), None)?;

    let build_get_res = build_get_req.get().await?;

    println!("Downloaded .build artifact");

    vercel_cache_helper::utils::extract_tar_gz(&build_get_res.bytes().await?.to_vec(), &build_dir)?;

    println!("Downloading cache artifact");

    let mut cache_get_req = remote_client.get(build_archive_hash.to_string(), None)?;

    let cache_get_res = cache_get_req.get().await?;

    println!("Downloaded cache artifact");

    vercel_cache_helper::utils::extract_tar_gz(&cache_get_res.bytes().await?.to_vec(), &cache_dir)?;

    println!("done!");

    Ok(())
}
