#![allow(unused)]

use core::panic;

use near_sdk::{env, near_bindgen, AccountId, Balance, BorshStorageKey, Timestamp, Promise, PromiseOrValue, PanicOnDefault};
use near_sdk::json_types::U128;
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::collections::LookupMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

pub type OrderId = String;

/// import file 
mod order;
use order::*;

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
#[near_bindgen]
struct EcomerceContract{
    owner_id: AccountId,
    orders: LookupMap<OrderId, Order>
}

#[derive(BorshDeserialize, BorshSerialize, BorshStorageKey)]
enum StorageKey{
    OrdersKey,
}

#[near_bindgen]
impl EcomerceContract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self{
        Self { 
            owner_id, 
            orders: LookupMap::new(StorageKey::OrdersKey),
        }
    }

    pub fn create_order(&mut self, order_id:OrderId, payer_id: AccountId, amount: U128) {
        assert_eq!(env::predecessor_account_id(), self.owner_id, "Only Owner can add Order");
        let order = Order::new(order_id.clone(), payer_id, amount.0);
        self.orders.insert(&order_id, &order);
    }

    #[payable]
    pub fn pay_order(&mut self, order_id: OrderId) -> PromiseOrValue<U128> {
        let mut order = self.get_order(order_id.clone());
        assert_eq(env::signer_account_id(), self.pay_order, "ERROR_DIFFERENT_PAYER");

        /// check enough deposit
        assert!(env::attached_deposit() >= order.get_amount(), "ERROR_DEPOSIT_NOT_ENOUGH");

        /// kiểm tra đơn hàng đã thanh toán chưa
        assert!(!order.is_completed, "ORDER IS PAYED BY {}", self.pay_order);

        order.is_completed = true;
        self.orders.insert(&order_id, &order);

        /// Trả tiền thừa
        if env::attached_deposit() > order.amount {
            let promise = Promise::new(env::signer_account_id()).transfer(env::attached_deposit() - order.amount);
            PromiseOrValue::Promise(promise)
        }
        else {
            PromiseOrValue::Value(U128(0))
        }
    }

    // /// get order by order_id
    pub fn get_order(&self, order_id: OrderId) -> Order {
        self.orders.get(&order_id).expect("NOT_FOUND_ORDER_ID")
    }

    // Refund lại tiền cho user
    /**
     * - Chỉ được thực hiện bởi owner của contract
     * - Kiểm tra đơn hàng đã được complete và refund chưa
     * - Thực hiện trả tiền cho user
     * - Cập nhật trạng thái đơn
     */
    pub fn refund(&mut self, order_id: OrderId) -> PromiseOrValue<U128> {
        /// check owner
        assert_eq!(env::predecessor_account_id(), self.owner_id);

        let mut order = self.get_order(order_id.clone());
        assert!(order.is_completed && !order.is_refund);

        let mut return_value:PromiseOrValue<U128>;
        if order.amount > 0 {
            let promise = Promise::new(order.payer_id.clone()).transfer(order.amount);
            return_value = PromiseOrValue::Promise(promise);
        }
        else {
            return_value = PromiseOrValue::Value(U128(0));
        }
        
        order.is_refund = true;
        self.orders.insert(&order_id, &order);
        return_value
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use near_sdk::test_utils::{VMContextBuilder, accounts};
    use near_sdk::{testing_env, MockedBlockchain};

    fn get_context(is_view: bool) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
        .current_account_id(accounts(0))
        .signer_account_id(accounts(0))
        .predecessor_account_id(accounts(0))
        .is_view(is_view);

        builder
    }

    #[test]
    fn test_pay_order() {
        
    
        let mut context = get_context(false);
        let alice: AccountId = accounts(0);

        context.account_balance(1000)
        .predecessor_account_id(alice.clone())
        .attached_deposit(1000)
        .signer_account_id(alice.clone());

        testing_env!(context.build());

        let mut contract = EcomerceContract::new(alice.clone());
        let order_amount = U128(1000);
        contract.create_order("order_1".to_owned(), alice.clone(), order_amount.0);
        contract.pay_order("order_1".to_owned());

        let order = contract.get_order("order_1".to_owned());

        // Test
        assert_eq!(order.order_id, "order_1".to_owned());
        assert_eq!(order.amount, order_amount.0);
        assert_eq!(order.payer_id, alice);
        assert!(order.is_completed);
    }

    #[test]
    #[should_panic(expected = "ERROR_DEPOSIT_NOT_ENOUGH")]
    fn test_pay_order_with_lack_balance() {
        let mut context = get_context(false);
        let alice: AccountId = accounts(0);

        context.account_balance(1000)
        .predecessor_account_id(alice.clone())
        .attached_deposit(1000)
        .signer_account_id(alice.clone());

        testing_env!(context.build());

        let mut contract = EcomerceContract::new(alice.clone());
        let order_amount = U128(3000);
        contract.create_order("order_1".to_owned(), alice.clone(), order_amount.0);
        contract.pay_order("order_1".to_owned());
    }
}

