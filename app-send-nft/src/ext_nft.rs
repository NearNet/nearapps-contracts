pub mod standard {
    use crate::NftUserAccountId;
    use near_contract_standards::non_fungible_token as nft;
    use near_sdk::ext_contract;

    #[ext_contract]
    trait Transfer {
        fn nft_transfer(
            receiver_id: NftUserAccountId,
            token_id: nft::TokenId,
            approval_id: Option<u64>,
            memo: Option<String>,
        );
        fn nft_transfer_call(
            receiver_id: NftUserAccountId,
            token_id: nft::TokenId,
            approval_id: Option<u64>,
            memo: Option<String>,
            msg: String,
        ) -> PromiseOrValue<bool>;
    }
}

pub mod nearapps {
    use crate::NftUserAccountId;
    use near_contract_standards::non_fungible_token as nft;
    use near_sdk::ext_contract;
    use nearapps_log::NearAppsTags;

    #[ext_contract]
    trait Transfer {
        fn nft_transfer_logged(
            receiver_id: NftUserAccountId,
            token_id: nft::TokenId,
            approval_id: Option<u64>,
            memo: Option<String>,
            nearapps_tags: NearAppsTags,
        );
        fn nft_transfer_call_logged(
            receiver_id: NftUserAccountId,
            token_id: nft::TokenId,
            approval_id: Option<u64>,
            memo: Option<String>,
            msg: String,
            nearapps_tags: NearAppsTags,
        ) -> PromiseOrValue<bool>;
    }
}
