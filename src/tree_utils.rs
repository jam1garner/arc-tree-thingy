use fltk::{app, tree::{Tree, TreeItem}, dialog::{FileChooser, FileChooserType}};
use smash_arc::{ArcFile, ArcLookup, FileNode, Hash40};

use std::path::Path;

pub fn build_tree(arc: &ArcFile, tree: &mut Tree, hash: impl Into<Hash40>, depth_left: usize) -> Result<(), ()> {
    let listing = match arc.get_dir_listing(hash) {
        Some(listing) => listing,
        None => return Err(())
    };

    for node in listing {
        match node {
            FileNode::Dir(dir) => {
                let path = dir.global_label();
                if let Some(path) = path.as_deref() {
                    tree.add(&path);
                }
                if depth_left > 0 {
                    build_tree(arc, tree, dir, depth_left - 1).unwrap();
                }
                if let Some(path) = path {
                    let _ = tree.close(&path, false);
                }
            }
            FileNode::File(file) => {
                if let Some(path) = file.global_label() {
                    tree.add(&path);
                }
            }
        }
    }

    Ok(())
}

pub fn get_path(tree_item: TreeItem) -> String {
    if let Some(label) = tree_item.label() {
        if let Some(parent) = tree_item.parent() {
            let path = get_path(parent);
            if path == "/" {
                label
            } else {
                format!("{}/{}", path, label)
            }
        } else {
            label
        }
    } else {
        "".to_owned()
    }
}

pub fn extract_tree_item(arc: &ArcFile, tree_item: TreeItem) {
    let path = get_path(tree_item);
    let contents = arc.get_file_contents(&*path, smash_arc::Region::UsEnglish).unwrap();
    let path = Path::new(&path);
    let file_name = path.file_name().unwrap().to_string_lossy();

    let out = {
        let mut dialog = FileChooser::new(".", "*", FileChooserType::Create, "Extract File");
        dialog.set_value(&file_name);
        dialog.show();
        while dialog.shown() {
            app::wait();
        }

        dialog.value(1)
    };

    if let Some(path) = out {
        std::fs::write(
            path,
            contents
        ).unwrap();
    }
}
