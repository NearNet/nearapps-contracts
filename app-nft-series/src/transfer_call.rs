use crate::error::Error;
use near_contract_standards::non_fungible_token as nft;
use near_sdk::{env, require, AccountId, Balance, Gas, PromiseOrValue};
use near_units::parse_near;
use nearapps_near_ext::ensure;

const GAS_FOR_RESOLVE_TRANSFER: Gas = Gas(5_000_000_000_000);
const GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(45_000_000_000_000 + GAS_FOR_RESOLVE_TRANSFER.0);

const NO_DEPOSIT: Balance = 0;

/// Identical to [`nft::NonFungibleToken::nft_transfer_call`].
pub fn nft_transfer_call(
    token: &mut nft::NonFungibleToken,
    receiver_id: AccountId,
    token_id: nft::TokenId,
    approval_id: Option<u64>,
    memo: Option<String>,
    msg: String,
) -> PromiseOrValue<bool> {
    ensure(
        env::attached_deposit() == parse_near!("1 yN"),
        Error::OneYoctoNearRequired,
    );
    require!(
        env::prepaid_gas() > GAS_FOR_NFT_TRANSFER_CALL + GAS_FOR_RESOLVE_TRANSFER,
        "More gas is required"
    );

    let sender_id = env::predecessor_account_id();
    let (old_owner, old_approvals) =
        token.internal_transfer(&sender_id, &receiver_id, &token_id, approval_id, memo);
    // Initiating receiver's call and the callback
    nft::core::ext_receiver::nft_on_transfer(
        sender_id,
        old_owner.clone(),
        token_id.clone(),
        msg,
        receiver_id.clone(),
        0,
        env::prepaid_gas() - GAS_FOR_NFT_TRANSFER_CALL,
    )
    .then(nft::core::ext_self::nft_resolve_transfer(
        old_owner,
        receiver_id,
        token_id,
        old_approvals,
        env::current_account_id(),
        NO_DEPOSIT,
        GAS_FOR_RESOLVE_TRANSFER,
    ))
    .into()
}
