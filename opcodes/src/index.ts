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
    `Instruction: ${data.mnemonic} ${data.immediate ? '(immediate)' : ''}`,
    `{ bytes: ${data.bytes}, cycles: ${data.cycles.length === 1 ? data.cycles : JSON.stringify(data.cycles)} }`,
    // `Bytes: ${data.bytes}`,
    // `Cycles: ${data.cycles}`,
    `Operands: ${data.operands.map(x => `\n    ${x.name} ${x.immediate ? '(immediate)' : ''}`)}`,
    `Flags: Z N H C\n` +
    `       ${data.flags.Z} ${data.flags.N} ${data.flags.H} ${data.flags.C}`,
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

