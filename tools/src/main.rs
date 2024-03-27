use serde_json::Value;
use std::{collections::BTreeMap, fs};

fn main() {
    // let mut unprefixed_map: BTreeMap<u8, Opcode> = BTreeMap::new();
    // let json = fs::read_to_string("opcode.json").expect("Unable to read file");
    // let value: Value = serde_json::from_str(&json).expect("couldn't read value");
    // for (i, value) in value["Unprefixed"].as_array().unwrap().iter().enumerate() {
    //     let opcode = Opcode::new(
    //         i as u8,
    //         value["Name"].as_str().unwrap().to_string(),
    //         (
    //             value["TCyclesNoBranch"].as_u64().unwrap() as u8,
    //             value["TCyclesBranch"].as_u64().unwrap() as u8,
    //         ),
    //         (
    //             (value["TCyclesNoBranch"].as_u64().unwrap() as u8) / 4,
    //             (value["TCyclesBranch"].as_u64().unwrap() as u8) / 4,
    //         ),
    //         value["Length"].as_u64().unwrap() as u8,
    //         Flags {
    //             zero: get_flag(value["Flags"]["Z"].as_str().unwrap()),
    //             subtraction: get_flag(value["Flags"]["N"].as_str().unwrap()),
    //             half_carry: get_flag(value["Flags"]["H"].as_str().unwrap()),
    //             carry: get_flag(value["Flags"]["C"].as_str().unwrap()),
    //         },
    //     );

    //     unprefixed_map.insert(i as u8, opcode);
    // }
    // let unprefixed_string = serde_json::to_string(&unprefixed_map).unwrap();
    // fs::write("unprefixed.json", unprefixed_string).expect("Unable to write to file");

    // let mut cb_prefixed_map: BTreeMap<u8, Opcode> = BTreeMap::new();
    // for (i, value) in value["CBPrefixed"].as_array().unwrap().iter().enumerate() {
    //     let opcode = Opcode {
    //         value: i as u8,
    //         name: value["Name"].as_str().unwrap().to_string(),
    //         tcycles: (
    //             value["TCyclesNoBranch"].as_u64().unwrap() as u8,
    //             value["TCyclesBranch"].as_u64().unwrap() as u8,
    //         ),
    //         mcycles: (
    //             (value["TCyclesNoBranch"].as_u64().unwrap() as u8) / 4,
    //             (value["TCyclesBranch"].as_u64().unwrap() as u8) / 4,
    //         ),
    //         length: value["Length"].as_u64().unwrap() as u8,
    //         flags: Flags {
    //             zero: get_flag(value["Flags"]["Z"].as_str().unwrap()),
    //             subtraction: get_flag(value["Flags"]["N"].as_str().unwrap()),
    //             half_carry: get_flag(value["Flags"]["H"].as_str().unwrap()),
    //             carry: get_flag(value["Flags"]["C"].as_str().unwrap()),
    //         },
    //     };
    //     cb_prefixed_map.insert(i as u8, opcode);
    // }

    // let cb_prefixed_string = serde_json::to_string(&cb_prefixed_map).unwrap();
    // fs::write("cb_prefixed.json", cb_prefixed_string).expect("Unable to write to file");
}

// fn get_flag(value: &str) -> Flag {
//     match value {
//         value if "ZNHC".contains(value) => Flag::Dependent,
//         "-" => Flag::Independent,
//         "0" => Flag::Unset,
//         "1" => Flag::Set,
//         _ => panic!(),
//     }
// }
