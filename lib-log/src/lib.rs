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

pub fn vec_to_string(strings: &[String]) -> String {
    String::from("[") + &strings.join(", ") + "]"
}

pub fn print_vec(strings: &[String]) {
    println!("{}", vec_to_string(strings));
}
