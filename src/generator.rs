use crate::enums::{
    FileType,
    Value
};

use crate::trees::*;

use std::{
    io::BufWriter,
    fs::File,
};

use rbx_dom_weak::RbxTree;

// handles xml writing
fn handle_xml(tree: &RbxTree, file_name: &str) {
    let data_model = tree.get_instance(tree.get_root_id()).unwrap();
    let children_ids = data_model.get_children_ids();

    let file = BufWriter::new(File::create(format!("{}.rbxmx", file_name)).expect("couldn't create file"));
    rbx_xml::to_writer_default(file, &tree, children_ids).expect("couldn't write file");
}

// handle binary writing
fn handle_binary(tree: &RbxTree, file_name: &str) {
    let data_model = tree.get_instance(tree.get_root_id()).unwrap();
    let children_ids = data_model.get_children_ids();

    let file = BufWriter::new(File::create(format!("{}.rbxm", file_name)).expect("couldn't create file"));
    rbx_binary::encode(&tree, children_ids, file).expect("couldn't encode file");
}

pub fn generate(value: Value,  file_type: FileType, file_name: &str) {
    println!("generating value...");

    if value == Value::BoolValue {
        let tree = bool_value_tree();

        match file_type {
            FileType::Binary => handle_binary(&tree, file_name),
            FileType::XML => handle_xml(&tree, file_name)
        };
    };

    if value == Value::StringValue {
        let tree = string_value_tree();

        match file_type {
            FileType::Binary => handle_binary(&tree, file_name),
            FileType::XML => handle_xml(&tree, file_name)
        };
    }

    if value == Value::NumberValue {
        let tree = number_value_tree();
        
        match file_type {
            FileType::Binary => handle_binary(&tree, file_name),
            FileType::XML => handle_xml(&tree, file_name)
        };
    };

    if value == Value::IntValue {
        let tree = int_value_tree();

        match file_type {
            FileType::Binary => handle_binary(&tree, file_name),
            FileType::XML => handle_xml(&tree, file_name)
        }
    }

    println!("created new {0:?} file named {2} with a {1:?}", file_type, value, file_name)
}