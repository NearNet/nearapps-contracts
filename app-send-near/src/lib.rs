// TODO: require user registration, which can be automatic, for
// tracking it's funds. If the funds that failed to get send is too
// little for registering the user (if it's not already registered),
// then the fund should be absorbed by the contract.

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{
    env, ext_contract, near_bindgen, AccountId, Balance, BorshStorageKey, Gas, PanicOnDefault,
    Promise,
};
use near_units::parse_gas;
use nearapps_log::{NearAppsAccount, NearAppsTags};
use nearapps_near_ext::{ensure, types::JBalance, OrPanicStr};

pub mod error;

use error::Error;

const GAS_ON_SEND: Gas = Gas(parse_gas!("30 Tgas") as u64);

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct SendNear {
    owner: AccountId,
    deposits: LookupMap<AccountId, Balance>,
    nearapps_logger: AccountId,
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    UserDeposits,
}

#[ext_contract(ext_self)]
trait OnSend {
    /// After a send operation,  
    /// If successful, will log and return `true`.  
    /// Otherwise if failed, will deposit `amount` back to the `sender`'s
    /// balance.
    fn on_send(sender: AccountId, amount: JBalance, nearapps_tags: NearAppsTags);
}

// TODO: have the attached deposit cover for the account storage, in case
// it's needed.
#[near_bindgen]
impl SendNear {
    #[init]
    pub fn new(owner: AccountId, nearapps_logger: AccountId) -> Self {
        ensure(!env::state_exists(), Error::AlreadyInitialized);

        Self {
            owner,
            deposits: LookupMap::new(StorageKey::UserDeposits),
            nearapps_logger,
        }
    }

    pub fn get_owner(&mut self) -> AccountId {
        self.owner.clone()
    }

    pub fn change_owner(&mut self, new_owner: AccountId) {
        self.assert_owner();
        self.owner = new_owner;
    }

    /// Sends the attached amount to `receiver`. On failure`*`,
    /// the attached amount is deposited on the sender's account.
    ///
    /// `*` Some failures will result in the fund being absorved by this
    /// contract. Check [`Error`] for more information.
    #[payable]
    pub fn send_attached_logged(
        &mut self,
        receiver: AccountId,
        nearapps_tags: NearAppsTags,
    ) -> Promise {
        let sender = env::predecessor_account_id();
        let amount = env::attached_deposit();

        let send = Promise::new(receiver).transfer(amount);
        let on_send = ext_self::on_send(
            //
            sender,
            JBalance(amount),
            nearapps_tags,
            env::current_account_id(),
            0,
            GAS_ON_SEND,
        );

        send.then(on_send)
    }

    /// Sends `amount` to `receiver`, based on the attached amount and the
    /// sender's balance.  
    /// Remaining amounts are (re)deposited to the sender's account.
    ///
    /// On failure`*`, the amount that was sent to the `receiver`
    /// is deposited back onto the sender's account.
    ///
    /// `*` Some failures will result in the fund being absorved by this
    /// contract. Check [`Error`] for more information.
    #[payable]
    pub fn send_logged(
        &mut self,
        receiver: AccountId,
        amount: JBalance,
        nearapps_tags: NearAppsTags,
    ) -> Promise {
        let sender = env::predecessor_account_id();
        let attached = env::attached_deposit();

        // check if needs to withdraw from deposits
        #[allow(clippy::comparison_chain)]
        if amount.0 > attached {
            let balance = self.deposits.get(&sender).unwrap_or_default();

            // on failure, the attached fund is returned to
            // the predecessor
            ensure(balance + attached >= amount.0, Error::InsufficientFunds);

            let deposit_back = balance + attached - amount.0;
            match deposit_back {
                // no longer has funds
                0 => {
                    // removes data to save space
                    self.deposits.remove(&sender);
                }

                // has some funds to be deposited back
                n => {
                    self.deposits.insert(&sender, &n);
                }
            }
        } else if attached > amount.0 {
            let balance = self.deposits.get(&sender).unwrap_or_default();
            let deposit_back = balance + attached - amount.0;
            self.deposits.insert(&sender, &deposit_back);
        };

        let send = Promise::new(receiver).transfer(amount.0);
        let on_send = ext_self::on_send(
            //
            sender,
            amount,
            nearapps_tags,
            env::current_account_id(),
            0,
            GAS_ON_SEND,
        );

        send.then(on_send)
    }

    /// Gets the balance of a user.
    pub fn get_balance(&self, user: AccountId) -> JBalance {
        JBalance(self.deposits.get(&user).or_panic_str(Error::MissingUser))
    }

    /// Withdraws everything that a user may have on their balance.
    pub fn withdraw_logged(&mut self, nearapps_tags: NearAppsTags) -> Promise {
        let user = env::predecessor_account_id();
        let amount = self
            .deposits
            //
            .remove(&user)
            .or_panic_str(Error::MissingUser);
        let send = Promise::new(user.clone()).transfer(amount);
        let on_send = ext_self::on_send(
            //
            user,
            JBalance(amount),
            nearapps_tags,
            env::current_account_id(),
            0,
            GAS_ON_SEND,
        );

        send.then(on_send)
    }

    /// After a send operation,  
    /// If successful, will log and return `true`.  
    /// Otherwise if failed, will deposit `amount` back to the `sender`'s
    /// balance.
    #[private]
    pub fn on_send(
        &mut self,
        sender: AccountId,
        amount: JBalance,
        nearapps_tags: NearAppsTags,
    ) -> bool {
        use near_sdk::PromiseResult;

        // unexpected results count will cause funds to be absorbed by
        // this contract (it will not be deposited on the sender's balance)
        ensure(env::promise_results_count() == 1, Error::WrongResultCount);

        match env::promise_result(0) {
            PromiseResult::Successful(_) => {
                // best-effort call for nearapps log
                let _ = self.log(nearapps_tags);

                true
            }
            PromiseResult::NotReady | PromiseResult::Failed => {
                let previous = self
                    .deposits
                    //
                    .get(&sender)
                    .unwrap_or_default();
                self.deposits.insert(&sender, &(previous + amount.0));

                false
            }
        }
    }
}

pub trait Owner {
    fn assert_owner(&self);
}

impl Owner for SendNear {
    fn assert_owner(&self) {
        ensure(env::predecessor_account_id() == self.owner, Error::NotOwner)
    }
}

impl nearapps_log::NearAppsAccount for SendNear {
    fn nearapps_account(&self) -> near_sdk::AccountId {
        self.nearapps_logger.clone()
    }
}
