use serde_json::Value;
use std::fs;

// This tool is used to create the opcode json files. The files can be turn into rust files that have a const in them for the
// json that will be serialized into hashmaps.

// TODO: add a readme for this.

fn main() {
    let json = fs::read_to_string("opcode.json").expect("Unable to read file");
    let value: Value = serde_json::from_str(&json).expect("Couldn't read value");

    let mut unprefixed_list: Vec<String> = Vec::new();
    for (i, value) in value["Unprefixed"].as_array().unwrap().iter().enumerate() {
        let name = value["Name"].as_str().unwrap().to_string();
        let tcycles = (
            value["TCyclesNoBranch"].as_u64().unwrap() as u8,
            value["TCyclesBranch"].as_u64().unwrap() as u8,
        );
        let length = value["Length"].as_u64().unwrap() as u8;
        unprefixed_list.push(format!(
            "OpCode::new({:#04X}, \"{}\", {:?}, {}),",
            i, name, tcycles, length
        ));
    }
    let unprefixed_string = unprefixed_list.join("\n");
    fs::write("unprefixed.txt", unprefixed_string).expect("Unable to write to file");

    let mut cb_prefixed_list: Vec<String> = Vec::new();
    for (i, value) in value["CBPrefixed"].as_array().unwrap().iter().enumerate() {
        let name = value["Name"].as_str().unwrap().to_string();
        let tcycles = (
            value["TCyclesNoBranch"].as_u64().unwrap() as u8,
            value["TCyclesBranch"].as_u64().unwrap() as u8,
        );
        let length = value["Length"].as_u64().unwrap() as u8;
        cb_prefixed_list.push(format!(
            "OpCode::new({:#04X}, \"{}\", {:?}, {}),",
            i, name, tcycles, length
        ));
    }
    let cb_prefixed_string = cb_prefixed_list.join("\n");
    fs::write("cb_prefixed.txt", cb_prefixed_string).expect("Unable to write to file");
}
