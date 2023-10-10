use indicatif::{ProgressBar, ProgressStyle};
use std::io::{Seek, Read};

pub async fn upload(
    remote_client: vercel_cache_helper::vercel::remote_client::RemoteClient,
    path: &Option<std::path::PathBuf>,
) -> vercel_cache_helper::Result<()> {
    let cache_dir = if let Some(cache_dir) = vercel_cache_helper::utils::get_cache_dir() {
        println!("Cache dir found: {:?}", cache_dir);
        cache_dir
    } else {
        println!("Cache dir not found");
        return Ok(());
    };

    let project_dir = if let Some(path) = path {
        path.clone()
    } else {
        std::env::current_dir()?
    };

    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(std::time::Duration::new(0, 500));
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("[{spinner}] {prefix} {wide_msg}")
            .unwrap()
            .tick_chars("/|\\- "),
    );
    pb.set_message("Preparing to upload artifacts...");

    let build_dir = project_dir.join(".build");
    let output_dir = tempfile::tempdir()?;
    let build_dir_dest = output_dir.path().join(".build");
    let cache_dir_dest = output_dir.path().join("cache");

    vercel_cache_helper::utils::copy_recursively(build_dir, build_dir_dest)?;
    vercel_cache_helper::utils::copy_recursively(cache_dir, cache_dir_dest)?;

    let mut output_dir_archive = tempfile::tempfile()?;

    vercel_cache_helper::utils::create_filtered_tar_zst_archive(
        &output_dir.path().to_path_buf(),
        &output_dir_archive,
    )?;

    pb.finish_with_message("Artifact archive created.");
    pb.reset();

    output_dir_archive
        .seek(std::io::SeekFrom::Start(0))
        .unwrap();

    let mut output_archive_buf: Vec<u8> = Vec::new();
    let output_archive_size = output_dir_archive.read_to_end(&mut output_archive_buf)?;

    println!("Output archive bytes read: {} bytes", output_archive_size);

    let mut output_put_req = remote_client.put(
        vercel_cache_helper::vercel::constants::FASTN_VERCEL_REMOTE_CACHE_HASH.to_string(),
        None,
    )?;

    pb.set_message("Uploading artfiacts...");

    let response = output_put_req
        .buffer(&mut output_archive_buf, output_archive_size).await?;

    if !response.status().is_success() {
        println!("Could not upload artifacts.");
        return Ok(());
    }

    pb.finish_with_message("Artifacts uploaded successfully.");

    Ok(())
}
