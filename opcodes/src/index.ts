import opcodeModes from './opcodes.json';
import path from 'path';
import fs from 'fs/promises';

const OUT_DIR = '../dist';


type Mode = keyof typeof opcodeModes;

const jsonToPrettyInline = (data: any) => JSON
  .stringify(data, null, 2)
// .replaceAll('\n', '')
// .replaceAll('    ', '');

const parseMode = (mode: Mode): string[] => Object
  .entries(opcodeModes[mode])
  .sort(([oa, a], [ob, b]) => {
    const dif = a.mnemonic.localeCompare(b.mnemonic);
    if (dif !== 0) return dif;

    const bytes = a.bytes - b.bytes
    if (bytes !== 0) return bytes;

    return parseInt(oa, 16) - parseInt(ob, 16)
  })
  .map(([opcode, value]) => parseOpcode(opcode as any, value))
  .flat();

const parseOpcode = (
  opcode: keyof typeof opcodeModes[Mode],
  data: (typeof opcodeModes[Mode])[keyof typeof opcodeModes[Mode]]
): string[] => {
  const comments = [
    `Instruction: ${data.mnemonic} ${data.bytes} byte${data.bytes === 1 ? '' : 's'} ${data.immediate ? '(immediate)' : ''}`,
    `Operands: ${jsonToPrettyInline(data.operands)}`,
    `Flags: ${jsonToPrettyInline(data.flags)}`,
  ]

  return [
    ...comments.map(x => x.split("\n").map(y => `// ${y}`)).flat(),
    `${opcode} => Ok(Instruction::${data.mnemonic}),\n`,
  ];
}

Promise.all(
  (Object.keys(opcodeModes) as Array<Mode>)
    .map(async (mode) => await fs.writeFile(
      path.resolve(__dirname, OUT_DIR, `parsed-opcodes-${mode}.rs`),
      [
        "/* NOTE: THIS FILE HAS BEEN GENERATED. DO NOT EDIT. */",
        "",
        "impl Instruction {",
        `\tpub fn try_from_opcode_${mode}(byte: u8, program: &mut Program) -> Result<Self, String> {`,
        "\t\tmatch byte {",
        ...parseMode(mode).map(line => `\t\t\t${line}`),
        "\n\n}",
        "\n}",
        "}",
      ]
        .map(line => line.replaceAll("\t", "    "))
        .join("\n")
    ))
);

