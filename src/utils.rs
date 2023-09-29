use std::io::Read;

fn id_to_cache_key(id: &str) -> String {
    // TODO: use MAIN_SEPARATOR here
    id.replace(['/', '\\'], "_")
}

pub fn get_cache_dir() -> Option<std::path::PathBuf> {
    let cache_dir = dirs::cache_dir()?;
    let base_path = cache_dir.join("fastn.com");

    if !base_path.exists() {
        if let Err(err) = std::fs::create_dir_all(&base_path) {
            eprintln!("Failed to create cache directory: {}", err);
            return None;
        }
    }

    Some(
        base_path.join(id_to_cache_key(
            &std::env::current_dir()
                .expect("can't read current dir")
                .to_string_lossy(),
        )),
    )
}

pub fn generate_hash(content: impl AsRef<[u8]>) -> String {
    use sha2::digest::FixedOutput;
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(content);
    format!("{:X}", hasher.finalize_fixed())
}

pub fn generate_file_hash(mut file: std::fs::File) -> vercel_cache_helper::Result<String> {
    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf)?;
    Ok(generate_hash(buf))
}

pub fn create_tar_gz_archive(
    src_folder: &std::path::PathBuf,
    dest_file: &std::fs::File,
) -> vercel_cache_helper::Result<()> {
    println!("Creating archive from: {:?}", src_folder);

    if !src_folder.exists() {
        println!("Source folder does not exist: {:?}", src_folder);
        return Ok(());
    }

    if std::fs::read_dir(src_folder)?.next().is_none() {
        println!("Source folder is empty: {:?}", src_folder);
        return Ok(());
    }

    let gz_encoder = std::io::BufWriter::new(flate2::write::GzEncoder::new(
        dest_file,
        flate2::Compression::default(),
    ));
    let mut tar_builder = tar::Builder::new(gz_encoder);

    tar_builder.append_dir_all("", src_folder).map_err(|e| {
        println!("Error creating archive: {:?}", e);
        e
    })?;

    println!("Archive created successfully.");
    Ok(())
}

pub fn extract_tar_gz(
    file: std::fs::File,
    dest_path: &std::path::Path,
) -> vercel_cache_helper::Result<()> {
    println!(
        "Preparing to extract archive in {}...",
        dest_path.to_string_lossy()
    );
    let archive = flate2::read::GzDecoder::new(file);
    let mut archive = tar::Archive::new(archive);
    if let Err(err) = archive.unpack(dest_path) {
        println!("Error extracting archive: {}", err);
        return Err(err.into());
    }
    println!("Unpacked archive in: {}", &dest_path.to_string_lossy());
    Ok(())
}
