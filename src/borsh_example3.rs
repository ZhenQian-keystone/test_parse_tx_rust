use borsh::{BorshDeserialize, BorshSerialize};
use sha2::Digest;
use solana_program::pubkey::Pubkey;


// https://solana.fm/tx/5J9aU3DQ48b9kP7DfCbbP4zDYnLwAXD8xNNeVv3uSMAx6DPu853fercc8RTuvSCXUxUiUgQrzC7E7EKbWSwhJTWa
#[derive(BorshSerialize, BorshDeserialize,Debug)]
#[borsh(crate = "borsh")]
pub struct ProposalCreateArgs {
    /// Index of the multisig transaction this proposal is associated with.
    pub transaction_index: u64,
    /// Whether the proposal should be initialized with status `Draft`.
    pub draft: bool
}

#[cfg(test)]
mod tests {
    use super::*;
    use borsh::from_slice;
    use crate::util::{sighash, SIGHASH_GLOBAL_NAMESPACE};

    #[test]
    fn test_sighash() {
        let sig_hash = sighash(SIGHASH_GLOBAL_NAMESPACE, "proposal_create");
        assert_eq!(
            "dc3c49e01e6c4f9f",
            hex::encode(sig_hash)
        )
    }
    #[test]
    fn test_encode_args(){
        let ix_data = "010000000000000000";// splited 8bytes
        let proposal_create_args = from_slice::<ProposalCreateArgs>(&hex::decode(ix_data).unwrap()).unwrap();

        assert_eq!(proposal_create_args.transaction_index, 1);
        assert_eq!(proposal_create_args.draft, false);
    }

}