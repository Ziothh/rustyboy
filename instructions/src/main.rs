mod data;
mod instruction;

fn main() {
    let schema = data::parse();
    data::print_opcode_map(&schema.unprefixed);

}

