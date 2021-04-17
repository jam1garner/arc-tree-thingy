use ssbh_lib::{Ssbh, SsbhFile};
use binread::{BinReaderExt, io::Cursor};

use ssbh_lib::formats::skel::SkelBoneEntry;

// Format strings must literals, so to store multi-line ones, I'll just use raw strings
// in a macro, it's bad, I know
macro_rules! fmt_lit {
    (anim) => {
r#"Namco Animation File v{}.{}

Name: {:?}
Animations: {}
Frame count: {}
"#
    };
    (skel) => {
r#"Namco Skeleton File v{}.{}

Bone count: {}

Bones:
{}
"#
    };
}

pub fn info(contents: Vec<u8>) -> String {
    let mut contents = Cursor::new(contents);
    let ssbh: Ssbh = match contents.read_le() {
        Ok(x) => x,
        Err(_) => return "No file info".to_owned()
    };

    match ssbh.data {
        SsbhFile::Anim(anim) => {
            format!(
                fmt_lit!(anim),
                anim.major_version,
                anim.minor_version,
                &anim.name.get_string().unwrap_or("None"),
                anim.animations.elements.len(),
                anim.final_frame_index,
            )
        }
        SsbhFile::Skel(skel) => {
            format!(
                fmt_lit!(skel),
                skel.major_version,
                skel.minor_version,
                skel.bone_entries.elements.len(),
                bone_list(&skel.bone_entries.elements),
            )
        }
        _ => "SSBH File".to_owned()
    }
}

fn bone_list(bones: &[SkelBoneEntry]) -> String {
    bones.iter()
        .filter_map(|bone| Some(format!("- {}", bone.name.get_string()?)))
        .collect::<Vec<String>>()
        .join("\n")
}
