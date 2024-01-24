#![feature(if_let_guard)]

use std::fs;

mod data;
// mod instruction;

fn main() {
    let schema = data::parse();
    let mut data = schema.unprefixed.iter().collect::<Vec<_>>();
    data.sort_by_key(|(opcode, _)| opcode.as_byte());

    let res = data
        .iter()
        .map(|(opcode, instruction)| {
            let operands = instruction
                .operands
                .iter()
                .map(|x| match x.immediate {
                    false => format!(
                        "[{}{}]",
                        x.name,
                        if x.increment {
                            "+"
                        } else if x.decrement {
                            "-"
                        } else {
                            ""
                        }
                    ),
                    true => x.name.clone(),
                })
                .enumerate()
                .fold(Vec::with_capacity(2), |mut acc, (i, x)| {
                    if i != 2 {
                        acc.push(x);
                    } else {
                        acc[1] = format!("{}+{x}", acc[1]);
                    }

                    acc
                })
                .join(",");

            return [
                //
                format!("// {} {operands}", instruction.mnemonic),
                format!(
                    "// cycles: {}",
                    instruction.cycles.iter().fold(String::new(), |acc, num| {
                        let prefix = if acc != "" { "," } else { "" };
                        format!("{acc}{prefix}{num}")
                    })
                ),
                format!("{opcode} => self.todo(),"),
            ]
            .join("\n");
        })
        .collect::<Vec<_>>();

    res.iter().for_each(|x| {
        println!("{x}");
    });

    // fs::write("./unprefixed.rs", res.join("\n")).unwrap();
}
