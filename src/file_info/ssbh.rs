use binread::{BinReaderExt, io::Cursor};

use ssbh_lib::{Ssbh, SsbhFile};
use ssbh_lib::formats::skel::SkelBoneEntry;
use ssbh_lib::formats::mesh::MeshObject;
use ssbh_lib::formats::matl::{MatlEntry, MatlAttribute, Param};

use std::fmt::Display;

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
    (mesh) => {
r#"Namco Mesh File v{}.{}

Mesh name: {}
Object count: {}
Vertex count: {}

Mesh Objects:
{}
"#
    };
    (matl) => {
r#"Namco Material Parameter File v{}.{}

Materials ({}):
{}
"#
    }
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
        SsbhFile::Mesh(mesh) => {
            format!(
                fmt_lit!(mesh),
                mesh.major_version,
                mesh.minor_version,
                mesh.model_name.get_string().unwrap_or("None"),
                mesh.objects.elements.len(),
                vert_count(&mesh.objects.elements),
                mesh_list(&mesh.objects.elements),
            )
        }
        SsbhFile::Matl(matl) => {
            format!(
                fmt_lit!(matl),
                matl.major_version,
                matl.minor_version,
                matl.entries.elements.len(),
                matl_list(&matl.entries.elements),
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

fn mesh_list(meshes: &[MeshObject]) -> String {
    meshes.iter()
        .filter_map(|mesh| Some(format!("- {}", mesh.name.get_string()?)))
        .collect::<Vec<String>>()
        .join("\n")
}

fn vert_count(meshes: &[MeshObject]) -> usize {
    meshes.iter()
        .map(|mesh| mesh.vertex_count as usize)
        .sum()
}

macro_rules! matl_fmt {
    () => {
r#"- {}
    - Shader: {}
    - Attributes:
{}
"#
    };
}

fn matl_list(matls: &[MatlEntry]) -> String {
    matls.iter()
        .map(|matl| format!(
            matl_fmt!(),
            matl.material_label.get_string().unwrap_or("UnknownMaterial"),
            matl.shader_label.get_string().unwrap_or("UnknownShader"),
            matl_attr_list(&matl.attributes.elements),
        ))
        .collect::<Vec<String>>()
        .join("\n")
}

fn matl_attr_list(attrs: &[MatlAttribute]) -> String {
    attrs.iter()
        .filter_map(|attr| Some(match (*attr.param.data).as_ref()? {
            Param::Float(flt) => format!("        - {:?}: {}", attr.param_id, flt),
            Param::Boolean(bl) => format!("        - {:?}: {}", attr.param_id, match bl {
                0 => "false".to_string(),
                1 => "true".to_string(),
                val => val.to_string(),
            }),
            Param::Vector4(vec) => format!(
                "        - {:?}: ({}, {}, {}, {})",
                attr.param_id,
                vec.x,
                vec.y,
                vec.z,
                vec.w
            ),
            Param::MatlString(string) => format!(
                "        - {:?}: {:?}",
                attr.param_id,
                string.get_string().unwrap_or("")
            ),
            _ => format!("        - {:?}", attr.param_id)
        }))
        .collect::<Vec<String>>()
        .join("\n")
}
