use rbx_dom_weak::{
    RbxInstanceProperties,
    RbxTree,
    RbxValue
};

use maplit::hashmap;


fn data_model_tree() -> RbxTree {
    let root_properties = RbxInstanceProperties {
        name: "root_tree".to_owned(),
        class_name: "DataModel".to_owned(),
        properties: hashmap! {}
    };

    return RbxTree::new(root_properties);
}

pub fn bool_value_tree() -> RbxTree {
    let mut root_tree = data_model_tree();

    let bool_value_properties = RbxInstanceProperties {
        name: "BoolValue".to_owned(),
        class_name: "BoolValue".to_owned(),
        properties: hashmap! {
            "Value".to_owned() => RbxValue::Bool {
                value: true
            }
        }
    };

    let root_id = root_tree.get_root_id();
    let _bool_value_id = root_tree.insert_instance(bool_value_properties, root_id);

    return root_tree;
}

pub fn string_value_tree() -> RbxTree {
    let mut root_tree = data_model_tree();

    let string_value_properties = RbxInstanceProperties {
        name: "StringValue".to_owned(),
        class_name: "StringValue".to_owned(),
        properties: hashmap! {
            "Value".to_owned() => RbxValue::String {
                value: "Hello world!".to_owned()
            }
        }
    };

    let root_id = root_tree.get_root_id();
    let _string_value_id = root_tree.insert_instance(string_value_properties, root_id);

    return root_tree;
}

pub fn number_value_tree() -> RbxTree {
    let mut root_tree = data_model_tree();

    let number_value_properties = RbxInstanceProperties {
        name: "NumberValue".to_owned(),
        class_name: "NumberValue".to_owned(),
        properties: hashmap! {
            "Value".to_owned() => RbxValue::Float64 {
                value: 42824.0
            }
        }
    };

    let root_id = root_tree.get_root_id();
    let _number_value_id = root_tree.insert_instance(number_value_properties, root_id);

    return root_tree;
}

pub fn int_value_tree() -> RbxTree {
    let mut root_tree = data_model_tree();

    let int_value_properties = RbxInstanceProperties {
        name: "IntValue".to_owned(),
        class_name: "IntValue".to_owned(),
        properties: hashmap! {
            "Value".to_owned() => RbxValue::Int64 {
                value: 74
            }
        }
    };

    let root_id = root_tree.get_root_id();
    let _int_value_id = root_tree.insert_instance(int_value_properties, root_id);

    return root_tree;
}