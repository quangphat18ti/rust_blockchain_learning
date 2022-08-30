use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate="near_sdk::serde")]
pub struct Order{
    pub order_id: OrderId,
    pub payer_id: AccountId,
    pub amount: Balance,
    pub is_completed: bool,
    pub is_refund: bool,
    pub created_at: Timestamp,
}

impl Order {
    pub fn new(order_id: OrderId, payer_id: AccountId, amount: Balance) -> Self{
        Self { 
            order_id, 
            payer_id, 
            amount, 
            is_completed: false, 
            is_refund: false, 
            created_at: env::block_timestamp(), 
        }
    }

    pub fn get_amount(&self) -> Balance {
        self.amount
    }
}