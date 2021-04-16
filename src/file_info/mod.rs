use smash_arc::ArcFile;
use std::path::Path;

pub fn get(arc: &ArcFile, path: &str) -> String {
    let extension = Path::new(path).extension().map(|x| x.to_string_lossy().into_owned());

    if let Some(ext) = extension {
        match &*ext {
            "nutexb" => {
                format!("Namco Texture")
            }
            _ => format!("No info")
        }
    } else {
        format!("No info")
    }
}
