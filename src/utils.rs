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

// Copy files from source to destination recursively.
// https://nick.groenen.me/notes/recursively-copy-files-in-rust/
pub fn copy_recursively(
    source: impl AsRef<std::path::Path>,
    destination: impl AsRef<std::path::Path>,
) -> vercel_cache_helper::Result<()> {
    std::fs::create_dir_all(&destination)?;
    for entry in std::fs::read_dir(source)? {
        let entry = entry?;
        let filetype = entry.file_type()?;
        if filetype.is_dir() {
            copy_recursively(entry.path(), destination.as_ref().join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), destination.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
pub fn create_tar_zst_archive(
    src_folder: &std::path::PathBuf,
    dest_file: &std::fs::File,
) -> std::io::Result<()> {
    println!("Creating archive from: {:?}", src_folder);

    if !src_folder.exists() {
        println!("Source folder does not exist: {:?}", src_folder);
        return Ok(());
    }

    if std::fs::read_dir(src_folder)?.next().is_none() {
        println!("Source folder is empty: {:?}", src_folder);
        return Ok(());
    }

    // Create the Zstd encoder with proper error handling
    let zst_encoder =
        match zstd::stream::write::Encoder::new(dest_file, zstd::DEFAULT_COMPRESSION_LEVEL) {
            Ok(encoder) => encoder,
            Err(err) => {
                println!("Error creating Zstd encoder: {:?}", err);
                return Err(err);
            }
        };

    let mut tar_builder = tar::Builder::new(zst_encoder);

    tar_builder.append_dir_all("", src_folder)?;

    // Ensure the archive is properly flushed and closed
    match tar_builder.into_inner()?.finish() {
        Ok(_) => {
            println!("Archive created successfully.");
            Ok(())
        }
        Err(err) => {
            println!("Error closing the archive: {:?}", err);
            Err(err)
        }
    }
}

pub fn extract_tar_zst(file: std::fs::File, dest_path: &std::path::PathBuf) -> std::io::Result<()> {
    println!(
        "Preparing to extract archive in {}...",
        dest_path.to_string_lossy()
    );

    let zst_decoder = zstd::stream::read::Decoder::new(file)?;

    let mut archive = tar::Archive::new(zst_decoder);

    // Ensure that directories are created as needed while extracting
    archive.unpack(dest_path)?;

    println!("Unpacked archive in: {}", &dest_path.to_string_lossy());
    Ok(())
}
