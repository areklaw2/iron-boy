pub const CB_PREFIXED: &str = r#"{
  "0": {
    "value": 0,
    "name": "RLC B",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Depend",
      "64": "Unset",
      "32": "Unset",
      "128": "Depend"
    }
  },
  "1": {
    "value": 1,
    "name": "RLC C",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Depend",
      "16": "Depend",
      "32": "Unset",
      "64": "Unset"
    }
  },
  "2": {
    "value": 2,
    "name": "RLC D",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Unset",
      "16": "Depend",
      "128": "Depend",
      "32": "Unset"
    }
  },
  "3": {
    "value": 3,
    "name": "RLC E",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Depend",
      "64": "Unset",
      "32": "Unset",
      "16": "Depend"
    }
  },
  "4": {
    "value": 4,
    "name": "RLC H",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Unset",
      "128": "Depend",
      "64": "Unset",
      "16": "Depend"
    }
  },
  "5": {
    "value": 5,
    "name": "RLC L",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Depend",
      "32": "Unset",
      "16": "Depend",
      "64": "Unset"
    }
  },
  "6": {
    "value": 6,
    "name": "RLC (HL)",
    "tcycles": [16, 16],
    "mcycles": [4, 4],
    "length": 2,
    "flags_to_action": {
      "64": "Unset",
      "128": "Depend",
      "16": "Depend",
      "32": "Unset"
    }
  },
  "7": {
    "value": 7,
    "name": "RLC A",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Depend",
      "128": "Depend",
      "64": "Unset",
      "32": "Unset"
    }
  },
  "8": {
    "value": 8,
    "name": "RRC B",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Unset",
      "128": "Depend",
      "32": "Unset",
      "16": "Depend"
    }
  },
  "9": {
    "value": 9,
    "name": "RRC C",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Depend",
      "64": "Unset",
      "16": "Depend",
      "32": "Unset"
    }
  },
  "10": {
    "value": 10,
    "name": "RRC D",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Unset",
      "32": "Unset",
      "16": "Depend",
      "128": "Depend"
    }
  },
  "11": {
    "value": 11,
    "name": "RRC E",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Unset",
      "64": "Unset",
      "16": "Depend",
      "128": "Depend"
    }
  },
  "12": {
    "value": 12,
    "name": "RRC H",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Unset",
      "128": "Depend",
      "16": "Depend",
      "64": "Unset"
    }
  },
  "13": {
    "value": 13,
    "name": "RRC L",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Depend",
      "32": "Unset",
      "16": "Depend",
      "64": "Unset"
    }
  },
  "14": {
    "value": 14,
    "name": "RRC (HL)",
    "tcycles": [16, 16],
    "mcycles": [4, 4],
    "length": 2,
    "flags_to_action": {
      "128": "Depend",
      "64": "Unset",
      "32": "Unset",
      "16": "Depend"
    }
  },
  "15": {
    "value": 15,
    "name": "RRC A",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Depend",
      "64": "Unset",
      "16": "Depend",
      "32": "Unset"
    }
  },
  "16": {
    "value": 16,
    "name": "RL B",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Unset",
      "128": "Depend",
      "64": "Unset",
      "16": "Depend"
    }
  },
  "17": {
    "value": 17,
    "name": "RL C",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Unset",
      "16": "Depend",
      "64": "Unset",
      "128": "Depend"
    }
  },
  "18": {
    "value": 18,
    "name": "RL D",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Unset",
      "64": "Unset",
      "16": "Depend",
      "128": "Depend"
    }
  },
  "19": {
    "value": 19,
    "name": "RL E",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Unset",
      "32": "Unset",
      "16": "Depend",
      "128": "Depend"
    }
  },
  "20": {
    "value": 20,
    "name": "RL H",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Unset",
      "128": "Depend",
      "16": "Depend",
      "32": "Unset"
    }
  },
  "21": {
    "value": 21,
    "name": "RL L",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Unset",
      "128": "Depend",
      "64": "Unset",
      "16": "Depend"
    }
  },
  "22": {
    "value": 22,
    "name": "RL (HL)",
    "tcycles": [16, 16],
    "mcycles": [4, 4],
    "length": 2,
    "flags_to_action": {
      "32": "Unset",
      "64": "Unset",
      "16": "Depend",
      "128": "Depend"
    }
  },
  "23": {
    "value": 23,
    "name": "RL A",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Depend",
      "64": "Unset",
      "16": "Depend",
      "32": "Unset"
    }
  },
  "24": {
    "value": 24,
    "name": "RR B",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Depend",
      "128": "Depend",
      "32": "Unset",
      "64": "Unset"
    }
  },
  "25": {
    "value": 25,
    "name": "RR C",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Unset",
      "128": "Depend",
      "16": "Depend",
      "32": "Unset"
    }
  },
  "26": {
    "value": 26,
    "name": "RR D",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Unset",
      "64": "Unset",
      "16": "Depend",
      "128": "Depend"
    }
  },
  "27": {
    "value": 27,
    "name": "RR E",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Unset",
      "64": "Unset",
      "16": "Depend",
      "128": "Depend"
    }
  },
  "28": {
    "value": 28,
    "name": "RR H",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Unset",
      "64": "Unset",
      "16": "Depend",
      "128": "Depend"
    }
  },
  "29": {
    "value": 29,
    "name": "RR L",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Depend",
      "128": "Depend",
      "64": "Unset",
      "32": "Unset"
    }
  },
  "30": {
    "value": 30,
    "name": "RR (HL)",
    "tcycles": [16, 16],
    "mcycles": [4, 4],
    "length": 2,
    "flags_to_action": {
      "16": "Depend",
      "128": "Depend",
      "64": "Unset",
      "32": "Unset"
    }
  },
  "31": {
    "value": 31,
    "name": "RR A",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Unset",
      "16": "Depend",
      "128": "Depend",
      "64": "Unset"
    }
  },
  "32": {
    "value": 32,
    "name": "SLA B",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Unset",
      "16": "Depend",
      "128": "Depend",
      "64": "Unset"
    }
  },
  "33": {
    "value": 33,
    "name": "SLA C",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Depend",
      "128": "Depend",
      "32": "Unset",
      "64": "Unset"
    }
  },
  "34": {
    "value": 34,
    "name": "SLA D",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Unset",
      "16": "Depend",
      "128": "Depend",
      "32": "Unset"
    }
  },
  "35": {
    "value": 35,
    "name": "SLA E",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Depend",
      "128": "Depend",
      "64": "Unset",
      "32": "Unset"
    }
  },
  "36": {
    "value": 36,
    "name": "SLA H",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Depend",
      "32": "Unset",
      "64": "Unset",
      "128": "Depend"
    }
  },
  "37": {
    "value": 37,
    "name": "SLA L",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Unset",
      "32": "Unset",
      "128": "Depend",
      "16": "Depend"
    }
  },
  "38": {
    "value": 38,
    "name": "SLA (HL)",
    "tcycles": [16, 16],
    "mcycles": [4, 4],
    "length": 2,
    "flags_to_action": {
      "64": "Unset",
      "32": "Unset",
      "16": "Depend",
      "128": "Depend"
    }
  },
  "39": {
    "value": 39,
    "name": "SLA A",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Depend",
      "32": "Unset",
      "128": "Depend",
      "64": "Unset"
    }
  },
  "40": {
    "value": 40,
    "name": "SRA B",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Depend",
      "128": "Depend",
      "64": "Unset",
      "32": "Unset"
    }
  },
  "41": {
    "value": 41,
    "name": "SRA C",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Depend",
      "64": "Unset",
      "32": "Unset",
      "16": "Depend"
    }
  },
  "42": {
    "value": 42,
    "name": "SRA D",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Unset",
      "32": "Unset",
      "128": "Depend",
      "16": "Depend"
    }
  },
  "43": {
    "value": 43,
    "name": "SRA E",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Unset",
      "16": "Depend",
      "64": "Unset",
      "128": "Depend"
    }
  },
  "44": {
    "value": 44,
    "name": "SRA H",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Unset",
      "32": "Unset",
      "128": "Depend",
      "16": "Depend"
    }
  },
  "45": {
    "value": 45,
    "name": "SRA L",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Unset",
      "32": "Unset",
      "128": "Depend",
      "16": "Depend"
    }
  },
  "46": {
    "value": 46,
    "name": "SRA (HL)",
    "tcycles": [16, 16],
    "mcycles": [4, 4],
    "length": 2,
    "flags_to_action": {
      "32": "Unset",
      "128": "Depend",
      "64": "Unset",
      "16": "Depend"
    }
  },
  "47": {
    "value": 47,
    "name": "SRA A",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Unset",
      "128": "Depend",
      "32": "Unset",
      "16": "Depend"
    }
  },
  "48": {
    "value": 48,
    "name": "SWAP B",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Unset",
      "64": "Unset",
      "32": "Unset",
      "128": "Depend"
    }
  },
  "49": {
    "value": 49,
    "name": "SWAP C",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Unset",
      "16": "Unset",
      "128": "Depend",
      "32": "Unset"
    }
  },
  "50": {
    "value": 50,
    "name": "SWAP D",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Unset",
      "64": "Unset",
      "32": "Unset",
      "128": "Depend"
    }
  },
  "51": {
    "value": 51,
    "name": "SWAP E",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Depend",
      "64": "Unset",
      "32": "Unset",
      "16": "Unset"
    }
  },
  "52": {
    "value": 52,
    "name": "SWAP H",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Depend",
      "64": "Unset",
      "16": "Unset",
      "32": "Unset"
    }
  },
  "53": {
    "value": 53,
    "name": "SWAP L",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Unset",
      "64": "Unset",
      "128": "Depend",
      "32": "Unset"
    }
  },
  "54": {
    "value": 54,
    "name": "SWAP (HL)",
    "tcycles": [16, 16],
    "mcycles": [4, 4],
    "length": 2,
    "flags_to_action": {
      "128": "Depend",
      "64": "Unset",
      "32": "Unset",
      "16": "Unset"
    }
  },
  "55": {
    "value": 55,
    "name": "SWAP A",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Unset",
      "128": "Depend",
      "16": "Unset",
      "64": "Unset"
    }
  },
  "56": {
    "value": 56,
    "name": "SRL B",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Unset",
      "32": "Unset",
      "128": "Depend",
      "16": "Depend"
    }
  },
  "57": {
    "value": 57,
    "name": "SRL C",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Depend",
      "64": "Unset",
      "32": "Unset",
      "16": "Depend"
    }
  },
  "58": {
    "value": 58,
    "name": "SRL D",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Depend",
      "32": "Unset",
      "16": "Depend",
      "64": "Unset"
    }
  },
  "59": {
    "value": 59,
    "name": "SRL E",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Depend",
      "16": "Depend",
      "32": "Unset",
      "64": "Unset"
    }
  },
  "60": {
    "value": 60,
    "name": "SRL H",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Unset",
      "128": "Depend",
      "64": "Unset",
      "16": "Depend"
    }
  },
  "61": {
    "value": 61,
    "name": "SRL L",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Unset",
      "32": "Unset",
      "128": "Depend",
      "16": "Depend"
    }
  },
  "62": {
    "value": 62,
    "name": "SRL (HL)",
    "tcycles": [16, 16],
    "mcycles": [4, 4],
    "length": 2,
    "flags_to_action": {
      "32": "Unset",
      "64": "Unset",
      "128": "Depend",
      "16": "Depend"
    }
  },
  "63": {
    "value": 63,
    "name": "SRL A",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Unset",
      "128": "Depend",
      "32": "Unset",
      "16": "Depend"
    }
  },
  "64": {
    "value": 64,
    "name": "BIT 0,B",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Set",
      "64": "Unset",
      "16": "Ignore",
      "128": "Depend"
    }
  },
  "65": {
    "value": 65,
    "name": "BIT 0,C",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "128": "Depend",
      "64": "Unset",
      "32": "Set"
    }
  },
  "66": {
    "value": 66,
    "name": "BIT 0,D",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Depend",
      "64": "Unset",
      "32": "Set",
      "16": "Ignore"
    }
  },
  "67": {
    "value": 67,
    "name": "BIT 0,E",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Unset",
      "128": "Depend",
      "32": "Set",
      "16": "Ignore"
    }
  },
  "68": {
    "value": 68,
    "name": "BIT 0,H",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Depend",
      "16": "Ignore",
      "32": "Set",
      "64": "Unset"
    }
  },
  "69": {
    "value": 69,
    "name": "BIT 0,L",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Unset",
      "16": "Ignore",
      "128": "Depend",
      "32": "Set"
    }
  },
  "70": {
    "value": 70,
    "name": "BIT 0,(HL)",
    "tcycles": [12, 12],
    "mcycles": [3, 3],
    "length": 2,
    "flags_to_action": {
      "128": "Depend",
      "16": "Ignore",
      "32": "Set",
      "64": "Unset"
    }
  },
  "71": {
    "value": 71,
    "name": "BIT 0,A",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "64": "Unset",
      "128": "Depend",
      "32": "Set"
    }
  },
  "72": {
    "value": 72,
    "name": "BIT 1,B",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Depend",
      "16": "Ignore",
      "32": "Set",
      "64": "Unset"
    }
  },
  "73": {
    "value": 73,
    "name": "BIT 1,C",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Depend",
      "64": "Unset",
      "16": "Ignore",
      "32": "Set"
    }
  },
  "74": {
    "value": 74,
    "name": "BIT 1,D",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "64": "Unset",
      "128": "Depend",
      "32": "Set"
    }
  },
  "75": {
    "value": 75,
    "name": "BIT 1,E",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Unset",
      "128": "Depend",
      "32": "Set",
      "16": "Ignore"
    }
  },
  "76": {
    "value": 76,
    "name": "BIT 1,H",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Depend",
      "64": "Unset",
      "16": "Ignore",
      "32": "Set"
    }
  },
  "77": {
    "value": 77,
    "name": "BIT 1,L",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Unset",
      "32": "Set",
      "16": "Ignore",
      "128": "Depend"
    }
  },
  "78": {
    "value": 78,
    "name": "BIT 1,(HL)",
    "tcycles": [12, 12],
    "mcycles": [3, 3],
    "length": 2,
    "flags_to_action": {
      "128": "Depend",
      "32": "Set",
      "16": "Ignore",
      "64": "Unset"
    }
  },
  "79": {
    "value": 79,
    "name": "BIT 1,A",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Unset",
      "128": "Depend",
      "32": "Set",
      "16": "Ignore"
    }
  },
  "80": {
    "value": 80,
    "name": "BIT 2,B",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "128": "Depend",
      "64": "Unset",
      "32": "Set"
    }
  },
  "81": {
    "value": 81,
    "name": "BIT 2,C",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Depend",
      "16": "Ignore",
      "32": "Set",
      "64": "Unset"
    }
  },
  "82": {
    "value": 82,
    "name": "BIT 2,D",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "128": "Depend",
      "64": "Unset",
      "32": "Set"
    }
  },
  "83": {
    "value": 83,
    "name": "BIT 2,E",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Set",
      "128": "Depend",
      "64": "Unset",
      "16": "Ignore"
    }
  },
  "84": {
    "value": 84,
    "name": "BIT 2,H",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Set",
      "64": "Unset",
      "16": "Ignore",
      "128": "Depend"
    }
  },
  "85": {
    "value": 85,
    "name": "BIT 2,L",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Set",
      "128": "Depend",
      "64": "Unset",
      "16": "Ignore"
    }
  },
  "86": {
    "value": 86,
    "name": "BIT 2,(HL)",
    "tcycles": [12, 12],
    "mcycles": [3, 3],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "128": "Depend",
      "32": "Set",
      "64": "Unset"
    }
  },
  "87": {
    "value": 87,
    "name": "BIT 2,A",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Set",
      "64": "Unset",
      "128": "Depend",
      "16": "Ignore"
    }
  },
  "88": {
    "value": 88,
    "name": "BIT 3,B",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Set",
      "128": "Depend",
      "16": "Ignore",
      "64": "Unset"
    }
  },
  "89": {
    "value": 89,
    "name": "BIT 3,C",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Depend",
      "64": "Unset",
      "16": "Ignore",
      "32": "Set"
    }
  },
  "90": {
    "value": 90,
    "name": "BIT 3,D",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Unset",
      "128": "Depend",
      "16": "Ignore",
      "32": "Set"
    }
  },
  "91": {
    "value": 91,
    "name": "BIT 3,E",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Unset",
      "128": "Depend",
      "16": "Ignore",
      "32": "Set"
    }
  },
  "92": {
    "value": 92,
    "name": "BIT 3,H",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Set",
      "128": "Depend",
      "16": "Ignore",
      "64": "Unset"
    }
  },
  "93": {
    "value": 93,
    "name": "BIT 3,L",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Depend",
      "64": "Unset",
      "16": "Ignore",
      "32": "Set"
    }
  },
  "94": {
    "value": 94,
    "name": "BIT 3,(HL)",
    "tcycles": [12, 12],
    "mcycles": [3, 3],
    "length": 2,
    "flags_to_action": {
      "32": "Set",
      "128": "Depend",
      "64": "Unset",
      "16": "Ignore"
    }
  },
  "95": {
    "value": 95,
    "name": "BIT 3,A",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "128": "Depend",
      "32": "Set",
      "64": "Unset"
    }
  },
  "96": {
    "value": 96,
    "name": "BIT 4,B",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Depend",
      "64": "Unset",
      "16": "Ignore",
      "32": "Set"
    }
  },
  "97": {
    "value": 97,
    "name": "BIT 4,C",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Depend",
      "64": "Unset",
      "16": "Ignore",
      "32": "Set"
    }
  },
  "98": {
    "value": 98,
    "name": "BIT 4,D",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Depend",
      "32": "Set",
      "64": "Unset",
      "16": "Ignore"
    }
  },
  "99": {
    "value": 99,
    "name": "BIT 4,E",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "32": "Set",
      "128": "Depend",
      "64": "Unset"
    }
  },
  "100": {
    "value": 100,
    "name": "BIT 4,H",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Unset",
      "128": "Depend",
      "32": "Set",
      "16": "Ignore"
    }
  },
  "101": {
    "value": 101,
    "name": "BIT 4,L",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Unset",
      "16": "Ignore",
      "128": "Depend",
      "32": "Set"
    }
  },
  "102": {
    "value": 102,
    "name": "BIT 4,(HL)",
    "tcycles": [12, 12],
    "mcycles": [3, 3],
    "length": 2,
    "flags_to_action": {
      "128": "Depend",
      "16": "Ignore",
      "64": "Unset",
      "32": "Set"
    }
  },
  "103": {
    "value": 103,
    "name": "BIT 4,A",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "32": "Set",
      "64": "Unset",
      "128": "Depend"
    }
  },
  "104": {
    "value": 104,
    "name": "BIT 5,B",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Set",
      "128": "Depend",
      "16": "Ignore",
      "64": "Unset"
    }
  },
  "105": {
    "value": 105,
    "name": "BIT 5,C",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "64": "Unset",
      "32": "Set",
      "128": "Depend"
    }
  },
  "106": {
    "value": 106,
    "name": "BIT 5,D",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Depend",
      "32": "Set",
      "64": "Unset",
      "16": "Ignore"
    }
  },
  "107": {
    "value": 107,
    "name": "BIT 5,E",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Set",
      "128": "Depend",
      "16": "Ignore",
      "64": "Unset"
    }
  },
  "108": {
    "value": 108,
    "name": "BIT 5,H",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Depend",
      "32": "Set",
      "64": "Unset",
      "16": "Ignore"
    }
  },
  "109": {
    "value": 109,
    "name": "BIT 5,L",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "64": "Unset",
      "32": "Set",
      "128": "Depend"
    }
  },
  "110": {
    "value": 110,
    "name": "BIT 5,(HL)",
    "tcycles": [12, 12],
    "mcycles": [3, 3],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "64": "Unset",
      "128": "Depend",
      "32": "Set"
    }
  },
  "111": {
    "value": 111,
    "name": "BIT 5,A",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Unset",
      "128": "Depend",
      "32": "Set",
      "16": "Ignore"
    }
  },
  "112": {
    "value": 112,
    "name": "BIT 6,B",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Set",
      "64": "Unset",
      "128": "Depend",
      "16": "Ignore"
    }
  },
  "113": {
    "value": 113,
    "name": "BIT 6,C",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Depend",
      "64": "Unset",
      "32": "Set",
      "16": "Ignore"
    }
  },
  "114": {
    "value": 114,
    "name": "BIT 6,D",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Depend",
      "32": "Set",
      "16": "Ignore",
      "64": "Unset"
    }
  },
  "115": {
    "value": 115,
    "name": "BIT 6,E",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Unset",
      "128": "Depend",
      "16": "Ignore",
      "32": "Set"
    }
  },
  "116": {
    "value": 116,
    "name": "BIT 6,H",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Set",
      "128": "Depend",
      "64": "Unset",
      "16": "Ignore"
    }
  },
  "117": {
    "value": 117,
    "name": "BIT 6,L",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Depend",
      "64": "Unset",
      "32": "Set",
      "16": "Ignore"
    }
  },
  "118": {
    "value": 118,
    "name": "BIT 6,(HL)",
    "tcycles": [12, 12],
    "mcycles": [3, 3],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "32": "Set",
      "64": "Unset",
      "128": "Depend"
    }
  },
  "119": {
    "value": 119,
    "name": "BIT 6,A",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Depend",
      "32": "Set",
      "64": "Unset",
      "16": "Ignore"
    }
  },
  "120": {
    "value": 120,
    "name": "BIT 7,B",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Set",
      "128": "Depend",
      "64": "Unset",
      "16": "Ignore"
    }
  },
  "121": {
    "value": 121,
    "name": "BIT 7,C",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "32": "Set",
      "128": "Depend",
      "64": "Unset"
    }
  },
  "122": {
    "value": 122,
    "name": "BIT 7,D",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Unset",
      "128": "Depend",
      "16": "Ignore",
      "32": "Set"
    }
  },
  "123": {
    "value": 123,
    "name": "BIT 7,E",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Set",
      "64": "Unset",
      "16": "Ignore",
      "128": "Depend"
    }
  },
  "124": {
    "value": 124,
    "name": "BIT 7,H",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Unset",
      "128": "Depend",
      "32": "Set",
      "16": "Ignore"
    }
  },
  "125": {
    "value": 125,
    "name": "BIT 7,L",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Depend",
      "64": "Unset",
      "32": "Set",
      "16": "Ignore"
    }
  },
  "126": {
    "value": 126,
    "name": "BIT 7,(HL)",
    "tcycles": [12, 12],
    "mcycles": [3, 3],
    "length": 2,
    "flags_to_action": {
      "128": "Depend",
      "64": "Unset",
      "16": "Ignore",
      "32": "Set"
    }
  },
  "127": {
    "value": 127,
    "name": "BIT 7,A",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "32": "Set",
      "64": "Unset",
      "128": "Depend"
    }
  },
  "128": {
    "value": 128,
    "name": "RES 0,B",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Ignore",
      "64": "Ignore",
      "16": "Ignore",
      "32": "Ignore"
    }
  },
  "129": {
    "value": 129,
    "name": "RES 0,C",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "128": "Ignore",
      "32": "Ignore",
      "64": "Ignore"
    }
  },
  "130": {
    "value": 130,
    "name": "RES 0,D",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Ignore",
      "16": "Ignore",
      "64": "Ignore",
      "32": "Ignore"
    }
  },
  "131": {
    "value": 131,
    "name": "RES 0,E",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Ignore",
      "32": "Ignore",
      "16": "Ignore",
      "128": "Ignore"
    }
  },
  "132": {
    "value": 132,
    "name": "RES 0,H",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Ignore",
      "16": "Ignore",
      "64": "Ignore",
      "128": "Ignore"
    }
  },
  "133": {
    "value": 133,
    "name": "RES 0,L",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "64": "Ignore",
      "128": "Ignore",
      "32": "Ignore"
    }
  },
  "134": {
    "value": 134,
    "name": "RES 0,(HL)",
    "tcycles": [16, 16],
    "mcycles": [4, 4],
    "length": 2,
    "flags_to_action": {
      "64": "Ignore",
      "32": "Ignore",
      "16": "Ignore",
      "128": "Ignore"
    }
  },
  "135": {
    "value": 135,
    "name": "RES 0,A",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "64": "Ignore",
      "128": "Ignore",
      "32": "Ignore"
    }
  },
  "136": {
    "value": 136,
    "name": "RES 1,B",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "64": "Ignore",
      "128": "Ignore",
      "32": "Ignore"
    }
  },
  "137": {
    "value": 137,
    "name": "RES 1,C",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Ignore",
      "32": "Ignore",
      "64": "Ignore",
      "16": "Ignore"
    }
  },
  "138": {
    "value": 138,
    "name": "RES 1,D",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Ignore",
      "32": "Ignore",
      "16": "Ignore",
      "64": "Ignore"
    }
  },
  "139": {
    "value": 139,
    "name": "RES 1,E",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Ignore",
      "32": "Ignore",
      "128": "Ignore",
      "16": "Ignore"
    }
  },
  "140": {
    "value": 140,
    "name": "RES 1,H",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Ignore",
      "32": "Ignore",
      "16": "Ignore",
      "64": "Ignore"
    }
  },
  "141": {
    "value": 141,
    "name": "RES 1,L",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Ignore",
      "32": "Ignore",
      "64": "Ignore",
      "16": "Ignore"
    }
  },
  "142": {
    "value": 142,
    "name": "RES 1,(HL)",
    "tcycles": [16, 16],
    "mcycles": [4, 4],
    "length": 2,
    "flags_to_action": {
      "32": "Ignore",
      "16": "Ignore",
      "64": "Ignore",
      "128": "Ignore"
    }
  },
  "143": {
    "value": 143,
    "name": "RES 1,A",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "128": "Ignore",
      "64": "Ignore",
      "32": "Ignore"
    }
  },
  "144": {
    "value": 144,
    "name": "RES 2,B",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "128": "Ignore",
      "32": "Ignore",
      "64": "Ignore"
    }
  },
  "145": {
    "value": 145,
    "name": "RES 2,C",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Ignore",
      "16": "Ignore",
      "128": "Ignore",
      "32": "Ignore"
    }
  },
  "146": {
    "value": 146,
    "name": "RES 2,D",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Ignore",
      "16": "Ignore",
      "128": "Ignore",
      "64": "Ignore"
    }
  },
  "147": {
    "value": 147,
    "name": "RES 2,E",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Ignore",
      "16": "Ignore",
      "32": "Ignore",
      "128": "Ignore"
    }
  },
  "148": {
    "value": 148,
    "name": "RES 2,H",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Ignore",
      "32": "Ignore",
      "16": "Ignore",
      "64": "Ignore"
    }
  },
  "149": {
    "value": 149,
    "name": "RES 2,L",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Ignore",
      "64": "Ignore",
      "128": "Ignore",
      "16": "Ignore"
    }
  },
  "150": {
    "value": 150,
    "name": "RES 2,(HL)",
    "tcycles": [16, 16],
    "mcycles": [4, 4],
    "length": 2,
    "flags_to_action": {
      "128": "Ignore",
      "32": "Ignore",
      "16": "Ignore",
      "64": "Ignore"
    }
  },
  "151": {
    "value": 151,
    "name": "RES 2,A",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Ignore",
      "16": "Ignore",
      "32": "Ignore",
      "64": "Ignore"
    }
  },
  "152": {
    "value": 152,
    "name": "RES 3,B",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Ignore",
      "64": "Ignore",
      "16": "Ignore",
      "128": "Ignore"
    }
  },
  "153": {
    "value": 153,
    "name": "RES 3,C",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Ignore",
      "64": "Ignore",
      "32": "Ignore",
      "16": "Ignore"
    }
  },
  "154": {
    "value": 154,
    "name": "RES 3,D",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Ignore",
      "64": "Ignore",
      "16": "Ignore",
      "128": "Ignore"
    }
  },
  "155": {
    "value": 155,
    "name": "RES 3,E",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "32": "Ignore",
      "64": "Ignore",
      "128": "Ignore"
    }
  },
  "156": {
    "value": 156,
    "name": "RES 3,H",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Ignore",
      "16": "Ignore",
      "32": "Ignore",
      "64": "Ignore"
    }
  },
  "157": {
    "value": 157,
    "name": "RES 3,L",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Ignore",
      "16": "Ignore",
      "64": "Ignore",
      "32": "Ignore"
    }
  },
  "158": {
    "value": 158,
    "name": "RES 3,(HL)",
    "tcycles": [16, 16],
    "mcycles": [4, 4],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "64": "Ignore",
      "128": "Ignore",
      "32": "Ignore"
    }
  },
  "159": {
    "value": 159,
    "name": "RES 3,A",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Ignore",
      "16": "Ignore",
      "128": "Ignore",
      "32": "Ignore"
    }
  },
  "160": {
    "value": 160,
    "name": "RES 4,B",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Ignore",
      "64": "Ignore",
      "32": "Ignore",
      "16": "Ignore"
    }
  },
  "161": {
    "value": 161,
    "name": "RES 4,C",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "64": "Ignore",
      "128": "Ignore",
      "32": "Ignore"
    }
  },
  "162": {
    "value": 162,
    "name": "RES 4,D",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Ignore",
      "64": "Ignore",
      "32": "Ignore",
      "16": "Ignore"
    }
  },
  "163": {
    "value": 163,
    "name": "RES 4,E",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Ignore",
      "16": "Ignore",
      "32": "Ignore",
      "64": "Ignore"
    }
  },
  "164": {
    "value": 164,
    "name": "RES 4,H",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Ignore",
      "128": "Ignore",
      "16": "Ignore",
      "32": "Ignore"
    }
  },
  "165": {
    "value": 165,
    "name": "RES 4,L",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "32": "Ignore",
      "128": "Ignore",
      "64": "Ignore"
    }
  },
  "166": {
    "value": 166,
    "name": "RES 4,(HL)",
    "tcycles": [16, 16],
    "mcycles": [4, 4],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "64": "Ignore",
      "128": "Ignore",
      "32": "Ignore"
    }
  },
  "167": {
    "value": 167,
    "name": "RES 4,A",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Ignore",
      "128": "Ignore",
      "64": "Ignore",
      "16": "Ignore"
    }
  },
  "168": {
    "value": 168,
    "name": "RES 5,B",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "128": "Ignore",
      "64": "Ignore",
      "32": "Ignore"
    }
  },
  "169": {
    "value": 169,
    "name": "RES 5,C",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Ignore",
      "64": "Ignore",
      "16": "Ignore",
      "32": "Ignore"
    }
  },
  "170": {
    "value": 170,
    "name": "RES 5,D",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Ignore",
      "16": "Ignore",
      "64": "Ignore",
      "32": "Ignore"
    }
  },
  "171": {
    "value": 171,
    "name": "RES 5,E",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Ignore",
      "16": "Ignore",
      "128": "Ignore",
      "32": "Ignore"
    }
  },
  "172": {
    "value": 172,
    "name": "RES 5,H",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Ignore",
      "32": "Ignore",
      "16": "Ignore",
      "128": "Ignore"
    }
  },
  "173": {
    "value": 173,
    "name": "RES 5,L",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Ignore",
      "64": "Ignore",
      "16": "Ignore",
      "128": "Ignore"
    }
  },
  "174": {
    "value": 174,
    "name": "RES 5,(HL)",
    "tcycles": [16, 16],
    "mcycles": [4, 4],
    "length": 2,
    "flags_to_action": {
      "128": "Ignore",
      "32": "Ignore",
      "64": "Ignore",
      "16": "Ignore"
    }
  },
  "175": {
    "value": 175,
    "name": "RES 5,A",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "64": "Ignore",
      "32": "Ignore",
      "128": "Ignore"
    }
  },
  "176": {
    "value": 176,
    "name": "RES 6,B",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Ignore",
      "16": "Ignore",
      "64": "Ignore",
      "128": "Ignore"
    }
  },
  "177": {
    "value": 177,
    "name": "RES 6,C",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Ignore",
      "128": "Ignore",
      "16": "Ignore",
      "64": "Ignore"
    }
  },
  "178": {
    "value": 178,
    "name": "RES 6,D",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Ignore",
      "16": "Ignore",
      "32": "Ignore",
      "64": "Ignore"
    }
  },
  "179": {
    "value": 179,
    "name": "RES 6,E",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Ignore",
      "32": "Ignore",
      "64": "Ignore",
      "16": "Ignore"
    }
  },
  "180": {
    "value": 180,
    "name": "RES 6,H",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Ignore",
      "32": "Ignore",
      "128": "Ignore",
      "16": "Ignore"
    }
  },
  "181": {
    "value": 181,
    "name": "RES 6,L",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Ignore",
      "32": "Ignore",
      "16": "Ignore",
      "64": "Ignore"
    }
  },
  "182": {
    "value": 182,
    "name": "RES 6,(HL)",
    "tcycles": [16, 16],
    "mcycles": [4, 4],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "128": "Ignore",
      "64": "Ignore",
      "32": "Ignore"
    }
  },
  "183": {
    "value": 183,
    "name": "RES 6,A",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "64": "Ignore",
      "32": "Ignore",
      "128": "Ignore"
    }
  },
  "184": {
    "value": 184,
    "name": "RES 7,B",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Ignore",
      "16": "Ignore",
      "64": "Ignore",
      "32": "Ignore"
    }
  },
  "185": {
    "value": 185,
    "name": "RES 7,C",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Ignore",
      "32": "Ignore",
      "128": "Ignore",
      "16": "Ignore"
    }
  },
  "186": {
    "value": 186,
    "name": "RES 7,D",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "32": "Ignore",
      "128": "Ignore",
      "64": "Ignore"
    }
  },
  "187": {
    "value": 187,
    "name": "RES 7,E",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Ignore",
      "32": "Ignore",
      "64": "Ignore",
      "16": "Ignore"
    }
  },
  "188": {
    "value": 188,
    "name": "RES 7,H",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "128": "Ignore",
      "64": "Ignore",
      "32": "Ignore"
    }
  },
  "189": {
    "value": 189,
    "name": "RES 7,L",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Ignore",
      "128": "Ignore",
      "16": "Ignore",
      "32": "Ignore"
    }
  },
  "190": {
    "value": 190,
    "name": "RES 7,(HL)",
    "tcycles": [16, 16],
    "mcycles": [4, 4],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "32": "Ignore",
      "128": "Ignore",
      "64": "Ignore"
    }
  },
  "191": {
    "value": 191,
    "name": "RES 7,A",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Ignore",
      "32": "Ignore",
      "16": "Ignore",
      "128": "Ignore"
    }
  },
  "192": {
    "value": 192,
    "name": "SET 0,B",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Ignore",
      "16": "Ignore",
      "128": "Ignore",
      "32": "Ignore"
    }
  },
  "193": {
    "value": 193,
    "name": "SET 0,C",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Ignore",
      "128": "Ignore",
      "64": "Ignore",
      "16": "Ignore"
    }
  },
  "194": {
    "value": 194,
    "name": "SET 0,D",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Ignore",
      "16": "Ignore",
      "128": "Ignore",
      "64": "Ignore"
    }
  },
  "195": {
    "value": 195,
    "name": "SET 0,E",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Ignore",
      "16": "Ignore",
      "32": "Ignore",
      "64": "Ignore"
    }
  },
  "196": {
    "value": 196,
    "name": "SET 0,H",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Ignore",
      "16": "Ignore",
      "32": "Ignore",
      "128": "Ignore"
    }
  },
  "197": {
    "value": 197,
    "name": "SET 0,L",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Ignore",
      "128": "Ignore",
      "64": "Ignore",
      "16": "Ignore"
    }
  },
  "198": {
    "value": 198,
    "name": "SET 0,(HL)",
    "tcycles": [16, 16],
    "mcycles": [4, 4],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "32": "Ignore",
      "128": "Ignore",
      "64": "Ignore"
    }
  },
  "199": {
    "value": 199,
    "name": "SET 0,A",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Ignore",
      "64": "Ignore",
      "128": "Ignore",
      "16": "Ignore"
    }
  },
  "200": {
    "value": 200,
    "name": "SET 1,B",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Ignore",
      "32": "Ignore",
      "64": "Ignore",
      "16": "Ignore"
    }
  },
  "201": {
    "value": 201,
    "name": "SET 1,C",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Ignore",
      "32": "Ignore",
      "16": "Ignore",
      "128": "Ignore"
    }
  },
  "202": {
    "value": 202,
    "name": "SET 1,D",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Ignore",
      "16": "Ignore",
      "128": "Ignore",
      "32": "Ignore"
    }
  },
  "203": {
    "value": 203,
    "name": "SET 1,E",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Ignore",
      "32": "Ignore",
      "16": "Ignore",
      "128": "Ignore"
    }
  },
  "204": {
    "value": 204,
    "name": "SET 1,H",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "128": "Ignore",
      "64": "Ignore",
      "32": "Ignore"
    }
  },
  "205": {
    "value": 205,
    "name": "SET 1,L",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Ignore",
      "64": "Ignore",
      "32": "Ignore",
      "16": "Ignore"
    }
  },
  "206": {
    "value": 206,
    "name": "SET 1,(HL)",
    "tcycles": [16, 16],
    "mcycles": [4, 4],
    "length": 2,
    "flags_to_action": {
      "128": "Ignore",
      "64": "Ignore",
      "16": "Ignore",
      "32": "Ignore"
    }
  },
  "207": {
    "value": 207,
    "name": "SET 1,A",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Ignore",
      "64": "Ignore",
      "32": "Ignore",
      "16": "Ignore"
    }
  },
  "208": {
    "value": 208,
    "name": "SET 2,B",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Ignore",
      "128": "Ignore",
      "16": "Ignore",
      "32": "Ignore"
    }
  },
  "209": {
    "value": 209,
    "name": "SET 2,C",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "64": "Ignore",
      "128": "Ignore",
      "32": "Ignore"
    }
  },
  "210": {
    "value": 210,
    "name": "SET 2,D",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Ignore",
      "16": "Ignore",
      "128": "Ignore",
      "64": "Ignore"
    }
  },
  "211": {
    "value": 211,
    "name": "SET 2,E",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Ignore",
      "32": "Ignore",
      "64": "Ignore",
      "16": "Ignore"
    }
  },
  "212": {
    "value": 212,
    "name": "SET 2,H",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "64": "Ignore",
      "128": "Ignore",
      "32": "Ignore"
    }
  },
  "213": {
    "value": 213,
    "name": "SET 2,L",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Ignore",
      "16": "Ignore",
      "64": "Ignore",
      "32": "Ignore"
    }
  },
  "214": {
    "value": 214,
    "name": "SET 2,(HL)",
    "tcycles": [16, 16],
    "mcycles": [4, 4],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "64": "Ignore",
      "32": "Ignore",
      "128": "Ignore"
    }
  },
  "215": {
    "value": 215,
    "name": "SET 2,A",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Ignore",
      "32": "Ignore",
      "64": "Ignore",
      "16": "Ignore"
    }
  },
  "216": {
    "value": 216,
    "name": "SET 3,B",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "128": "Ignore",
      "32": "Ignore",
      "64": "Ignore"
    }
  },
  "217": {
    "value": 217,
    "name": "SET 3,C",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Ignore",
      "64": "Ignore",
      "16": "Ignore",
      "32": "Ignore"
    }
  },
  "218": {
    "value": 218,
    "name": "SET 3,D",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Ignore",
      "32": "Ignore",
      "128": "Ignore",
      "16": "Ignore"
    }
  },
  "219": {
    "value": 219,
    "name": "SET 3,E",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "32": "Ignore",
      "64": "Ignore",
      "128": "Ignore"
    }
  },
  "220": {
    "value": 220,
    "name": "SET 3,H",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "128": "Ignore",
      "64": "Ignore",
      "32": "Ignore"
    }
  },
  "221": {
    "value": 221,
    "name": "SET 3,L",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Ignore",
      "128": "Ignore",
      "16": "Ignore",
      "64": "Ignore"
    }
  },
  "222": {
    "value": 222,
    "name": "SET 3,(HL)",
    "tcycles": [16, 16],
    "mcycles": [4, 4],
    "length": 2,
    "flags_to_action": {
      "128": "Ignore",
      "64": "Ignore",
      "32": "Ignore",
      "16": "Ignore"
    }
  },
  "223": {
    "value": 223,
    "name": "SET 3,A",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Ignore",
      "128": "Ignore",
      "64": "Ignore",
      "16": "Ignore"
    }
  },
  "224": {
    "value": 224,
    "name": "SET 4,B",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "128": "Ignore",
      "32": "Ignore",
      "64": "Ignore"
    }
  },
  "225": {
    "value": 225,
    "name": "SET 4,C",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Ignore",
      "64": "Ignore",
      "128": "Ignore",
      "16": "Ignore"
    }
  },
  "226": {
    "value": 226,
    "name": "SET 4,D",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Ignore",
      "32": "Ignore",
      "128": "Ignore",
      "16": "Ignore"
    }
  },
  "227": {
    "value": 227,
    "name": "SET 4,E",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Ignore",
      "32": "Ignore",
      "16": "Ignore",
      "128": "Ignore"
    }
  },
  "228": {
    "value": 228,
    "name": "SET 4,H",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "32": "Ignore",
      "128": "Ignore",
      "64": "Ignore"
    }
  },
  "229": {
    "value": 229,
    "name": "SET 4,L",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Ignore",
      "64": "Ignore",
      "32": "Ignore",
      "16": "Ignore"
    }
  },
  "230": {
    "value": 230,
    "name": "SET 4,(HL)",
    "tcycles": [16, 16],
    "mcycles": [4, 4],
    "length": 2,
    "flags_to_action": {
      "32": "Ignore",
      "64": "Ignore",
      "128": "Ignore",
      "16": "Ignore"
    }
  },
  "231": {
    "value": 231,
    "name": "SET 4,A",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Ignore",
      "128": "Ignore",
      "16": "Ignore",
      "64": "Ignore"
    }
  },
  "232": {
    "value": 232,
    "name": "SET 5,B",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "64": "Ignore",
      "128": "Ignore",
      "32": "Ignore"
    }
  },
  "233": {
    "value": 233,
    "name": "SET 5,C",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Ignore",
      "16": "Ignore",
      "32": "Ignore",
      "64": "Ignore"
    }
  },
  "234": {
    "value": 234,
    "name": "SET 5,D",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Ignore",
      "128": "Ignore",
      "64": "Ignore",
      "16": "Ignore"
    }
  },
  "235": {
    "value": 235,
    "name": "SET 5,E",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Ignore",
      "64": "Ignore",
      "32": "Ignore",
      "16": "Ignore"
    }
  },
  "236": {
    "value": 236,
    "name": "SET 5,H",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "64": "Ignore",
      "128": "Ignore",
      "32": "Ignore"
    }
  },
  "237": {
    "value": 237,
    "name": "SET 5,L",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Ignore",
      "128": "Ignore",
      "32": "Ignore",
      "16": "Ignore"
    }
  },
  "238": {
    "value": 238,
    "name": "SET 5,(HL)",
    "tcycles": [16, 16],
    "mcycles": [4, 4],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "32": "Ignore",
      "64": "Ignore",
      "128": "Ignore"
    }
  },
  "239": {
    "value": 239,
    "name": "SET 5,A",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Ignore",
      "64": "Ignore",
      "16": "Ignore",
      "32": "Ignore"
    }
  },
  "240": {
    "value": 240,
    "name": "SET 6,B",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Ignore",
      "32": "Ignore",
      "16": "Ignore",
      "64": "Ignore"
    }
  },
  "241": {
    "value": 241,
    "name": "SET 6,C",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "64": "Ignore",
      "128": "Ignore",
      "32": "Ignore"
    }
  },
  "242": {
    "value": 242,
    "name": "SET 6,D",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "128": "Ignore",
      "64": "Ignore",
      "32": "Ignore"
    }
  },
  "243": {
    "value": 243,
    "name": "SET 6,E",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "128": "Ignore",
      "32": "Ignore",
      "64": "Ignore"
    }
  },
  "244": {
    "value": 244,
    "name": "SET 6,H",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "32": "Ignore",
      "128": "Ignore",
      "64": "Ignore"
    }
  },
  "245": {
    "value": 245,
    "name": "SET 6,L",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "64": "Ignore",
      "32": "Ignore",
      "128": "Ignore"
    }
  },
  "246": {
    "value": 246,
    "name": "SET 6,(HL)",
    "tcycles": [16, 16],
    "mcycles": [4, 4],
    "length": 2,
    "flags_to_action": {
      "32": "Ignore",
      "128": "Ignore",
      "64": "Ignore",
      "16": "Ignore"
    }
  },
  "247": {
    "value": 247,
    "name": "SET 6,A",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Ignore",
      "16": "Ignore",
      "32": "Ignore",
      "64": "Ignore"
    }
  },
  "248": {
    "value": 248,
    "name": "SET 7,B",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Ignore",
      "128": "Ignore",
      "16": "Ignore",
      "64": "Ignore"
    }
  },
  "249": {
    "value": 249,
    "name": "SET 7,C",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Ignore",
      "128": "Ignore",
      "16": "Ignore",
      "64": "Ignore"
    }
  },
  "250": {
    "value": 250,
    "name": "SET 7,D",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Ignore",
      "128": "Ignore",
      "16": "Ignore",
      "32": "Ignore"
    }
  },
  "251": {
    "value": 251,
    "name": "SET 7,E",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Ignore",
      "64": "Ignore",
      "16": "Ignore",
      "32": "Ignore"
    }
  },
  "252": {
    "value": 252,
    "name": "SET 7,H",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "64": "Ignore",
      "128": "Ignore",
      "32": "Ignore"
    }
  },
  "253": {
    "value": 253,
    "name": "SET 7,L",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Ignore",
      "128": "Ignore",
      "16": "Ignore",
      "64": "Ignore"
    }
  },
  "254": {
    "value": 254,
    "name": "SET 7,(HL)",
    "tcycles": [16, 16],
    "mcycles": [4, 4],
    "length": 2,
    "flags_to_action": {
      "128": "Ignore",
      "32": "Ignore",
      "64": "Ignore",
      "16": "Ignore"
    }
  },
  "255": {
    "value": 255,
    "name": "SET 7,A",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "64": "Ignore",
      "32": "Ignore",
      "128": "Ignore"
    }
  }
}"#;
