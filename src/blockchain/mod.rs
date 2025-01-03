use ethers::{
    prelude::*,
    core::types::{Address, TransactionReceipt, U256},
    abi::Abi,
};
use std::{error::Error, sync::Arc};

pub struct BlockchainService {
    contract: Contract<Provider<Http>>,
}

impl BlockchainService {
    pub async fn new(
        rpc_url: &str,
        contract_address: Address,
        _private_key: &str,
    ) -> Result<Self, Box<dyn Error>> {
        let client = Arc::new(Provider::<Http>::try_from(rpc_url)?);
        let abi: Abi = serde_json::from_str(include_str!("../../contracts/abi.json"))?;
        let contract = Contract::new(contract_address, abi, Arc::clone(&client));

        Ok(Self { contract })
    }

    pub async fn create_course(
        &self,
        course_id: String,
        metadata: String,
        price: U256,
    ) -> Result<TransactionReceipt, Box<dyn Error>> {
        let tx = self.contract
            .method::<_, ()>("createCourse", (course_id, metadata, price))?
            .send()
            .await?
            .await?
            .ok_or("Transaction failed")?;

        Ok(tx)
    }

    pub async fn issue_certificate(
        &self,
        student: Address,
        course_id: String,
    ) -> Result<TransactionReceipt, Box<dyn Error>> {
        let tx = self.contract
            .method::<_, ()>("issueCertificate", (student, course_id))?
            .send()
            .await?
            .await?
            .ok_or("Transaction failed")?;

        Ok(tx)
    }
}