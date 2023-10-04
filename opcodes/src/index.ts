import opcodeModes from './opcodes.json';
import path from 'path';
import fs from 'fs/promises';

const OUT_FILE = path.resolve(__dirname, '../dist/parsed-opcodes.rs');

const contents: string[] = [];

type Mode = keyof typeof opcodeModes;

const jsonToPrettyInline = (data: any) => JSON.stringify(data, null, 2).replaceAll('\n', '').replaceAll('    ', '');

const parseMode = (mode: Mode): string[] => Object
  .entries(opcodeModes[mode])
  .map(([opcode, value]) => parseOpcode(opcode as any, value))
  .flat();

const parseOpcode = (
  opcode: keyof typeof opcodeModes[Mode],
  data: (typeof opcodeModes[Mode])[keyof typeof opcodeModes[Mode]]
): string[] => {

  return [
    `/// Instruction: ${data.mnemonic} ${data.bytes}bytes ${data.immediate ? '(immediate)' : ''}`,
    `/// Operands: ${jsonToPrettyInline(data.operands)}`,
    `/// Flags: ${jsonToPrettyInline(data.flags)}`,
    `${opcode} => Instruction::${data.mnemonic},`,
  ];
}

(Object.keys(opcodeModes) as Array<Mode>)
  .forEach(mode => {
    contents.push(`\tpub fn try_from_opcode_${mode}(byte: u8, program: &mut Program) -> Result<Self, String> {`);
    contents.push("\t\tmatch byte {");

    contents.push(...parseMode(mode).map(line => `\t\t\t${line}`))

    contents.push("\n\n}");
    contents.push("\n}");
  });

await fs.writeFile(
  OUT_FILE,
  [
    "/* NOTE: THIS FILE HAS BEEN GENERATED. DO NOT EDIT. */",
    "",
    "impl Instruction {",
    ...contents,
    "}",
  ]
    .map(line => line.replaceAll("\t", "    "))
    .join("\n")
);
