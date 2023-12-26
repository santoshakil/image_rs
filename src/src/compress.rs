use std::{error::Error, fs, io::Seek, path::PathBuf};

pub fn compress(path: PathBuf, output: usize) -> Result<String, Box<dyn Error>> {
    let img = image::open(&path)?;
    let mut image = img.to_rgb8();
    let mut temp = tempfile::tempfile()?;
    image::codecs::jpeg::JpegEncoder::new(&mut temp).encode(
        &image,
        image.width(),
        image.height(),
        image::ColorType::Rgb8,
    )?;
    let mut file_size = temp.metadata()?.len() as usize;
    if file_size <= output {
        return Ok(path.to_str().unwrap().to_string());
    }
    while file_size > output {
        let scale = (output as f64 / file_size as f64).sqrt();
        let compressed_image = image::imageops::resize(
            &image,
            (image.width() as f64 * scale) as u32,
            (image.height() as f64 * scale) as u32,
            image::imageops::FilterType::Lanczos3,
        );
        image = compressed_image;
        temp = tempfile::tempfile()?;
        image::codecs::jpeg::JpegEncoder::new(&mut temp).encode(
            &image,
            image.width(),
            image.height(),
            image::ColorType::Rgb8,
        )?;
        file_size = temp.metadata()?.len() as usize;
    }
    let mut output_path = path.clone();
    let file_name = output_path.file_name().unwrap().to_str().unwrap();
    let compressed_file_name = format!("compressed_{}", file_name);
    output_path.set_file_name(compressed_file_name);
    output_path.set_extension("jpg");
    let mut output_file = fs::File::create(&output_path)?;
    temp.seek(std::io::SeekFrom::Start(0))?;
    std::io::copy(&mut temp, &mut output_file)?;
    return Ok(output_path.to_str().unwrap().to_string());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_compress() {
        let path = Path::new("image/4mb.jpg");
        let output = 1024 * 200;
        let result = compress(path.to_path_buf(), output);
        assert!(result.is_ok());
    }
}
