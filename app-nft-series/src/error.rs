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
    /// A call that was supposed to be made by the owner was made
    /// by a different predecessor.
    #[strum(serialize = "ERR_NOT_OWNER")]
    NotOwner,

    /// A standard minting operation tried to use the
    /// token-series delimiter on the token's name, which
    /// must not be allowed.
    #[strum(serialize = "ERR_NFT_TOKEN_ID_WITH_DELIMITER")]
    TokenIdWithSeriesDelimiter,

    #[strum(serialize = "ERR_NFT_SERIES_MISSING")]
    MissingSeries,

    #[strum(serialize = "ERR_NFT_SERIES_MAX_CAPACITY")]
    SeriesMaxCapacity,

    #[strum(serialize = "ERR_NFT_SERIES_NOT_MINTABLE")]
    SeriesNotMintable,

    #[strum(serialize = "ERR_NFT_SERIES_NOT_ENOUGH_CAPACITY")]
    SeriesNotEnoughtCapacity,

    /// For security reasons, the nft transfers require an attached
    /// amount of one yocto.
    ///
    /// For the nft contracts, this avoids limited accounts from directly
    /// and explicitly  invoking the transfers. Because of this, we need
    /// to attach this one yocto, but in order to not have to manage this
    /// internally on the contract, it simply also requires that amount to
    /// be attached, and forwards it to the transfer method call.
    #[strum(serialize = "ERR_NFT_SERIES_REQUIRED_ONE_YOCTO")]
    OneYoctoNearRequired,

    #[strum(serialize = "ERR_NFT_SERIES_SERIES_ALREADY_CREATED")]
    SeriesAlreadyCreated,
}

impl Error {
    /// Calls [`near_sdk::env::panic_str()`] with this error's message.
    pub fn panic(&self) -> ! {
        near_sdk::env::panic_str(&self.to_string())
    }
}
