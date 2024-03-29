use cpu::opcode::{FlagAction, OpCode};
use serde_json::Value;
use std::{
    collections::{BTreeMap, HashMap},
    fs,
};

// This tool is used to create the opcode json files. The files can be turn into rust files that have a const in them for the
// json that will be serialized into hashmaps.

// TODO: add a readme for this.

fn main() {
    let json = fs::read_to_string("opcode.json").expect("Unable to read file");
    let value: Value = serde_json::from_str(&json).expect("Couldn't read value");

    let mut unprefixed_map: BTreeMap<u8, OpCode> = BTreeMap::new();
    for (i, value) in value["Unprefixed"].as_array().unwrap().iter().enumerate() {
        let name = value["Name"].as_str().unwrap().to_string();
        let tcycles = (
            value["TCyclesNoBranch"].as_u64().unwrap() as u8,
            value["TCyclesBranch"].as_u64().unwrap() as u8,
        );
        let mcycles = (tcycles.0 / 4, tcycles.0 / 4);
        let length = value["Length"].as_u64().unwrap() as u8;
        let mut flags_to_actions: HashMap<u8, FlagAction> = HashMap::new();
        flags_to_actions.insert(
            0b1000_0000,
            get_flag_action(value["Flags"]["Z"].as_str().unwrap()),
        );
        flags_to_actions.insert(
            0b0100_0000,
            get_flag_action(value["Flags"]["N"].as_str().unwrap()),
        );
        flags_to_actions.insert(
            0b0010_0000,
            get_flag_action(value["Flags"]["H"].as_str().unwrap()),
        );
        flags_to_actions.insert(
            0b0001_0000,
            get_flag_action(value["Flags"]["C"].as_str().unwrap()),
        );
        let opcode = OpCode::new(i as u8, name, tcycles, mcycles, length, flags_to_actions);
        unprefixed_map.insert(i as u8, opcode);
    }
    let unprefixed_string = serde_json::to_string(&unprefixed_map).unwrap();
    fs::write("unprefixed.json", unprefixed_string).expect("Unable to write to file");

    let mut cb_prefixed_map: BTreeMap<u8, OpCode> = BTreeMap::new();
    for (i, value) in value["CBPrefixed"].as_array().unwrap().iter().enumerate() {
        let name = value["Name"].as_str().unwrap().to_string();
        let tcycles = (
            value["TCyclesNoBranch"].as_u64().unwrap() as u8,
            value["TCyclesBranch"].as_u64().unwrap() as u8,
        );
        let mcycles = (tcycles.0 / 4, tcycles.0 / 4);
        let length = value["Length"].as_u64().unwrap() as u8;
        let mut flags_to_actions: HashMap<u8, FlagAction> = HashMap::new();
        flags_to_actions.insert(
            0b1000_0000,
            get_flag_action(value["Flags"]["Z"].as_str().unwrap()),
        );
        flags_to_actions.insert(
            0b0100_0000,
            get_flag_action(value["Flags"]["N"].as_str().unwrap()),
        );
        flags_to_actions.insert(
            0b0010_0000,
            get_flag_action(value["Flags"]["H"].as_str().unwrap()),
        );
        flags_to_actions.insert(
            0b0001_0000,
            get_flag_action(value["Flags"]["C"].as_str().unwrap()),
        );
        let opcode = OpCode::new(i as u8, name, tcycles, mcycles, length, flags_to_actions);
        cb_prefixed_map.insert(i as u8, opcode);
    }
    let cb_prefixed_string = serde_json::to_string(&cb_prefixed_map).unwrap();
    fs::write("cb_prefixed.json", cb_prefixed_string).expect("Unable to write to file");
}

fn get_flag_action(value: &str) -> FlagAction {
    match value {
        value if "ZNHC".contains(value) => FlagAction::Depend,
        "-" => FlagAction::Ignore,
        "0" => FlagAction::Unset,
        "1" => FlagAction::Set,
        _ => panic!(),
    }
}
