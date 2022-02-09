use strum_macros::Display;

/// User-facing errors.
///
/// This maps error variants into error messages.  
/// If a user makes use of some interface incorrectly,
/// this should be used with [`near_sdk::env::panic_str()`].
///
/// Otherwise, if some internal error occurred such that it would
/// indicate an internal bug, then `[panic!()]` should be used
/// as it prints line code information that would be useful for
/// debugging and fixing the problem.
#[derive(Debug, Display)]
pub enum Error {
    #[strum(serialize = "ERR_SEND_NFT_ALREADY_INITIALIZED")]
    AlreadyInitialized,

    /// A call that was supposed to be made by the owner was made
    /// by a different predecessor.
    #[strum(serialize = "ERR_SEND_NFT_NOT_OWNER")]
    NotOwner,

    /// When the owner is trying to invoke a command when it's
    /// not supposed to.
    #[strum(serialize = "ERR_SEND_NFT_MUST_NOT_BE_OWNER")]
    MustNotBeOwner,

    /// When trying to add a new Nft (and it's protocol),
    /// and the Nft was already included.
    #[strum(serialize = "ERR_SEND_NFT_NFT_ALREADY_INCLUDED")]
    NftProtocolAlreadyIncluded,

    /// When trying to manipulate/remove a Nft (and it's protocol),
    /// and the Nft is missing.
    #[strum(serialize = "ERR_SEND_NFT_NFT_NOT_INCLUDED")]
    NftProtocolNotIncluded,

    /// The operation required the NearAppsTags, but this was missing.
    #[strum(serialize = "ERR_SEND_NFT_MISSING_NEARAPPS_TAGS")]
    NearAppsTagsMissing,

    /// The Nft Token was already owned.
    #[strum(serialize = "ERR_SEND_NFT_TOKEN_ALREADY_OWNED")]
    NftTokenAlreadyOwned,

    #[strum(serialize = "ERR_SEND_NFT_TOKEN_ALREADY_OWNED_BY_USER")]
    UserAlreadyOwnedTheNftToken,

    /// The nft contract is not registered, or has an unknown protocol.
    #[strum(serialize = "ERR_SEND_NFT_UNKOWN_PROTOCOL")]
    UnkownProtocol,

    // TODO: if the memo is a json object, it's possible to create
    // a json Value containing everything, including the nearapps tags,
    // and send everything - but this is still restrictive and hould
    // require some extra gas
    //
    /// A call is trying to use a memo to make a standard nft transfer
    /// a token, but the memo will already be used by the nearapps tags.
    #[strum(serialize = "ERR_SEND_NFT_MEMO_NOT_ALLOWED")]
    MemoNotAllowed,

    #[strum(serialize = "ERR_SEND_NFT_MISSING_TOKEN_ID")]
    MissingTokenId,

    /// When a token is received, it's status gets to be on
    /// "standby". If it fails to get sent, it also returns
    /// to the "standby" status.
    #[strum(serialize = "ERR_SEND_NFT_TOKEN_NOT_ON_STANDBY")]
    TokenNotOnSending,

    /// When trying to send the token, it must first be on standby.
    #[strum(serialize = "ERR_SEND_NFT_TOKEN_NOT_ON_STANDBY")]
    TokenNotOnStandby,

    #[strum(serialize = "ERR_SEND_NFT_NOT_TOKEN_OWNER")]
    NotTokenOwner,

    #[strum(serialize = "ERR_SEND_NFT_MISSING_USER")]
    MissingUser,

    /// When a Nft contract is not yet registered for a user.
    ///
    /// The user needs to register both himself and each Nft
    /// contract that it wants to use.
    #[strum(serialize = "ERR_SEND_NFT_CONTRACT_DISABLED_FOR_USER")]
    NftDisabledForUser,

    /// During a user removal, if they still have enabled
    /// NFT contracts, they should first be disabled in order
    /// to remove the user from the contract.
    #[strum(serialize = "ERR_SEND_NFT_USER_HAS_ENABLED_NFT")]
    NftsStillEnabledForUser,

    /// For security reasons, the nft transfers require an attached
    /// amount of one yocto.
    ///
    /// For the nft contracts, this avoids limited accounts from directly
    /// and explicitly  invoking the transfers. Because of this, we need
    /// to attach this one yocto, but in order to not have to manage this
    /// internally on the contract, it simply also requires that amount to
    /// be attached, and forwards it to the transfer method call.
    #[strum(serialize = "ERR_SEND_NFT_MISSING_ONE_YOCTO")]
    OneYoctoNearRequired,

    /// Trying to register a user that's already registered.
    #[strum(serialize = "ERR_SEND_NFT_USER_ALREADY_REGISTERED")]
    UserAlreadyRegistered,

    /// Trying to enable a NFT contract for a user, where it's already
    /// enabled.
    #[strum(serialize = "ERR_SEND_NFT_NFT_CONTRACT_ALREADY_ENABLED")]
    NftContractAlreadyEnabled,

    /// When trying to disable a nft contract for a user,
    /// first that user cannot have any tokens for that nft
    /// contract.
    #[strum(serialize = "ERR_SEND_NFT_NFT_TOKENS_STILL_OWNED_BY_USER")]
    NftTokensStillOwnedByUser,

    /// When trying to disable a nft contract in overall,
    /// first that nft contract cannot have any tokens owned
    /// by any user.
    #[strum(serialize = "ERR_SEND_NFT_NFT_TOKENS_STILL_OWNED_BY_USERS")]
    NftTokensStillOwnedByUsers,

    #[strum(serialize = "ERR_SEND_NFT_JSON_ERROR_FOR_NEARAPPS_TAGS")]
    JsonErrorForNearAppsTags,

    /// The `limit` should be greater than zero.
    #[strum(serialize = "ERR_SEND_NFT_LIMIT_ZERO")]
    LimitAtZero,

    /// The `receiver` of a transfer must not be the send-nft
    /// contract itself.
    #[strum(serialize = "ERR_SEND_NFT_SELF_RECEIVER")]
    SelfReceiver,

    /// It's unknown whether the Nft had success in a transfer
    /// operation.
    #[strum(serialize = "ERR_SEND_NFT_NFT_UNKNOWN_SUCCESS")]
    NftContractUnknownSuccess,
}

impl Error {
    /// Calls [`near_sdk::env::panic_str()`] with this error's message.
    pub fn panic(&self) -> ! {
        near_sdk::env::panic_str(&self.to_string())
    }
}
