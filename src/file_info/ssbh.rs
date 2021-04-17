use binread::{BinReaderExt, io::Cursor};

use ssbh_lib::{Ssbh, SsbhFile, SsbhString};
use ssbh_lib::formats::skel::SkelBoneEntry;
use ssbh_lib::formats::mesh::MeshObject;
use ssbh_lib::formats::matl::{MatlEntry, MatlAttribute, Param};
use ssbh_lib::formats::modl::ModlEntry;
use ssbh_lib::formats::hlpb::{HlpbRotateAim, HlpbRotateInterpolation};

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
    };
    (modl) => {
r#"Namco Model File v{}.{}

Model: {:?}
Skeleton File: {:?}
Animation File: {:?}
Mesh File: {:?}

Material Files:
{}

Entries:
{}
"#
    };
    (hlpb) => {
r#"Namco Helper Bone File v{}.{}

Rotate Aim Entries: {}
Rotate Interpolation Entries: {}

Rotate Aim Entries:
{}

Rotate Interpolation Entries:
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
        SsbhFile::Modl(modl) => {
            format!(
                fmt_lit!(modl),
                modl.major_version,
                modl.minor_version,
                modl.model_file_name.get_string().unwrap_or("None"),
                modl.skeleton_file_name.get_string().unwrap_or("None"),
                modl.animation_file_name.as_ref()
                    .map(|s| s.get_string())
                    .flatten()
                    .unwrap_or("None"),
                modl.mesh_string.get_string().unwrap_or("None"),
                str_list(&modl.material_file_names.elements),
                modl_entry_list(&modl.entries.elements),
            )
        }
        SsbhFile::Hlpb(hlpb) => {
            format!(
                fmt_lit!(hlpb),
                hlpb.major_version,
                hlpb.minor_version,
                hlpb.aim_entries.elements.len(),
                hlpb.interpolation_entries.elements.len(),
                aim_list(&hlpb.aim_entries.elements),
                interpolation_list(&hlpb.interpolation_entries.elements),
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

fn str_list(strs: &[SsbhString]) -> String {
    strs.iter()
        .filter_map(|string| Some(format!("    - {:?}", string.get_string()?)))
        .collect::<Vec<String>>()
        .join("\n")
}

fn modl_entry_list(entries: &[ModlEntry]) -> String {
    entries.iter()
        .map(|entry| format!(
            "    - {}[{}]: {:?}",
            entry.mesh_name.get_string().unwrap_or("None"),
            entry.sub_index,
            entry.material_label.get_string().unwrap_or("UnknownMaterial"),
        ))
        .collect::<Vec<String>>()
        .join("\n")

}

macro_rules! aim_fmt {
    () => {
r#"- {}
    - AimBone[0]: {:?}
    - AimBone[1]: {:?}
    - AimType[0]: {:?}
    - AimType[1]: {:?}
    - TargetBone[0]: {:?}
    - TargetBone[1]: {:?}
"#
    };
}

macro_rules! ssbh_str {
    ($expr:expr) => {
        $expr.get_string().unwrap_or("None")
    };
}

fn aim_list(entries: &[HlpbRotateAim]) -> String {
    entries.iter()
        .map(|aim| format!(
            aim_fmt!(),
            ssbh_str!(aim.name),
            ssbh_str!(aim.aim_bone_name1),
            ssbh_str!(aim.aim_bone_name2),
            ssbh_str!(aim.aim_type1),
            ssbh_str!(aim.aim_type2),
            ssbh_str!(aim.target_bone_name1),
            ssbh_str!(aim.target_bone_name2),
        ))
        .collect::<Vec<String>>()
        .join("\n")
}

macro_rules! interp_fmt {
    () => {
r#"- {}
    - Bone: {:?}
    - Root Bone: {:?}
    - Parent Bone: {:?}
    - Driver Bone: {:?}
    - Minimum Range: ({}, {}, {})
    - Maximum Range: ({}, {}, {})
"#
    };
}

fn interpolation_list(entries: &[HlpbRotateInterpolation]) -> String {
    entries.iter()
        .map(|interp| format!(
            interp_fmt!(),
            ssbh_str!(interp.name),
            ssbh_str!(interp.bone_name),
            ssbh_str!(interp.root_bone_name),
            ssbh_str!(interp.parent_bone_name),
            ssbh_str!(interp.driver_bone_name),
            interp.range_min.x, interp.range_min.y, interp.range_min.z,
            interp.range_max.x, interp.range_max.y, interp.range_max.z,
        ))
        .collect::<Vec<String>>()
        .join("\n")
}
