use std::{fs, path::Path};

pub fn is_file(path: &str) -> bool {
    if let Ok(metadata) = fs::metadata(path) {
        return metadata.is_file();
    }
    false
}

pub fn is_dir(path: &str) -> bool {
    if let Ok(metadata) = fs::metadata(path) {
        return metadata.is_dir();
    }
    false
}

pub fn get_file(dir: &str) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();
    let entries = fs::read_dir(dir).unwrap();

    for entry in entries {
        let path = entry.unwrap().path();
        vec.push(path.display().to_string());
    }
    vec
}

pub fn is_music(file_name: &str) -> bool {
    if is_file(&file_name) {
        let music_extensions = vec![".flac", ".mp3", ".aac", ".wav", ".m4a"];
        for ext in music_extensions {
            if file_name.ends_with(ext) {
                return true;
            }
        }
        return false;
    }
    false
}

pub fn pathinfo(path: &str) -> (Option<&str>, Option<&str>, Option<&str>) {
    let path = Path::new(path);
    let filename = path.file_name().and_then(|os_str| os_str.to_str());
    let extension = path.extension().and_then(|os_str| os_str.to_str());
    let dirname = path.parent().and_then(|os_str| os_str.to_str());
    (dirname, filename, extension)
}
