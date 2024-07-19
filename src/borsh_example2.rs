use borsh::{BorshDeserialize, BorshSerialize};
use sha2::Digest;
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
    pub members: Vec<Member>,
    /// How many seconds must pass between transaction voting, settlement, and execution.
    pub time_lock: u32,
    /// Memo is used for indexing only.
    pub memo: Option<String>,
}

#[derive(BorshSerialize, BorshDeserialize,Debug)]
#[borsh(crate = "borsh")]
pub struct Member {
    pub key: Pubkey,
    pub permissions: Permissions,
}
#[derive(BorshSerialize, BorshDeserialize,Debug)]
#[borsh(crate = "borsh")]
pub struct Permissions {
    pub mask: u8,
}

// Namespace for calculating instruction sighash signatures for any instruction
// not affecting program state.
pub const SIGHASH_GLOBAL_NAMESPACE: &str = "global";

// We don't technically use sighash, because the input arguments aren't given.
// Rust doesn't have method overloading so no need to use the arguments.
// However, we do namespace methods in the preeimage so that we can use
// different traits with the same method name.
pub fn sighash(namespace: &str, name: &str) -> [u8; 8] {
    let preimage = format!("{}:{}", namespace, name);

    let mut sighash = [0u8; 8];
    sighash.copy_from_slice(&crate::hash::hash(preimage.as_bytes()).to_bytes()[..8]);
    sighash
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;
    use borsh::from_slice;

    #[test]
    fn test_sighash() {
        let sig_hash = sighash(SIGHASH_GLOBAL_NAMESPACE, "multisig_create");
        assert_eq!(
            "7a4d509f54585ac5",
            hex::encode(sig_hash)
        )
    }
    #[test]
    fn test_encode_args(){
        let authority = None;
        let threshold = 2;
        let time_lock = 0;
        let memo = Some("{\"n\":\"TESTMULTISIG\",\"d\":\"TEST MULTI SIG\",\"i\":\"\"}".to_string());
        let members = vec![
            Member {
                key: Pubkey::from_str("3w1iMvjKGxpbGaaSekNUsZBcVKERg2BCsUZMGrjcTMsj").unwrap(),
                permissions: Permissions {
                    mask: 7,
                }
            },
            Member {
                key: Pubkey::from_str("HJ3oKYB44fyHEieTGB8syyDXGmYZUr6Dd8dDzYVMEgD9").unwrap(),
                permissions: Permissions {
                    mask: 7,
                }
            },
            Member {
                key: Pubkey::from_str("4ySuE5EjYqjZoBA87Nu4jRqsBTHTKyppRqp24SMv3WFw").unwrap(),
                permissions: Permissions {
                    mask: 7,
                }
            },
            Member {
                key: Pubkey::from_str("HjtLu2LRtJPXzoJtR1BjWcC6PtorzwyJ6MjnHbZAWgfN").unwrap(),
                permissions: Permissions {
                    mask: 7,
                }
            },
            Member {
                key: Pubkey::from_str("AuvGDMSBA66jhnbN7iSDS2g7iz6Uwo7BKDi91wL6hoVX").unwrap(),
                permissions: Permissions {
                    mask: 7,
                }
            },
        ];

        let args = MultisigCreateArgs {
            config_authority: authority,
            threshold,
            members,
            time_lock,
            memo,
        };

        let encoded = borsh::to_vec(&args).unwrap();
        let encoded_hex = hex::encode(encoded.clone());
        // https://github.com/coral-xyz/anchor/blob/852fcc77beb6302474a11e0f8e6f1e688021be36/ts/packages/anchor/src/coder/borsh/discriminator.ts
        // default discriminator is 8 byte

        // use base58 to decode the 7a4d509f54585ac5
        // https://solana.stackexchange.com/questions/5840/what-is-first-8-bytes-of-anchor-generated-instruction-data
        // https://github.com/coral-xyz/anchor/blob/37cc99c2b6c24e2bf03ff9d58ed451deb586ffef/lang/syn/src/codegen/program/dispatch.rs#L34
        // https://blog.labeleven.dev/anatomy-of-solana-program-invocations-using-anchor
        // https://solana.stackexchange.com/questions/3135/how-do-you-find-a-matching-idl-instruction-using-the-anchor-instruction-discrimi/3185#3185
        // https://github.com/coral-xyz/anchor/blob/2a07d841c65d6f303aa9c2b0c68a6e69c4739aab/lang/syn/src/codegen/program/common.rs#L9-L23
        // Sha256("<namespace>:<rust-identifier>")[..8],
        assert_eq!(
            "7a4d509f54585ac5",
            hex::encode(sighash(SIGHASH_GLOBAL_NAMESPACE, "multisig_create"))
        );

        // split 8 bytes
        assert_eq!(
            "000200050000002b8d8b3addd92759f55b840c2852f5bd50aee3552fe987ee0d4fe24b9043df8e07f219076b2850cbb770c807661d874e09a7224de024d6579e43cc1df392a12244073b08df2ea93b9fc9ecb8a965773e18c6c8c4f66696dda8eb6ea61ca420700c5607f8b770467b0eaae4e081f7e4b66db848c91d63a4f3de46092fe5ccff4427dec50793479bb7ee58060b82e4bdba7ec1a026bacabbb96a8d1c72f21f2a1dd98ad8de070000000001300000007b226e223a22544553544d554c5449534947222c2264223a2254455354204d554c544920534947222c2269223a22227d",
            encoded_hex
        );
        // 000200050000002b8d8b3addd92759f55b840c2852f5bd50aee3552fe987ee0d4fe24b9043df8e07f219076b2850cbb770c807661d874e09a7224de024d6579e43cc1df392a12244073b08df2ea93b9fc9ecb8a965773e18c6c8c4f66696dda8eb6ea61ca420700c5607f8b770467b0eaae4e081f7e4b66db848c91d63a4f3de46092fe5ccff4427dec50793479bb7ee58060b82e4bdba7ec1a026bacabbb96a8d1c72f21f2a1dd98ad8de070000000001300000007b226e223a22544553544d554c5449534947222c2264223a2254455354204d554c544920534947222c2269223a22227d

        // decode args
        let multi_sig_create_args = from_slice::<MultisigCreateArgs>(&encoded).unwrap();

        assert_eq!(
            2,
            multi_sig_create_args.threshold
        )
    }

}