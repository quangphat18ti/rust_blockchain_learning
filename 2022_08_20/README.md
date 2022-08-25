# Voting
 NEAR Core Contract
Confirm thông tin:

1> Build được file .wasm với build.sh và cargo build --target wasm32-unknown-unknown --release

2> Login: near login

3> Deploy được contract lên near
  + near deploy
  
4> Hiểu được voting contract cách mà voting contract thực thi

5> Sử dụng env::log() để viết log ra màn hình.

6> Vì chỉ có các validator mới có thể sử dụng method vote() nên em đã xây dựng 1 vài methods để can thiệp vào contract:
 + Tuy nhiên việc này không hề đúng với logic của 1 smart contract.
 + Đây chỉ là em thêm vào để thử hoạt động của smart contract.
 
pub fn set_vote_end(&mut self);
pub fn set_vote_start(&mut self);
pub fn get_result(&self);
pub fn check_log(&self);
pub fn add_vote(&mut self, account_id : String, stake:u128);
pub fn get_votes(&self) -> HashMap<String, U128>;

7> Sử dụng các hàm call và view để gọi đến các method của contract
