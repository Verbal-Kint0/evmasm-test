use clap::clap_app;
use num_bigint::BigUint;
use rsevmasm::{Disassembly, Instruction};

pub fn disassemble(hex_data: &str) -> Result<(), rsevmasm::DisassemblyError> {
    for (addr, instruction) in Disassembly::from_hex_str(hex_data)?.instructions.iter() {
        match instruction {
            Instruction::Push(arg) => println!(
                "{:#x} PUSH {:#x}",
                addr,
                BigUint::from_bytes_be(arg.as_slice())
            ),
            Instruction::Dup(arg) => println!("{:#x} DUP {:#x}", addr, arg),
            Instruction::Swap(arg) => println!("{:#x} SWAP {:#x}", addr, arg),
            Instruction::Log(arg) => println!("{:#x} LOG {:#x}", addr, arg),
            i => println!("{:#x} {}", addr, format!("{:?}", i).to_uppercase()),
        }
    }

    Ok(())
}

fn main() -> Result<(), rsevmasm::DisassemblyError> {
    let args = clap_app!(app =>
        (version: "0.1")
        (author: "xpdiem")
        (about: "EVM Disassembly PoC")
        (@arg input: -x --hex +required +takes_value "Byte Code Hex String")
        (@arg decompile: -d --decompile "Decompile Input Hex")
    )
    .get_matches();

    if args.is_present("decompile") {
        let hex = args.value_of("input").unwrap();
        disassemble(hex)?;
    }

    Ok(())
}
