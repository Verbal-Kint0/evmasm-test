use thiserror::Error;
use rsevmasm::DisassemblyError;
use move_binary_format::errors::PartialVMError;
use hex::FromHexError;


#[derive(Error, Debug)]
pub enum DisasmError {
    #[error("{0}")]
    Evm(#[from] DisassemblyError),
    #[error("{0}")]
    Hex(#[from] FromHexError),
    #[error("{0}")]
    Move(#[from] PartialVMError)
}
