use crate::non_fungible_token::token::TokenId;
use near_sdk::{AccountId, Promise};

pub trait NonFungibleTokenReceiver {
    /// Take some action after receiving a non-fungible token
    ///
    /// Requirements:
    /// * Contract MUST restrict calls to this function to a set of whitelisted NFT
    ///   contracts
    ///
    /// Arguments:
    /// * `sender_id`: the sender of `nft_transfer_call`
    /// * `previous_owner_id`: the account that owned the NFT prior to it being
    ///   transfered to this contract, which can differ from `sender_id` if using
    ///   Approval Management extension
    /// * `token_id`: the `token_id` argument given to `nft_transfer_call`
    /// * `msg`: information necessary for this contract to know how to process the
    ///   request. This may include method names and/or arguments.
    ///
    /// Returns true if token should be returned to `sender_id`
    fn nft_on_transfer(
        &mut self,
        sender_id: AccountId,
        previous_owner_id: AccountId,
        token_id: TokenId,
        msg: String,
    ) -> Promise;
}
