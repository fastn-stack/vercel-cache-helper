use std::io::Read;

pub async fn upload(
    remote_client: vercel_cache_helper::vercel::remote_cache_client::RemoteClient,
) -> vercel_cache_helper::Result<()> {
    println!("Here");
    let cache_dir = if let Some(cache_dir) = vercel_cache_helper::utils::get_cache_dir() {
        println!("Cache dir found: {:?}", cache_dir);
        cache_dir
    } else {
        println!("Cache dir not found");
        return Ok(());
    };

    let current_dir = std::env::current_dir()?;
    let build_dir = current_dir.join(".build");

    if !build_dir.exists() {
        println!("Build dir does not exist: {:?}", build_dir);
        return Ok(());
    }

    let mut build_dir_archive = tempfile::tempfile()?;
    let mut cache_dir_archive = tempfile::tempfile()?;

    vercel_cache_helper::utils::create_tar_gz_archive(&cache_dir, &cache_dir_archive)?;
    vercel_cache_helper::utils::create_tar_gz_archive(&build_dir, &build_dir_archive)?;

    let mut build_archive_buf: Vec<u8> = Vec::new();
    let mut cache_archive_buf: Vec<u8> = Vec::new();

    build_dir_archive.read_to_end(&mut build_archive_buf)?;
    cache_dir_archive.read_to_end(&mut cache_archive_buf)?;

    let build_archive_size = build_dir_archive.metadata()?.len();
    let cache_archive_size = cache_dir_archive.metadata()?.len();

    println!("Build archive size: {} bytes", build_archive_size);
    println!("Cache archive size: {} bytes", cache_archive_size);

    let build_archive_hash = vercel_cache_helper::utils::generate_hash(&build_archive_buf);
    let cache_dir_hash = vercel_cache_helper::utils::generate_hash(&cache_archive_buf);

    println!("Build archive hash: {:?}", build_archive_hash);
    println!("Cache dir hash: {:?}", cache_dir_hash);

    let mut build_put_req = remote_client.put(build_archive_hash, None)?;
    build_put_req.buffer(&mut build_archive_buf, build_archive_size).await?;

    let mut cache_put_req = remote_client.put(cache_dir_hash, None)?;
    let res = cache_put_req.buffer(&mut cache_archive_buf, cache_archive_size).await?;

    println!("Response: {:?}", res);
    println!("Response body: {:?}", res.text().await?);

    println!("done!");

    Ok(())
}