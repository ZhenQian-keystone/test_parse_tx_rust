use borsh::{from_slice, BorshDeserialize, BorshSerialize};
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::pubkey::Pubkey;
use std::io::Read;

#[derive(BorshSerialize, BorshDeserialize)]
#[borsh(crate = "borsh")]
pub struct MyInstruction {
    pub lamports: u64,
}
pub fn create_instruction(
    program_id: &Pubkey,
    from: &Pubkey,
    to: &Pubkey,
    lamports: u64,
) -> Instruction {
    let instr = MyInstruction { lamports };

    Instruction::new_with_borsh(
        *program_id,
        &instr,
        vec![AccountMeta::new(*from, true), AccountMeta::new(*to, false)],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solana_instruction_borsh_encode() {
        let program_id = Pubkey::new_unique();
        let from = Pubkey::new_unique();
        let to = Pubkey::new_unique();
        let lamports = 111;
        let instr = create_instruction(&program_id, &from, &to, lamports);
        let encoded_instr = instr.data;
        let borsh_data = borsh::to_vec(&lamports).unwrap();
        assert_eq!(encoded_instr, borsh_data);
        let decoded_instr = from_slice::<MyInstruction>(&borsh_data).unwrap();
        assert_eq!(decoded_instr.lamports, lamports);
    }
}
