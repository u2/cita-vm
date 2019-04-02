use super::err;
use super::interpreter;
use super::opcodes;
use numext_fixed_uint::U256;
use numext_fixed_hash::{H160, H256};

pub trait DataProvider {
    fn get_balance(&self, address: &H160) -> U256;

    fn add_refund(&mut self, address: &H160, n: u64);
    fn sub_refund(&mut self, address: &H160, n: u64);
    fn get_refund(&self, address: &H160) -> u64;

    fn get_code_size(&self, address: &H160) -> u64;
    fn get_code(&self, address: &H160) -> Vec<u8>;
    fn get_code_hash(&self, address: &H160) -> H256;

    fn get_block_hash(&self, number: &U256) -> H256;

    fn get_storage(&self, address: &H160, key: &H256) -> H256;
    fn set_storage(&mut self, address: &H160, key: H256, value: H256);
    fn get_storage_origin(&self, address: &H160, key: &H256) -> H256;
    fn set_storage_origin(&mut self, address: &H160, key: H256, value: H256);

    fn selfdestruct(&mut self, address: &H160, refund_address: &H160) -> bool;
    fn sha3(&self, input: &[u8]) -> H256;
    // is_empty returns whether the given account is empty. Empty
    // is defined according to EIP161 (balance = nonce = code = 0).
    fn is_empty(&self, address: &H160) -> bool;

    // call is a low-level function for
    //   OpCode::CALL
    //   OpCode::CALLCODE
    //   OpCode::DELEGATECALL
    //   OpCode::STATICCALL
    //   OpCode::CREATE
    //   OpCode::CREATE2
    fn call(
        &self,
        opcode: opcodes::OpCode,
        params: interpreter::InterpreterParams,
    ) -> (Result<interpreter::InterpreterResult, err::Error>);
}
