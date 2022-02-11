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
}

impl Error {
    /// Calls [`near_sdk::env::panic_str()`] with this error's message.
    pub fn panic(&self) -> ! {
        near_sdk::env::panic_str(&self.to_string())
    }
}
