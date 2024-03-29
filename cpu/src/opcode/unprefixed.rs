pub const UNPREFIXED: &str = r#"{
  "0": {
    "value": 0,
    "name": "NOP",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "128": "Ignore",
      "32": "Ignore",
      "64": "Ignore",
      "16": "Ignore"
    }
  },
  "1": {
    "value": 1,
    "name": "LD BC,u16",
    "tcycles": [12, 12],
    "mcycles": [3, 3],
    "length": 3,
    "flags_to_action": {
      "64": "Ignore",
      "32": "Ignore",
      "16": "Ignore",
      "128": "Ignore"
    }
  },
  "2": {
    "value": 2,
    "name": "LD (BC),A",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 1,
    "flags_to_action": {
      "16": "Ignore",
      "128": "Ignore",
      "64": "Ignore",
      "32": "Ignore"
    }
  },
  "3": {
    "value": 3,
    "name": "INC BC",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 1,
    "flags_to_action": {
      "64": "Ignore",
      "32": "Ignore",
      "128": "Ignore",
      "16": "Ignore"
    }
  },
  "4": {
    "value": 4,
    "name": "INC B",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "32": "Depend",
      "16": "Ignore",
      "128": "Depend",
      "64": "Unset"
    }
  },
  "5": {
    "value": 5,
    "name": "DEC B",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "128": "Depend",
      "16": "Ignore",
      "64": "Set",
      "32": "Depend"
    }
  },
  "6": {
    "value": 6,
    "name": "LD B,u8",
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
  "7": {
    "value": 7,
    "name": "RLCA",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "16": "Depend",
      "128": "Unset",
      "64": "Unset",
      "32": "Unset"
    }
  },
  "8": {
    "value": 8,
    "name": "LD (u16),SP",
    "tcycles": [20, 20],
    "mcycles": [5, 5],
    "length": 3,
    "flags_to_action": {
      "128": "Ignore",
      "64": "Ignore",
      "32": "Ignore",
      "16": "Ignore"
    }
  },
  "9": {
    "value": 9,
    "name": "ADD HL,BC",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 1,
    "flags_to_action": {
      "32": "Depend",
      "16": "Depend",
      "128": "Ignore",
      "64": "Unset"
    }
  },
  "10": {
    "value": 10,
    "name": "LD A,(BC)",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 1,
    "flags_to_action": {
      "128": "Ignore",
      "16": "Ignore",
      "64": "Ignore",
      "32": "Ignore"
    }
  },
  "11": {
    "value": 11,
    "name": "DEC BC",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 1,
    "flags_to_action": {
      "128": "Ignore",
      "32": "Ignore",
      "16": "Ignore",
      "64": "Ignore"
    }
  },
  "12": {
    "value": 12,
    "name": "INC C",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "64": "Unset",
      "128": "Depend",
      "32": "Depend",
      "16": "Ignore"
    }
  },
  "13": {
    "value": 13,
    "name": "DEC C",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "128": "Depend",
      "64": "Set",
      "16": "Ignore",
      "32": "Depend"
    }
  },
  "14": {
    "value": 14,
    "name": "LD C,u8",
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
  "15": {
    "value": 15,
    "name": "RRCA",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "64": "Unset",
      "128": "Unset",
      "16": "Depend",
      "32": "Unset"
    }
  },
  "16": {
    "value": 16,
    "name": "STOP",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "32": "Ignore",
      "128": "Ignore",
      "16": "Ignore",
      "64": "Ignore"
    }
  },
  "17": {
    "value": 17,
    "name": "LD DE,u16",
    "tcycles": [12, 12],
    "mcycles": [3, 3],
    "length": 3,
    "flags_to_action": {
      "16": "Ignore",
      "32": "Ignore",
      "128": "Ignore",
      "64": "Ignore"
    }
  },
  "18": {
    "value": 18,
    "name": "LD (DE),A",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 1,
    "flags_to_action": {
      "16": "Ignore",
      "64": "Ignore",
      "128": "Ignore",
      "32": "Ignore"
    }
  },
  "19": {
    "value": 19,
    "name": "INC DE",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 1,
    "flags_to_action": {
      "32": "Ignore",
      "16": "Ignore",
      "128": "Ignore",
      "64": "Ignore"
    }
  },
  "20": {
    "value": 20,
    "name": "INC D",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "16": "Ignore",
      "128": "Depend",
      "64": "Unset",
      "32": "Depend"
    }
  },
  "21": {
    "value": 21,
    "name": "DEC D",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "64": "Set",
      "16": "Ignore",
      "128": "Depend",
      "32": "Depend"
    }
  },
  "22": {
    "value": 22,
    "name": "LD D,u8",
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
  "23": {
    "value": 23,
    "name": "RLA",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "128": "Unset",
      "64": "Unset",
      "16": "Depend",
      "32": "Unset"
    }
  },
  "24": {
    "value": 24,
    "name": "JR i8",
    "tcycles": [12, 12],
    "mcycles": [3, 3],
    "length": 2,
    "flags_to_action": {
      "32": "Ignore",
      "64": "Ignore",
      "16": "Ignore",
      "128": "Ignore"
    }
  },
  "25": {
    "value": 25,
    "name": "ADD HL,DE",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 1,
    "flags_to_action": {
      "16": "Depend",
      "64": "Unset",
      "128": "Ignore",
      "32": "Depend"
    }
  },
  "26": {
    "value": 26,
    "name": "LD A,(DE)",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 1,
    "flags_to_action": {
      "16": "Ignore",
      "64": "Ignore",
      "32": "Ignore",
      "128": "Ignore"
    }
  },
  "27": {
    "value": 27,
    "name": "DEC DE",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 1,
    "flags_to_action": {
      "32": "Ignore",
      "16": "Ignore",
      "64": "Ignore",
      "128": "Ignore"
    }
  },
  "28": {
    "value": 28,
    "name": "INC E",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "64": "Unset",
      "128": "Depend",
      "32": "Depend",
      "16": "Ignore"
    }
  },
  "29": {
    "value": 29,
    "name": "DEC E",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "16": "Ignore",
      "64": "Set",
      "128": "Depend",
      "32": "Depend"
    }
  },
  "30": {
    "value": 30,
    "name": "LD E,u8",
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
  "31": {
    "value": 31,
    "name": "RRA",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "64": "Unset",
      "32": "Unset",
      "128": "Unset",
      "16": "Depend"
    }
  },
  "32": {
    "value": 32,
    "name": "JR NZ,i8",
    "tcycles": [8, 12],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Ignore",
      "16": "Ignore",
      "128": "Ignore",
      "64": "Ignore"
    }
  },
  "33": {
    "value": 33,
    "name": "LD HL,u16",
    "tcycles": [12, 12],
    "mcycles": [3, 3],
    "length": 3,
    "flags_to_action": {
      "16": "Ignore",
      "32": "Ignore",
      "128": "Ignore",
      "64": "Ignore"
    }
  },
  "34": {
    "value": 34,
    "name": "LD (HL+),A",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 1,
    "flags_to_action": {
      "32": "Ignore",
      "128": "Ignore",
      "64": "Ignore",
      "16": "Ignore"
    }
  },
  "35": {
    "value": 35,
    "name": "INC HL",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 1,
    "flags_to_action": {
      "32": "Ignore",
      "16": "Ignore",
      "128": "Ignore",
      "64": "Ignore"
    }
  },
  "36": {
    "value": 36,
    "name": "INC H",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "64": "Unset",
      "32": "Depend",
      "16": "Ignore",
      "128": "Depend"
    }
  },
  "37": {
    "value": 37,
    "name": "DEC H",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "16": "Ignore",
      "128": "Depend",
      "64": "Set",
      "32": "Depend"
    }
  },
  "38": {
    "value": 38,
    "name": "LD H,u8",
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
  "39": {
    "value": 39,
    "name": "DAA",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "16": "Depend",
      "128": "Depend",
      "64": "Ignore",
      "32": "Unset"
    }
  },
  "40": {
    "value": 40,
    "name": "JR Z,i8",
    "tcycles": [8, 12],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "128": "Ignore",
      "64": "Ignore",
      "32": "Ignore",
      "16": "Ignore"
    }
  },
  "41": {
    "value": 41,
    "name": "ADD HL,HL",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 1,
    "flags_to_action": {
      "64": "Unset",
      "32": "Depend",
      "16": "Depend",
      "128": "Ignore"
    }
  },
  "42": {
    "value": 42,
    "name": "LD A,(HL+)",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 1,
    "flags_to_action": {
      "32": "Ignore",
      "128": "Ignore",
      "64": "Ignore",
      "16": "Ignore"
    }
  },
  "43": {
    "value": 43,
    "name": "DEC HL",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 1,
    "flags_to_action": {
      "16": "Ignore",
      "64": "Ignore",
      "32": "Ignore",
      "128": "Ignore"
    }
  },
  "44": {
    "value": 44,
    "name": "INC L",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "16": "Ignore",
      "32": "Depend",
      "64": "Unset",
      "128": "Depend"
    }
  },
  "45": {
    "value": 45,
    "name": "DEC L",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "32": "Depend",
      "64": "Set",
      "16": "Ignore",
      "128": "Depend"
    }
  },
  "46": {
    "value": 46,
    "name": "LD L,u8",
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
  "47": {
    "value": 47,
    "name": "CPL",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "128": "Ignore",
      "64": "Set",
      "32": "Set",
      "16": "Ignore"
    }
  },
  "48": {
    "value": 48,
    "name": "JR NC,i8",
    "tcycles": [8, 12],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "128": "Ignore",
      "32": "Ignore",
      "64": "Ignore"
    }
  },
  "49": {
    "value": 49,
    "name": "LD SP,u16",
    "tcycles": [12, 12],
    "mcycles": [3, 3],
    "length": 3,
    "flags_to_action": {
      "64": "Ignore",
      "32": "Ignore",
      "16": "Ignore",
      "128": "Ignore"
    }
  },
  "50": {
    "value": 50,
    "name": "LD (HL-),A",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 1,
    "flags_to_action": {
      "16": "Ignore",
      "128": "Ignore",
      "64": "Ignore",
      "32": "Ignore"
    }
  },
  "51": {
    "value": 51,
    "name": "INC SP",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 1,
    "flags_to_action": {
      "32": "Ignore",
      "64": "Ignore",
      "16": "Ignore",
      "128": "Ignore"
    }
  },
  "52": {
    "value": 52,
    "name": "INC (HL)",
    "tcycles": [12, 12],
    "mcycles": [3, 3],
    "length": 1,
    "flags_to_action": {
      "64": "Unset",
      "128": "Depend",
      "32": "Depend",
      "16": "Ignore"
    }
  },
  "53": {
    "value": 53,
    "name": "DEC (HL)",
    "tcycles": [12, 12],
    "mcycles": [3, 3],
    "length": 1,
    "flags_to_action": {
      "64": "Set",
      "16": "Ignore",
      "32": "Depend",
      "128": "Depend"
    }
  },
  "54": {
    "value": 54,
    "name": "LD (HL),u8",
    "tcycles": [12, 12],
    "mcycles": [3, 3],
    "length": 2,
    "flags_to_action": {
      "32": "Ignore",
      "128": "Ignore",
      "64": "Ignore",
      "16": "Ignore"
    }
  },
  "55": {
    "value": 55,
    "name": "SCF",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "32": "Unset",
      "16": "Set",
      "128": "Ignore",
      "64": "Unset"
    }
  },
  "56": {
    "value": 56,
    "name": "JR C,i8",
    "tcycles": [8, 12],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "32": "Ignore",
      "64": "Ignore",
      "128": "Ignore"
    }
  },
  "57": {
    "value": 57,
    "name": "ADD HL,SP",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 1,
    "flags_to_action": {
      "32": "Depend",
      "16": "Depend",
      "128": "Ignore",
      "64": "Unset"
    }
  },
  "58": {
    "value": 58,
    "name": "LD A,(HL-)",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 1,
    "flags_to_action": {
      "64": "Ignore",
      "16": "Ignore",
      "32": "Ignore",
      "128": "Ignore"
    }
  },
  "59": {
    "value": 59,
    "name": "DEC SP",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 1,
    "flags_to_action": {
      "64": "Ignore",
      "128": "Ignore",
      "16": "Ignore",
      "32": "Ignore"
    }
  },
  "60": {
    "value": 60,
    "name": "INC A",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "32": "Depend",
      "64": "Unset",
      "128": "Depend",
      "16": "Ignore"
    }
  },
  "61": {
    "value": 61,
    "name": "DEC A",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "64": "Set",
      "16": "Ignore",
      "128": "Depend",
      "32": "Depend"
    }
  },
  "62": {
    "value": 62,
    "name": "LD A,u8",
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
  "63": {
    "value": 63,
    "name": "CCF",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "16": "Depend",
      "32": "Unset",
      "128": "Ignore",
      "64": "Unset"
    }
  },
  "64": {
    "value": 64,
    "name": "LD B,B",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "16": "Ignore",
      "32": "Ignore",
      "64": "Ignore",
      "128": "Ignore"
    }
  },
  "65": {
    "value": 65,
    "name": "LD B,C",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "64": "Ignore",
      "16": "Ignore",
      "128": "Ignore",
      "32": "Ignore"
    }
  },
  "66": {
    "value": 66,
    "name": "LD B,D",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "128": "Ignore",
      "64": "Ignore",
      "32": "Ignore",
      "16": "Ignore"
    }
  },
  "67": {
    "value": 67,
    "name": "LD B,E",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "64": "Ignore",
      "32": "Ignore",
      "128": "Ignore",
      "16": "Ignore"
    }
  },
  "68": {
    "value": 68,
    "name": "LD B,H",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "32": "Ignore",
      "16": "Ignore",
      "64": "Ignore",
      "128": "Ignore"
    }
  },
  "69": {
    "value": 69,
    "name": "LD B,L",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "128": "Ignore",
      "32": "Ignore",
      "16": "Ignore",
      "64": "Ignore"
    }
  },
  "70": {
    "value": 70,
    "name": "LD B,(HL)",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 1,
    "flags_to_action": {
      "64": "Ignore",
      "32": "Ignore",
      "16": "Ignore",
      "128": "Ignore"
    }
  },
  "71": {
    "value": 71,
    "name": "LD B,A",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "64": "Ignore",
      "128": "Ignore",
      "32": "Ignore",
      "16": "Ignore"
    }
  },
  "72": {
    "value": 72,
    "name": "LD C,B",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "16": "Ignore",
      "128": "Ignore",
      "64": "Ignore",
      "32": "Ignore"
    }
  },
  "73": {
    "value": 73,
    "name": "LD C,C",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "16": "Ignore",
      "128": "Ignore",
      "64": "Ignore",
      "32": "Ignore"
    }
  },
  "74": {
    "value": 74,
    "name": "LD C,D",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "128": "Ignore",
      "64": "Ignore",
      "16": "Ignore",
      "32": "Ignore"
    }
  },
  "75": {
    "value": 75,
    "name": "LD C,E",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "16": "Ignore",
      "64": "Ignore",
      "32": "Ignore",
      "128": "Ignore"
    }
  },
  "76": {
    "value": 76,
    "name": "LD C,H",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "16": "Ignore",
      "128": "Ignore",
      "64": "Ignore",
      "32": "Ignore"
    }
  },
  "77": {
    "value": 77,
    "name": "LD C,L",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "32": "Ignore",
      "64": "Ignore",
      "16": "Ignore",
      "128": "Ignore"
    }
  },
  "78": {
    "value": 78,
    "name": "LD C,(HL)",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 1,
    "flags_to_action": {
      "64": "Ignore",
      "128": "Ignore",
      "32": "Ignore",
      "16": "Ignore"
    }
  },
  "79": {
    "value": 79,
    "name": "LD C,A",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "128": "Ignore",
      "16": "Ignore",
      "32": "Ignore",
      "64": "Ignore"
    }
  },
  "80": {
    "value": 80,
    "name": "LD D,B",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "128": "Ignore",
      "32": "Ignore",
      "16": "Ignore",
      "64": "Ignore"
    }
  },
  "81": {
    "value": 81,
    "name": "LD D,C",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "128": "Ignore",
      "16": "Ignore",
      "64": "Ignore",
      "32": "Ignore"
    }
  },
  "82": {
    "value": 82,
    "name": "LD D,D",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "128": "Ignore",
      "16": "Ignore",
      "32": "Ignore",
      "64": "Ignore"
    }
  },
  "83": {
    "value": 83,
    "name": "LD D,E",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "32": "Ignore",
      "16": "Ignore",
      "64": "Ignore",
      "128": "Ignore"
    }
  },
  "84": {
    "value": 84,
    "name": "LD D,H",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "64": "Ignore",
      "128": "Ignore",
      "32": "Ignore",
      "16": "Ignore"
    }
  },
  "85": {
    "value": 85,
    "name": "LD D,L",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "128": "Ignore",
      "64": "Ignore",
      "32": "Ignore",
      "16": "Ignore"
    }
  },
  "86": {
    "value": 86,
    "name": "LD D,(HL)",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 1,
    "flags_to_action": {
      "32": "Ignore",
      "16": "Ignore",
      "128": "Ignore",
      "64": "Ignore"
    }
  },
  "87": {
    "value": 87,
    "name": "LD D,A",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "16": "Ignore",
      "32": "Ignore",
      "64": "Ignore",
      "128": "Ignore"
    }
  },
  "88": {
    "value": 88,
    "name": "LD E,B",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "64": "Ignore",
      "16": "Ignore",
      "128": "Ignore",
      "32": "Ignore"
    }
  },
  "89": {
    "value": 89,
    "name": "LD E,C",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "128": "Ignore",
      "16": "Ignore",
      "64": "Ignore",
      "32": "Ignore"
    }
  },
  "90": {
    "value": 90,
    "name": "LD E,D",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "32": "Ignore",
      "128": "Ignore",
      "64": "Ignore",
      "16": "Ignore"
    }
  },
  "91": {
    "value": 91,
    "name": "LD E,E",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "16": "Ignore",
      "128": "Ignore",
      "64": "Ignore",
      "32": "Ignore"
    }
  },
  "92": {
    "value": 92,
    "name": "LD E,H",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "128": "Ignore",
      "64": "Ignore",
      "32": "Ignore",
      "16": "Ignore"
    }
  },
  "93": {
    "value": 93,
    "name": "LD E,L",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "32": "Ignore",
      "16": "Ignore",
      "128": "Ignore",
      "64": "Ignore"
    }
  },
  "94": {
    "value": 94,
    "name": "LD E,(HL)",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 1,
    "flags_to_action": {
      "32": "Ignore",
      "128": "Ignore",
      "64": "Ignore",
      "16": "Ignore"
    }
  },
  "95": {
    "value": 95,
    "name": "LD E,A",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "128": "Ignore",
      "64": "Ignore",
      "32": "Ignore",
      "16": "Ignore"
    }
  },
  "96": {
    "value": 96,
    "name": "LD H,B",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "128": "Ignore",
      "16": "Ignore",
      "32": "Ignore",
      "64": "Ignore"
    }
  },
  "97": {
    "value": 97,
    "name": "LD H,C",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "16": "Ignore",
      "128": "Ignore",
      "64": "Ignore",
      "32": "Ignore"
    }
  },
  "98": {
    "value": 98,
    "name": "LD H,D",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "128": "Ignore",
      "64": "Ignore",
      "16": "Ignore",
      "32": "Ignore"
    }
  },
  "99": {
    "value": 99,
    "name": "LD H,E",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "32": "Ignore",
      "128": "Ignore",
      "16": "Ignore",
      "64": "Ignore"
    }
  },
  "100": {
    "value": 100,
    "name": "LD H,H",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "16": "Ignore",
      "64": "Ignore",
      "128": "Ignore",
      "32": "Ignore"
    }
  },
  "101": {
    "value": 101,
    "name": "LD H,L",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "32": "Ignore",
      "16": "Ignore",
      "128": "Ignore",
      "64": "Ignore"
    }
  },
  "102": {
    "value": 102,
    "name": "LD H,(HL)",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 1,
    "flags_to_action": {
      "32": "Ignore",
      "16": "Ignore",
      "64": "Ignore",
      "128": "Ignore"
    }
  },
  "103": {
    "value": 103,
    "name": "LD H,A",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "64": "Ignore",
      "32": "Ignore",
      "128": "Ignore",
      "16": "Ignore"
    }
  },
  "104": {
    "value": 104,
    "name": "LD L,B",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "64": "Ignore",
      "128": "Ignore",
      "16": "Ignore",
      "32": "Ignore"
    }
  },
  "105": {
    "value": 105,
    "name": "LD L,C",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "32": "Ignore",
      "64": "Ignore",
      "128": "Ignore",
      "16": "Ignore"
    }
  },
  "106": {
    "value": 106,
    "name": "LD L,D",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "64": "Ignore",
      "32": "Ignore",
      "16": "Ignore",
      "128": "Ignore"
    }
  },
  "107": {
    "value": 107,
    "name": "LD L,E",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "16": "Ignore",
      "64": "Ignore",
      "32": "Ignore",
      "128": "Ignore"
    }
  },
  "108": {
    "value": 108,
    "name": "LD L,H",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "16": "Ignore",
      "32": "Ignore",
      "64": "Ignore",
      "128": "Ignore"
    }
  },
  "109": {
    "value": 109,
    "name": "LD L,L",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "16": "Ignore",
      "128": "Ignore",
      "64": "Ignore",
      "32": "Ignore"
    }
  },
  "110": {
    "value": 110,
    "name": "LD L,(HL)",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 1,
    "flags_to_action": {
      "32": "Ignore",
      "64": "Ignore",
      "16": "Ignore",
      "128": "Ignore"
    }
  },
  "111": {
    "value": 111,
    "name": "LD L,A",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "32": "Ignore",
      "64": "Ignore",
      "128": "Ignore",
      "16": "Ignore"
    }
  },
  "112": {
    "value": 112,
    "name": "LD (HL),B",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 1,
    "flags_to_action": {
      "64": "Ignore",
      "128": "Ignore",
      "32": "Ignore",
      "16": "Ignore"
    }
  },
  "113": {
    "value": 113,
    "name": "LD (HL),C",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 1,
    "flags_to_action": {
      "32": "Ignore",
      "64": "Ignore",
      "128": "Ignore",
      "16": "Ignore"
    }
  },
  "114": {
    "value": 114,
    "name": "LD (HL),D",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 1,
    "flags_to_action": {
      "32": "Ignore",
      "128": "Ignore",
      "64": "Ignore",
      "16": "Ignore"
    }
  },
  "115": {
    "value": 115,
    "name": "LD (HL),E",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 1,
    "flags_to_action": {
      "32": "Ignore",
      "128": "Ignore",
      "64": "Ignore",
      "16": "Ignore"
    }
  },
  "116": {
    "value": 116,
    "name": "LD (HL),H",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 1,
    "flags_to_action": {
      "128": "Ignore",
      "16": "Ignore",
      "32": "Ignore",
      "64": "Ignore"
    }
  },
  "117": {
    "value": 117,
    "name": "LD (HL),L",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 1,
    "flags_to_action": {
      "16": "Ignore",
      "64": "Ignore",
      "32": "Ignore",
      "128": "Ignore"
    }
  },
  "118": {
    "value": 118,
    "name": "HALT",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "64": "Ignore",
      "16": "Ignore",
      "128": "Ignore",
      "32": "Ignore"
    }
  },
  "119": {
    "value": 119,
    "name": "LD (HL),A",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 1,
    "flags_to_action": {
      "128": "Ignore",
      "32": "Ignore",
      "16": "Ignore",
      "64": "Ignore"
    }
  },
  "120": {
    "value": 120,
    "name": "LD A,B",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "32": "Ignore",
      "16": "Ignore",
      "64": "Ignore",
      "128": "Ignore"
    }
  },
  "121": {
    "value": 121,
    "name": "LD A,C",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "16": "Ignore",
      "128": "Ignore",
      "64": "Ignore",
      "32": "Ignore"
    }
  },
  "122": {
    "value": 122,
    "name": "LD A,D",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "64": "Ignore",
      "128": "Ignore",
      "32": "Ignore",
      "16": "Ignore"
    }
  },
  "123": {
    "value": 123,
    "name": "LD A,E",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "16": "Ignore",
      "128": "Ignore",
      "64": "Ignore",
      "32": "Ignore"
    }
  },
  "124": {
    "value": 124,
    "name": "LD A,H",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "128": "Ignore",
      "16": "Ignore",
      "64": "Ignore",
      "32": "Ignore"
    }
  },
  "125": {
    "value": 125,
    "name": "LD A,L",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "64": "Ignore",
      "128": "Ignore",
      "32": "Ignore",
      "16": "Ignore"
    }
  },
  "126": {
    "value": 126,
    "name": "LD A,(HL)",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 1,
    "flags_to_action": {
      "64": "Ignore",
      "16": "Ignore",
      "32": "Ignore",
      "128": "Ignore"
    }
  },
  "127": {
    "value": 127,
    "name": "LD A,A",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "128": "Ignore",
      "32": "Ignore",
      "16": "Ignore",
      "64": "Ignore"
    }
  },
  "128": {
    "value": 128,
    "name": "ADD A,B",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "16": "Depend",
      "32": "Depend",
      "128": "Depend",
      "64": "Unset"
    }
  },
  "129": {
    "value": 129,
    "name": "ADD A,C",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "64": "Unset",
      "16": "Depend",
      "32": "Depend",
      "128": "Depend"
    }
  },
  "130": {
    "value": 130,
    "name": "ADD A,D",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "32": "Depend",
      "128": "Depend",
      "16": "Depend",
      "64": "Unset"
    }
  },
  "131": {
    "value": 131,
    "name": "ADD A,E",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "64": "Unset",
      "32": "Depend",
      "128": "Depend",
      "16": "Depend"
    }
  },
  "132": {
    "value": 132,
    "name": "ADD A,H",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "128": "Depend",
      "64": "Unset",
      "16": "Depend",
      "32": "Depend"
    }
  },
  "133": {
    "value": 133,
    "name": "ADD A,L",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "32": "Depend",
      "64": "Unset",
      "128": "Depend",
      "16": "Depend"
    }
  },
  "134": {
    "value": 134,
    "name": "ADD A,(HL)",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 1,
    "flags_to_action": {
      "128": "Depend",
      "64": "Unset",
      "32": "Depend",
      "16": "Depend"
    }
  },
  "135": {
    "value": 135,
    "name": "ADD A,A",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "32": "Depend",
      "16": "Depend",
      "128": "Depend",
      "64": "Unset"
    }
  },
  "136": {
    "value": 136,
    "name": "ADC A,B",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "128": "Depend",
      "16": "Depend",
      "32": "Depend",
      "64": "Unset"
    }
  },
  "137": {
    "value": 137,
    "name": "ADC A,C",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "128": "Depend",
      "64": "Unset",
      "32": "Depend",
      "16": "Depend"
    }
  },
  "138": {
    "value": 138,
    "name": "ADC A,D",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "32": "Depend",
      "128": "Depend",
      "64": "Unset",
      "16": "Depend"
    }
  },
  "139": {
    "value": 139,
    "name": "ADC A,E",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "32": "Depend",
      "16": "Depend",
      "64": "Unset",
      "128": "Depend"
    }
  },
  "140": {
    "value": 140,
    "name": "ADC A,H",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "64": "Unset",
      "32": "Depend",
      "128": "Depend",
      "16": "Depend"
    }
  },
  "141": {
    "value": 141,
    "name": "ADC A,L",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "64": "Unset",
      "16": "Depend",
      "128": "Depend",
      "32": "Depend"
    }
  },
  "142": {
    "value": 142,
    "name": "ADC A,(HL)",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 1,
    "flags_to_action": {
      "64": "Unset",
      "32": "Depend",
      "128": "Depend",
      "16": "Depend"
    }
  },
  "143": {
    "value": 143,
    "name": "ADC A,A",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "32": "Depend",
      "128": "Depend",
      "64": "Unset",
      "16": "Depend"
    }
  },
  "144": {
    "value": 144,
    "name": "SUB A,B",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "64": "Set",
      "32": "Depend",
      "128": "Depend",
      "16": "Depend"
    }
  },
  "145": {
    "value": 145,
    "name": "SUB A,C",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "128": "Depend",
      "16": "Depend",
      "32": "Depend",
      "64": "Set"
    }
  },
  "146": {
    "value": 146,
    "name": "SUB A,D",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "128": "Depend",
      "32": "Depend",
      "64": "Set",
      "16": "Depend"
    }
  },
  "147": {
    "value": 147,
    "name": "SUB A,E",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "128": "Depend",
      "64": "Set",
      "16": "Depend",
      "32": "Depend"
    }
  },
  "148": {
    "value": 148,
    "name": "SUB A,H",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "64": "Set",
      "128": "Depend",
      "16": "Depend",
      "32": "Depend"
    }
  },
  "149": {
    "value": 149,
    "name": "SUB A,L",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "64": "Set",
      "16": "Depend",
      "128": "Depend",
      "32": "Depend"
    }
  },
  "150": {
    "value": 150,
    "name": "SUB A,(HL)",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 1,
    "flags_to_action": {
      "16": "Depend",
      "128": "Depend",
      "64": "Set",
      "32": "Depend"
    }
  },
  "151": {
    "value": 151,
    "name": "SUB A,A",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "32": "Depend",
      "128": "Depend",
      "64": "Set",
      "16": "Depend"
    }
  },
  "152": {
    "value": 152,
    "name": "SBC A,B",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "128": "Depend",
      "64": "Set",
      "32": "Depend",
      "16": "Depend"
    }
  },
  "153": {
    "value": 153,
    "name": "SBC A,C",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "16": "Depend",
      "64": "Set",
      "32": "Depend",
      "128": "Depend"
    }
  },
  "154": {
    "value": 154,
    "name": "SBC A,D",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "32": "Depend",
      "16": "Depend",
      "64": "Set",
      "128": "Depend"
    }
  },
  "155": {
    "value": 155,
    "name": "SBC A,E",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "16": "Depend",
      "128": "Depend",
      "64": "Set",
      "32": "Depend"
    }
  },
  "156": {
    "value": 156,
    "name": "SBC A,H",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "64": "Set",
      "32": "Depend",
      "16": "Depend",
      "128": "Depend"
    }
  },
  "157": {
    "value": 157,
    "name": "SBC A,L",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "64": "Set",
      "16": "Depend",
      "128": "Depend",
      "32": "Depend"
    }
  },
  "158": {
    "value": 158,
    "name": "SBC A,(HL)",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 1,
    "flags_to_action": {
      "64": "Set",
      "128": "Depend",
      "32": "Depend",
      "16": "Depend"
    }
  },
  "159": {
    "value": 159,
    "name": "SBC A,A",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "32": "Depend",
      "16": "Depend",
      "64": "Set",
      "128": "Depend"
    }
  },
  "160": {
    "value": 160,
    "name": "AND A,B",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "128": "Depend",
      "64": "Unset",
      "16": "Unset",
      "32": "Set"
    }
  },
  "161": {
    "value": 161,
    "name": "AND A,C",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "64": "Unset",
      "16": "Unset",
      "128": "Depend",
      "32": "Set"
    }
  },
  "162": {
    "value": 162,
    "name": "AND A,D",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "16": "Unset",
      "64": "Unset",
      "128": "Depend",
      "32": "Set"
    }
  },
  "163": {
    "value": 163,
    "name": "AND A,E",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "128": "Depend",
      "32": "Set",
      "64": "Unset",
      "16": "Unset"
    }
  },
  "164": {
    "value": 164,
    "name": "AND A,H",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "16": "Unset",
      "32": "Set",
      "128": "Depend",
      "64": "Unset"
    }
  },
  "165": {
    "value": 165,
    "name": "AND A,L",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "64": "Unset",
      "128": "Depend",
      "32": "Set",
      "16": "Unset"
    }
  },
  "166": {
    "value": 166,
    "name": "AND A,(HL)",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 1,
    "flags_to_action": {
      "64": "Unset",
      "32": "Set",
      "128": "Depend",
      "16": "Unset"
    }
  },
  "167": {
    "value": 167,
    "name": "AND A,A",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "64": "Unset",
      "128": "Depend",
      "16": "Unset",
      "32": "Set"
    }
  },
  "168": {
    "value": 168,
    "name": "XOR A,B",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "128": "Depend",
      "16": "Unset",
      "32": "Unset",
      "64": "Unset"
    }
  },
  "169": {
    "value": 169,
    "name": "XOR A,C",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "16": "Unset",
      "32": "Unset",
      "128": "Depend",
      "64": "Unset"
    }
  },
  "170": {
    "value": 170,
    "name": "XOR A,D",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "32": "Unset",
      "64": "Unset",
      "128": "Depend",
      "16": "Unset"
    }
  },
  "171": {
    "value": 171,
    "name": "XOR A,E",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "16": "Unset",
      "32": "Unset",
      "128": "Depend",
      "64": "Unset"
    }
  },
  "172": {
    "value": 172,
    "name": "XOR A,H",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "64": "Unset",
      "32": "Unset",
      "16": "Unset",
      "128": "Depend"
    }
  },
  "173": {
    "value": 173,
    "name": "XOR A,L",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "64": "Unset",
      "128": "Depend",
      "32": "Unset",
      "16": "Unset"
    }
  },
  "174": {
    "value": 174,
    "name": "XOR A,(HL)",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 1,
    "flags_to_action": {
      "64": "Unset",
      "128": "Depend",
      "16": "Unset",
      "32": "Unset"
    }
  },
  "175": {
    "value": 175,
    "name": "XOR A,A",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "64": "Unset",
      "16": "Unset",
      "128": "Depend",
      "32": "Unset"
    }
  },
  "176": {
    "value": 176,
    "name": "OR A,B",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "16": "Unset",
      "128": "Depend",
      "64": "Unset",
      "32": "Unset"
    }
  },
  "177": {
    "value": 177,
    "name": "OR A,C",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "64": "Unset",
      "16": "Unset",
      "32": "Unset",
      "128": "Depend"
    }
  },
  "178": {
    "value": 178,
    "name": "OR A,D",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "16": "Unset",
      "128": "Depend",
      "32": "Unset",
      "64": "Unset"
    }
  },
  "179": {
    "value": 179,
    "name": "OR A,E",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "16": "Unset",
      "128": "Depend",
      "64": "Unset",
      "32": "Unset"
    }
  },
  "180": {
    "value": 180,
    "name": "OR A,H",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "32": "Unset",
      "16": "Unset",
      "128": "Depend",
      "64": "Unset"
    }
  },
  "181": {
    "value": 181,
    "name": "OR A,L",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "128": "Depend",
      "64": "Unset",
      "16": "Unset",
      "32": "Unset"
    }
  },
  "182": {
    "value": 182,
    "name": "OR A,(HL)",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 1,
    "flags_to_action": {
      "16": "Unset",
      "32": "Unset",
      "128": "Depend",
      "64": "Unset"
    }
  },
  "183": {
    "value": 183,
    "name": "OR A,A",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "128": "Depend",
      "16": "Unset",
      "32": "Unset",
      "64": "Unset"
    }
  },
  "184": {
    "value": 184,
    "name": "CP A,B",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "16": "Depend",
      "32": "Depend",
      "64": "Set",
      "128": "Depend"
    }
  },
  "185": {
    "value": 185,
    "name": "CP A,C",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "128": "Depend",
      "16": "Depend",
      "64": "Set",
      "32": "Depend"
    }
  },
  "186": {
    "value": 186,
    "name": "CP A,D",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "128": "Depend",
      "32": "Depend",
      "16": "Depend",
      "64": "Set"
    }
  },
  "187": {
    "value": 187,
    "name": "CP A,E",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "128": "Depend",
      "32": "Depend",
      "64": "Set",
      "16": "Depend"
    }
  },
  "188": {
    "value": 188,
    "name": "CP A,H",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "32": "Depend",
      "16": "Depend",
      "64": "Set",
      "128": "Depend"
    }
  },
  "189": {
    "value": 189,
    "name": "CP A,L",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "64": "Set",
      "32": "Depend",
      "128": "Depend",
      "16": "Depend"
    }
  },
  "190": {
    "value": 190,
    "name": "CP A,(HL)",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 1,
    "flags_to_action": {
      "64": "Set",
      "32": "Depend",
      "16": "Depend",
      "128": "Depend"
    }
  },
  "191": {
    "value": 191,
    "name": "CP A,A",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "64": "Set",
      "16": "Depend",
      "32": "Depend",
      "128": "Depend"
    }
  },
  "192": {
    "value": 192,
    "name": "RET NZ",
    "tcycles": [8, 20],
    "mcycles": [2, 2],
    "length": 1,
    "flags_to_action": {
      "32": "Ignore",
      "128": "Ignore",
      "16": "Ignore",
      "64": "Ignore"
    }
  },
  "193": {
    "value": 193,
    "name": "POP BC",
    "tcycles": [12, 12],
    "mcycles": [3, 3],
    "length": 1,
    "flags_to_action": {
      "32": "Ignore",
      "16": "Ignore",
      "128": "Ignore",
      "64": "Ignore"
    }
  },
  "194": {
    "value": 194,
    "name": "JP NZ,u16",
    "tcycles": [12, 16],
    "mcycles": [3, 3],
    "length": 3,
    "flags_to_action": {
      "128": "Ignore",
      "64": "Ignore",
      "32": "Ignore",
      "16": "Ignore"
    }
  },
  "195": {
    "value": 195,
    "name": "JP u16",
    "tcycles": [16, 16],
    "mcycles": [4, 4],
    "length": 3,
    "flags_to_action": {
      "128": "Ignore",
      "64": "Ignore",
      "16": "Ignore",
      "32": "Ignore"
    }
  },
  "196": {
    "value": 196,
    "name": "CALL NZ,u16",
    "tcycles": [12, 24],
    "mcycles": [3, 3],
    "length": 3,
    "flags_to_action": {
      "32": "Ignore",
      "128": "Ignore",
      "64": "Ignore",
      "16": "Ignore"
    }
  },
  "197": {
    "value": 197,
    "name": "PUSH BC",
    "tcycles": [16, 16],
    "mcycles": [4, 4],
    "length": 1,
    "flags_to_action": {
      "64": "Ignore",
      "128": "Ignore",
      "32": "Ignore",
      "16": "Ignore"
    }
  },
  "198": {
    "value": 198,
    "name": "ADD A,u8",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Depend",
      "64": "Unset",
      "128": "Depend",
      "16": "Depend"
    }
  },
  "199": {
    "value": 199,
    "name": "RST 00h",
    "tcycles": [16, 16],
    "mcycles": [4, 4],
    "length": 1,
    "flags_to_action": {
      "128": "Ignore",
      "32": "Ignore",
      "64": "Ignore",
      "16": "Ignore"
    }
  },
  "200": {
    "value": 200,
    "name": "RET Z",
    "tcycles": [8, 20],
    "mcycles": [2, 2],
    "length": 1,
    "flags_to_action": {
      "64": "Ignore",
      "32": "Ignore",
      "128": "Ignore",
      "16": "Ignore"
    }
  },
  "201": {
    "value": 201,
    "name": "RET",
    "tcycles": [16, 16],
    "mcycles": [4, 4],
    "length": 1,
    "flags_to_action": {
      "16": "Ignore",
      "128": "Ignore",
      "32": "Ignore",
      "64": "Ignore"
    }
  },
  "202": {
    "value": 202,
    "name": "JP Z,u16",
    "tcycles": [12, 16],
    "mcycles": [3, 3],
    "length": 3,
    "flags_to_action": {
      "128": "Ignore",
      "64": "Ignore",
      "32": "Ignore",
      "16": "Ignore"
    }
  },
  "203": {
    "value": 203,
    "name": "PREFIX CB",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "128": "Ignore",
      "64": "Ignore",
      "16": "Ignore",
      "32": "Ignore"
    }
  },
  "204": {
    "value": 204,
    "name": "CALL Z,u16",
    "tcycles": [12, 24],
    "mcycles": [3, 3],
    "length": 3,
    "flags_to_action": {
      "16": "Ignore",
      "32": "Ignore",
      "64": "Ignore",
      "128": "Ignore"
    }
  },
  "205": {
    "value": 205,
    "name": "CALL u16",
    "tcycles": [24, 24],
    "mcycles": [6, 6],
    "length": 3,
    "flags_to_action": {
      "16": "Ignore",
      "128": "Ignore",
      "64": "Ignore",
      "32": "Ignore"
    }
  },
  "206": {
    "value": 206,
    "name": "ADC A,u8",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Unset",
      "32": "Depend",
      "16": "Depend",
      "128": "Depend"
    }
  },
  "207": {
    "value": 207,
    "name": "RST 08h",
    "tcycles": [16, 16],
    "mcycles": [4, 4],
    "length": 1,
    "flags_to_action": {
      "16": "Ignore",
      "64": "Ignore",
      "128": "Ignore",
      "32": "Ignore"
    }
  },
  "208": {
    "value": 208,
    "name": "RET NC",
    "tcycles": [8, 20],
    "mcycles": [2, 2],
    "length": 1,
    "flags_to_action": {
      "64": "Ignore",
      "16": "Ignore",
      "32": "Ignore",
      "128": "Ignore"
    }
  },
  "209": {
    "value": 209,
    "name": "POP DE",
    "tcycles": [12, 12],
    "mcycles": [3, 3],
    "length": 1,
    "flags_to_action": {
      "16": "Ignore",
      "64": "Ignore",
      "32": "Ignore",
      "128": "Ignore"
    }
  },
  "210": {
    "value": 210,
    "name": "JP NC,u16",
    "tcycles": [12, 16],
    "mcycles": [3, 3],
    "length": 3,
    "flags_to_action": {
      "16": "Ignore",
      "64": "Ignore",
      "128": "Ignore",
      "32": "Ignore"
    }
  },
  "211": {
    "value": 211,
    "name": "UNUSED",
    "tcycles": [0, 0],
    "mcycles": [0, 0],
    "length": 1,
    "flags_to_action": {
      "64": "Ignore",
      "16": "Ignore",
      "128": "Ignore",
      "32": "Ignore"
    }
  },
  "212": {
    "value": 212,
    "name": "CALL NC,u16",
    "tcycles": [12, 24],
    "mcycles": [3, 3],
    "length": 3,
    "flags_to_action": {
      "128": "Ignore",
      "16": "Ignore",
      "64": "Ignore",
      "32": "Ignore"
    }
  },
  "213": {
    "value": 213,
    "name": "PUSH DE",
    "tcycles": [16, 16],
    "mcycles": [4, 4],
    "length": 1,
    "flags_to_action": {
      "32": "Ignore",
      "128": "Ignore",
      "16": "Ignore",
      "64": "Ignore"
    }
  },
  "214": {
    "value": 214,
    "name": "SUB A,u8",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Depend",
      "64": "Set",
      "128": "Depend",
      "16": "Depend"
    }
  },
  "215": {
    "value": 215,
    "name": "RST 10h",
    "tcycles": [16, 16],
    "mcycles": [4, 4],
    "length": 1,
    "flags_to_action": {
      "16": "Ignore",
      "64": "Ignore",
      "32": "Ignore",
      "128": "Ignore"
    }
  },
  "216": {
    "value": 216,
    "name": "RET C",
    "tcycles": [8, 20],
    "mcycles": [2, 2],
    "length": 1,
    "flags_to_action": {
      "32": "Ignore",
      "128": "Ignore",
      "16": "Ignore",
      "64": "Ignore"
    }
  },
  "217": {
    "value": 217,
    "name": "RETI",
    "tcycles": [16, 16],
    "mcycles": [4, 4],
    "length": 1,
    "flags_to_action": {
      "128": "Ignore",
      "64": "Ignore",
      "16": "Ignore",
      "32": "Ignore"
    }
  },
  "218": {
    "value": 218,
    "name": "JP C,u16",
    "tcycles": [12, 16],
    "mcycles": [3, 3],
    "length": 3,
    "flags_to_action": {
      "128": "Ignore",
      "64": "Ignore",
      "16": "Ignore",
      "32": "Ignore"
    }
  },
  "219": {
    "value": 219,
    "name": "UNUSED",
    "tcycles": [0, 0],
    "mcycles": [0, 0],
    "length": 1,
    "flags_to_action": {
      "128": "Ignore",
      "64": "Ignore",
      "32": "Ignore",
      "16": "Ignore"
    }
  },
  "220": {
    "value": 220,
    "name": "CALL C,u16",
    "tcycles": [12, 24],
    "mcycles": [3, 3],
    "length": 3,
    "flags_to_action": {
      "64": "Ignore",
      "32": "Ignore",
      "16": "Ignore",
      "128": "Ignore"
    }
  },
  "221": {
    "value": 221,
    "name": "UNUSED",
    "tcycles": [0, 0],
    "mcycles": [0, 0],
    "length": 1,
    "flags_to_action": {
      "16": "Ignore",
      "128": "Ignore",
      "64": "Ignore",
      "32": "Ignore"
    }
  },
  "222": {
    "value": 222,
    "name": "SBC A,u8",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Depend",
      "64": "Set",
      "128": "Depend",
      "16": "Depend"
    }
  },
  "223": {
    "value": 223,
    "name": "RST 18h",
    "tcycles": [16, 16],
    "mcycles": [4, 4],
    "length": 1,
    "flags_to_action": {
      "128": "Ignore",
      "32": "Ignore",
      "64": "Ignore",
      "16": "Ignore"
    }
  },
  "224": {
    "value": 224,
    "name": "LD (FF00+u8),A",
    "tcycles": [12, 12],
    "mcycles": [3, 3],
    "length": 2,
    "flags_to_action": {
      "16": "Ignore",
      "64": "Ignore",
      "128": "Ignore",
      "32": "Ignore"
    }
  },
  "225": {
    "value": 225,
    "name": "POP HL",
    "tcycles": [12, 12],
    "mcycles": [3, 3],
    "length": 1,
    "flags_to_action": {
      "128": "Ignore",
      "16": "Ignore",
      "64": "Ignore",
      "32": "Ignore"
    }
  },
  "226": {
    "value": 226,
    "name": "LD (FF00+C),A",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 1,
    "flags_to_action": {
      "32": "Ignore",
      "128": "Ignore",
      "64": "Ignore",
      "16": "Ignore"
    }
  },
  "227": {
    "value": 227,
    "name": "UNUSED",
    "tcycles": [0, 0],
    "mcycles": [0, 0],
    "length": 1,
    "flags_to_action": {
      "128": "Ignore",
      "16": "Ignore",
      "64": "Ignore",
      "32": "Ignore"
    }
  },
  "228": {
    "value": 228,
    "name": "UNUSED",
    "tcycles": [0, 0],
    "mcycles": [0, 0],
    "length": 1,
    "flags_to_action": {
      "64": "Ignore",
      "32": "Ignore",
      "16": "Ignore",
      "128": "Ignore"
    }
  },
  "229": {
    "value": 229,
    "name": "PUSH HL",
    "tcycles": [16, 16],
    "mcycles": [4, 4],
    "length": 1,
    "flags_to_action": {
      "32": "Ignore",
      "16": "Ignore",
      "128": "Ignore",
      "64": "Ignore"
    }
  },
  "230": {
    "value": 230,
    "name": "AND A,u8",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "16": "Unset",
      "32": "Set",
      "128": "Depend",
      "64": "Unset"
    }
  },
  "231": {
    "value": 231,
    "name": "RST 20h",
    "tcycles": [16, 16],
    "mcycles": [4, 4],
    "length": 1,
    "flags_to_action": {
      "128": "Ignore",
      "16": "Ignore",
      "64": "Ignore",
      "32": "Ignore"
    }
  },
  "232": {
    "value": 232,
    "name": "ADD SP,i8",
    "tcycles": [16, 16],
    "mcycles": [4, 4],
    "length": 2,
    "flags_to_action": {
      "128": "Unset",
      "16": "Depend",
      "64": "Unset",
      "32": "Depend"
    }
  },
  "233": {
    "value": 233,
    "name": "JP HL",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "128": "Ignore",
      "32": "Ignore",
      "16": "Ignore",
      "64": "Ignore"
    }
  },
  "234": {
    "value": 234,
    "name": "LD (u16),A",
    "tcycles": [16, 16],
    "mcycles": [4, 4],
    "length": 3,
    "flags_to_action": {
      "128": "Ignore",
      "32": "Ignore",
      "64": "Ignore",
      "16": "Ignore"
    }
  },
  "235": {
    "value": 235,
    "name": "UNUSED",
    "tcycles": [0, 0],
    "mcycles": [0, 0],
    "length": 1,
    "flags_to_action": {
      "64": "Ignore",
      "32": "Ignore",
      "16": "Ignore",
      "128": "Ignore"
    }
  },
  "236": {
    "value": 236,
    "name": "UNUSED",
    "tcycles": [0, 0],
    "mcycles": [0, 0],
    "length": 1,
    "flags_to_action": {
      "16": "Ignore",
      "32": "Ignore",
      "64": "Ignore",
      "128": "Ignore"
    }
  },
  "237": {
    "value": 237,
    "name": "UNUSED",
    "tcycles": [0, 0],
    "mcycles": [0, 0],
    "length": 1,
    "flags_to_action": {
      "16": "Ignore",
      "32": "Ignore",
      "128": "Ignore",
      "64": "Ignore"
    }
  },
  "238": {
    "value": 238,
    "name": "XOR A,u8",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "32": "Unset",
      "64": "Unset",
      "16": "Unset",
      "128": "Depend"
    }
  },
  "239": {
    "value": 239,
    "name": "RST 28h",
    "tcycles": [16, 16],
    "mcycles": [4, 4],
    "length": 1,
    "flags_to_action": {
      "128": "Ignore",
      "64": "Ignore",
      "32": "Ignore",
      "16": "Ignore"
    }
  },
  "240": {
    "value": 240,
    "name": "LD A,(FF00+u8)",
    "tcycles": [12, 12],
    "mcycles": [3, 3],
    "length": 2,
    "flags_to_action": {
      "32": "Ignore",
      "128": "Ignore",
      "16": "Ignore",
      "64": "Ignore"
    }
  },
  "241": {
    "value": 241,
    "name": "POP AF",
    "tcycles": [12, 12],
    "mcycles": [3, 3],
    "length": 1,
    "flags_to_action": {
      "64": "Depend",
      "128": "Depend",
      "16": "Depend",
      "32": "Depend"
    }
  },
  "242": {
    "value": 242,
    "name": "LD A,(FF00+C)",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 1,
    "flags_to_action": {
      "32": "Ignore",
      "64": "Ignore",
      "16": "Ignore",
      "128": "Ignore"
    }
  },
  "243": {
    "value": 243,
    "name": "DI",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "128": "Ignore",
      "64": "Ignore",
      "16": "Ignore",
      "32": "Ignore"
    }
  },
  "244": {
    "value": 244,
    "name": "UNUSED",
    "tcycles": [0, 0],
    "mcycles": [0, 0],
    "length": 1,
    "flags_to_action": {
      "64": "Ignore",
      "16": "Ignore",
      "128": "Ignore",
      "32": "Ignore"
    }
  },
  "245": {
    "value": 245,
    "name": "PUSH AF",
    "tcycles": [16, 16],
    "mcycles": [4, 4],
    "length": 1,
    "flags_to_action": {
      "16": "Ignore",
      "128": "Ignore",
      "32": "Ignore",
      "64": "Ignore"
    }
  },
  "246": {
    "value": 246,
    "name": "OR A,u8",
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
  "247": {
    "value": 247,
    "name": "RST 30h",
    "tcycles": [16, 16],
    "mcycles": [4, 4],
    "length": 1,
    "flags_to_action": {
      "16": "Ignore",
      "64": "Ignore",
      "32": "Ignore",
      "128": "Ignore"
    }
  },
  "248": {
    "value": 248,
    "name": "LD HL,SP+i8",
    "tcycles": [12, 12],
    "mcycles": [3, 3],
    "length": 2,
    "flags_to_action": {
      "64": "Unset",
      "128": "Unset",
      "32": "Depend",
      "16": "Depend"
    }
  },
  "249": {
    "value": 249,
    "name": "LD SP,HL",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 1,
    "flags_to_action": {
      "128": "Ignore",
      "32": "Ignore",
      "16": "Ignore",
      "64": "Ignore"
    }
  },
  "250": {
    "value": 250,
    "name": "LD A,(u16)",
    "tcycles": [16, 16],
    "mcycles": [4, 4],
    "length": 3,
    "flags_to_action": {
      "64": "Ignore",
      "16": "Ignore",
      "32": "Ignore",
      "128": "Ignore"
    }
  },
  "251": {
    "value": 251,
    "name": "EI",
    "tcycles": [4, 4],
    "mcycles": [1, 1],
    "length": 1,
    "flags_to_action": {
      "32": "Ignore",
      "128": "Ignore",
      "16": "Ignore",
      "64": "Ignore"
    }
  },
  "252": {
    "value": 252,
    "name": "UNUSED",
    "tcycles": [0, 0],
    "mcycles": [0, 0],
    "length": 1,
    "flags_to_action": {
      "64": "Ignore",
      "16": "Ignore",
      "128": "Ignore",
      "32": "Ignore"
    }
  },
  "253": {
    "value": 253,
    "name": "UNUSED",
    "tcycles": [0, 0],
    "mcycles": [0, 0],
    "length": 1,
    "flags_to_action": {
      "64": "Ignore",
      "128": "Ignore",
      "16": "Ignore",
      "32": "Ignore"
    }
  },
  "254": {
    "value": 254,
    "name": "CP A,u8",
    "tcycles": [8, 8],
    "mcycles": [2, 2],
    "length": 2,
    "flags_to_action": {
      "64": "Set",
      "16": "Depend",
      "128": "Depend",
      "32": "Depend"
    }
  },
  "255": {
    "value": 255,
    "name": "RST 38h",
    "tcycles": [16, 16],
    "mcycles": [4, 4],
    "length": 1,
    "flags_to_action": {
      "128": "Ignore",
      "16": "Ignore",
      "32": "Ignore",
      "64": "Ignore"
    }
  }
}"#;
