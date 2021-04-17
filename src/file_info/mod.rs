use smash_arc::{ArcFile, ArcLookup};
use std::path::Path;

mod nutexb;
mod ssbh;

pub fn get(arc: &ArcFile, path: &str) -> String {
    let extension = Path::new(path)
        .extension()
        .map(|x| x.to_string_lossy().into_owned());

    macro_rules! get_contents {
        () => {
            match arc.get_file_contents(path, smash_arc::Region::UsEnglish) {
                Ok(x) => x,
                Err(_) => return String::from("Could not open file"),
            }
        };
    }

    if let Some(ext) = extension {
        match &*ext {
            "nutexb" => nutexb::info(get_contents!()),
            "nuhlpb" | "numatb" | "numdlb" | "nusrcmdlb" | "numshb" | "nusktb" | "nuanmb"
            | "nurpdb" | "nufxlb" | "nushdb" => ssbh::info(get_contents!()),
            _ => format!("No info"),
        }
    } else {
        format!("No info")
    }
}
