// This file is autogenerated with https://github.com/acheroncrypto/native-to-mainstay

use mainstay_lang::prelude::*;

declare_id!("11111111111111111111111111111111");

#[program]
pub mod spl_memo {
    use super::*;

    pub fn add_memo(ctx: Context<AddMemo>, memo: String) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AddMemo<'info> {
    // All memo accounts are optional and required to be signers
}
