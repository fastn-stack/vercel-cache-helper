use indicatif::{ProgressBar, ProgressStyle};
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
        println!("Cache dir found: '{:?}'", cache_dir);
        cache_dir
    } else {
        println!("Cache dir not found.");
        return Ok(());
    };
    let output_dir = tempfile::tempdir()?;

    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(std::time::Duration::new(0, 500));
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("[{spinner}] {prefix} {wide_msg}")
            .unwrap()
            .tick_chars("/|\\- "),
    );
    pb.set_message("Looking for artifacts...");
    let mut output_exists_req = remote_client.exists(
        vercel_cache_helper::vercel::constants::FASTN_VERCEL_REMOTE_CACHE_HASH.to_string(),
        None,
    )?;
    let output_artifact_exists = output_exists_req.send().await?;

    if output_artifact_exists {
        pb.finish_with_message("Remote artifact found");
    } else {
        pb.finish_with_message("No remote artifact found");
        return Ok(());
    }

    pb.reset();
    pb.set_message("Downloading artifacts");

    let mut output_dir_archive = tempfile::tempfile()?;
    let mut output_get_req = remote_client.get(
        vercel_cache_helper::vercel::constants::FASTN_VERCEL_REMOTE_CACHE_HASH.to_string(),
        None,
    )?;

    let mut output_get_res = output_get_req.get().await?;
    let download_size = output_get_res.content_length().unwrap_or(0);

    let mut downloaded_bytes: u64 = 0;

    while let Some(chunk) = output_get_res.chunk().await? {
        output_dir_archive.write_all(&chunk)?;

        downloaded_bytes += chunk.len() as u64;

        if download_size > 0 {
            pb.set_position(downloaded_bytes);
            pb.set_length(download_size);
        }
    }

    pb.finish_with_message("Remote artifacts downloaded");

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

    println!("Cached artifacts downloaded and copied successfully.");

    Ok(())
}
