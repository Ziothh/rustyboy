use std::{collections::HashMap, fs, path};

pub fn parse() -> Schema {
    let data_path = path::Path::new(env!("CARGO_MANIFEST_DIR")).join("./opcodes.json");
    let json = fs::read_to_string(data_path).unwrap();
    return serde_json::from_str(&json).unwrap();
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, std::hash::Hash, serde::Deserialize)]
#[serde(try_from = "String")]
pub struct Opcode(u8);

impl std::fmt::Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#04X}", self.0)
    }
}

impl TryFrom<String> for Opcode {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        return match value.strip_prefix("0x") {
            Some(hex_str) => match u8::from_str_radix(hex_str, 16) {
                Ok(code) => Ok(Opcode(code)),
                Err(err) => Err(format!("{err:#?}")),
            },
            None => Err("Missing 0x hex prefix".to_owned()),
        };
    }
}

pub type OpcodeMap = HashMap<Opcode, InstructionRecord>;

pub fn print_opcode_map(map: &OpcodeMap) {
    let mut data = map.iter().collect::<Vec<_>>();
    data.sort_by_key(|x| x.0 .0);

    for (opcode, instruction) in data {
        let operands = instruction
            .operands
            .iter()
            .map(|x| match x.immediate {
                false => format!("[{}]", x.name),
                true => x.name.clone(),
            })
            .collect::<Vec<_>>()
            .join(",");

        println!("{opcode} => {} {operands}", instruction.mnemonic);
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct Schema {
    pub unprefixed: OpcodeMap,
    pub prefixed: OpcodeMap,
}

#[derive(Debug, serde::Deserialize)]
pub struct OperandDescription {
    pub name: String,
    pub immediate: bool,
    pub bytes: Option<usize>,
}

#[derive(Debug, serde::Deserialize)]
#[allow(non_snake_case)]
pub struct FlagDescriptions {
    pub Z: char,
    pub N: char,
    pub H: char,
    pub C: char,
}

#[derive(Debug, serde::Deserialize)]
pub struct InstructionRecord {
    pub mnemonic: String,
    pub bytes: usize,
    /// [usize] | [usize, usize]
    pub cycles: Vec<usize>,
    /// [src] | [dest, src]
    pub operands: Vec<OperandDescription>,
    pub immediate: bool,
    pub flags: FlagDescriptions,
}
