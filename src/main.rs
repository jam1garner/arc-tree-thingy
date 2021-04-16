use fltk::{app, prelude::*, frame::*, group::*, button::*, window::*, tree::{Tree, TreeItem, TreeReason, TreeSort}};
use smash_arc::{ArcFile, FileNode, Hash40};

use std::sync::Mutex;
use std::rc::Rc;

fn build_tree(arc: &ArcFile, tree: &mut Tree, hash: impl Into<Hash40>, depth_left: usize) -> Result<(), ()> {
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

fn get_path(tree_item: TreeItem) -> String {
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

fn main() {
    let app = app::App::default();
    //let arc_path = fltk::dialog::file_chooser("Open data.arc", "*.arc", ".", false).unwrap();
    //let label_path = fltk::dialog::file_chooser("Open Labels", "*", ".", false).unwrap();
    let arc_path = "/home/jam/re/ult/900/data.arc";
    let label_path = "/home/jam/dev/ult/smash-arc/hash_labels.txt";
    Hash40::set_global_labels_file(&label_path).unwrap();
    let arc = Box::leak(Box::new(ArcFile::open(arc_path).unwrap()));

    let mut wind = Window::default()
        .with_size(750, 500)
        .center_screen()
        .with_label("Arc Tree Thing");


    let mut pack = Pack::default().size_of(&wind).center_of(&wind);
    pack.set_type(PackType::Horizontal);
    pack.set_spacing(0);
    
    let mut tree_pack = Pack::default().with_size(500, 500);

    let mut tree = Tree::new(0, 0, 500, 475, "Tree");
    tree.set_root_label("/");
    tree.set_sort_order(TreeSort::Ascending);

    build_tree(&arc, &mut tree, "/", 1).unwrap();
    tree.set_callback2(move |tree| {
        match tree.callback_reason() {
            TreeReason::Opened => {
                let mut path = get_path(tree.callback_item().unwrap());
                if let Err(_) = build_tree(arc, tree, &*path, 3) {
                    path.push('/');
                    build_tree(arc, tree, &*path, 3).unwrap();
                }
            }
            _ => ()
        }
    });

    tree.get_items()
        .unwrap()
        .into_iter()
        .for_each(|mut node| node.close());
    tree.root().unwrap().open();
    
    tree_pack.add(&tree);
    let tree = Rc::new(Mutex::new(tree));
    let tree_ref = Rc::clone(&tree);

    let mut button_pack = Pack::default().with_size(500, 25);

    let mut extract_button = Button::default().with_size(50, 25).with_label("Extract");
    extract_button.set_callback(move || println!("{}", tree_ref.lock().unwrap().first_selected_item().unwrap().label().unwrap()));
    button_pack.add(&extract_button);

    tree_pack.add(&button_pack);
    tree_pack.end();
    pack.add(&tree_pack);

    let mut frame = Frame::default().with_size(250, 500).right_of(&*tree.lock().unwrap(), 0);
    frame.set_color(Color::Red);

    let mut button = Button::default().size_of(&frame).center_of(&frame).with_label("Button1");

    button.set_callback(move || println!("Button hit"));
    pack.add(&frame);
    pack.end();

    wind.make_resizable(true);
    wind.end();
    wind.show();
    app.run().unwrap();
}
