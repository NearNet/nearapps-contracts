use near_sdk::Gas;

/// Expected Gas that the best-effort logging operation could
/// take.
///
/// Note:  This is currently a somehow high number because
/// it's not defined whether some signature verification should
/// take place during the logging operation.
pub const GAS_FOR_BEST_EFFORT_LOG: Gas = Gas(15_000_000_000_000);

/// Expected Gas that the callback logging operation could
/// take.
///
/// Note:  This is currently a somehow high number because
/// it's not defined whether some signature verification should
/// take place and how high the result bytes forwarding
/// operations could cost.
pub const GAS_FOR_ON_LOG: Gas = Gas(35_000_000_000_000);

/// Tag information required by NearApps.
#[derive(near_sdk::serde::Serialize, near_sdk::serde::Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct NearAppsTags {
    pub app_id: String,
    pub action_id: near_sdk::json_types::U64,
    pub user_id: near_sdk::AccountId,
}

/// One type of container for [`NearAppsTags`].
///
/// This is useful for testing if `nearapps_tags` is included
/// in serialized arguments.
#[derive(near_sdk::serde::Serialize, near_sdk::serde::Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct NearAppsTagsContained {
    pub nearapps_tags: NearAppsTags,
}

/// Generic container for [`NearAppsTags`].
///
/// This is useful to deserialize [`NearAppsTags`] while also deserializing
/// the rest of the parameters.
#[derive(near_sdk::serde::Serialize, near_sdk::serde::Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct NearAppsTagsContainer<T> {
    pub nearapps_tags: NearAppsTags,
    #[serde(flatten)]
    pub inner: T,
}

impl std::fmt::Display for NearAppsTags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            near_sdk::serde_json::to_string(&self).expect("failed to serialize log")
        )
    }
}

impl NearAppsTags {
    pub fn new(
        app_id: impl Into<String>,
        action_id: impl Into<near_sdk::json_types::U64>,
        user_id: impl AsRef<str>,
    ) -> Self
where {
        Self {
            app_id: app_id.into(),
            action_id: action_id.into(),
            user_id: user_id.as_ref().parse().unwrap(),
        }
    }

    pub fn dummy(action_id: u64) -> Self {
        Self::new("dummy-app-id", action_id, "dummy-user-id")
    }

    pub fn log_str(&self) {
        near_sdk::env::log_str(&self.to_string())
    }
}

#[near_sdk::ext_contract(ext_log)]
pub trait ExtLog {
    /// Logs nearapps tags and forwards the first call result.
    ///
    /// Should be used as a callback.
    fn on_log_result(nearapps_tags: NearAppsTags);

    /// Emits a nearapps log.
    fn log(nearapps_tags: NearAppsTags);
}

pub trait NearAppsAccount {
    fn nearapps_account(&self) -> near_sdk::AccountId;

    // TODO: stress and check maximum gas requirement.
    //
    /// Makes a new call to the nearapps account for logging.
    fn log(&self, nearapps_tags: NearAppsTags) -> near_sdk::Promise {
        ext_log::log(
            nearapps_tags,
            self.nearapps_account(),
            0,
            GAS_FOR_BEST_EFFORT_LOG,
        )
    }

    // TODO: stress and check maximum gas requirement.
    //
    /// Makes a new call to the nearapps account for logging,
    /// which will forward the first result as the return type.
    ///
    /// Should be used as a callback for returning a value.
    fn on_log_result(&self, nearapps_tags: NearAppsTags) -> near_sdk::Promise {
        ext_log::on_log_result(nearapps_tags, self.nearapps_account(), 0, GAS_FOR_ON_LOG)
    }
}

pub fn vec_to_string(strings: &[String]) -> String {
    String::from("[") + &strings.join(", ") + "]"
}

pub fn print_vec(strings: &[String]) {
    println!("{}", vec_to_string(strings));
}
