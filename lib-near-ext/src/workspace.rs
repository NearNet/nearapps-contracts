use async_trait::async_trait;

#[async_trait(?Send)]
pub trait Call {
    async fn call_with_json<MethodName: ToString, Args>(
        &self,
        signer: &workspaces::types::InMemorySigner,
        contract_id: &workspaces::AccountId,
        method_name: MethodName,
        args: Args,
        deposit: u128,
        gas: u64,
    ) -> Result<workspaces::network::CallExecutionDetails, anyhow::Error>
    where
        Args: near_sdk::serde::Serialize;
}

#[async_trait(?Send)]
impl<Network> Call for workspaces::Worker<Network>
where
    Network: workspaces::network::NetworkClient,
{
    async fn call_with_json<MethodName: ToString, Args>(
        &self,
        signer: &workspaces::types::InMemorySigner,
        contract_id: &workspaces::AccountId,
        method_name: MethodName,
        args: Args,
        deposit: u128,
        gas: u64,
    ) -> Result<workspaces::network::CallExecutionDetails, anyhow::Error>
    where
        Args: near_sdk::serde::Serialize,
    {
        use workspaces::network::CallExecutionDetails;

        let method_name = method_name.to_string();
        let contract_id = contract_id.clone();
        let gas = gas;
        let deposit = deposit;
        let args = near_sdk::serde_json::to_string(&args).unwrap();
        println!(
            "near call {contract} {method_name} '{args}' --accountId {signer} --gas {gas} --depositYocto {deposit}",
            contract = &contract_id,
            method_name = &method_name,
            args = &args,
            signer = &signer.inner().account_id,
            gas = gas,
            deposit = deposit
        );

        self.client()
            .call(
                signer,
                contract_id,
                method_name,
                args.into_bytes(),
                Some(gas),
                Some(deposit),
            )
            .await
            .map(CallExecutionDetails::from)
    }
}
