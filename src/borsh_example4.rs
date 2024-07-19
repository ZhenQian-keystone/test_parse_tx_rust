use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize,Debug)]
#[borsh(crate = "borsh")]
pub struct MultisigCreateArgs {
    /// The authority that can configure the multisig: add/remove members, change the threshold, etc.
    /// Should be set to `None` for autonomous multisigs.
    pub config_authority: Option<Pubkey>,
    /// The number of signatures required to execute a transaction.
    pub threshold: u16,
    /// The members of the multisig.
    pub members: Vec<crate::borsh_example2::Member>,
    /// How many seconds must pass between transaction voting, settlement, and execution.
    pub time_lock: u32,
    /// Memo is used for indexing only.
    pub memo: Option<String>,
}

#[derive(BorshSerialize, BorshDeserialize,Debug)]
#[borsh(crate = "borsh")]
pub struct Member {
    pub key: Pubkey,
    pub permissions: crate::borsh_example2::Permissions,
}
#[derive(BorshSerialize, BorshDeserialize,Debug)]
#[borsh(crate = "borsh")]
pub struct Permissions {
    pub mask: u8,
}
#[derive(BorshSerialize, BorshDeserialize,Debug)]
#[borsh(crate = "borsh")]
pub struct ProposalCreateArgs {
    /// Index of the multisig transaction this proposal is associated with.
    pub transaction_index: u64,
    /// Whether the proposal should be initialized with status `Draft`.
    pub draft: bool
}


pub enum SquadsMultisigInstruction {
    CreateMultisig(MultisigCreateArgs),
    CreateProposal(ProposalCreateArgs),
}


pub fn parse_instruction_data(data:&[u8]) -> Result<SquadsMultisigInstruction> {
    let sig_hash = data[..8];
    let sig_hash_hex = hex::encode(sig_hash);

    let ix_data = data[8..];

}