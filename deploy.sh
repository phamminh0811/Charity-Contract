# near deploy --wasmFile res/charity_contract.wasm --accountId charity.abc.testnet --initFunction new--initArgs '{"owner_id": "abc.testnet"}'

near deploy --wasmFile res/charity_contract.wasm --accountId abc.testnet

# near create-account charity.example-acct.testnet --masterAccount example-acct.testnet --initialBalance 100

# near call charity.example-acct.testnet create_campaign '{"title":"Tu thien lu lut","details":"Giong viec tu thien cua nghe si nhung minh bach hon","receiving_account":"phamminh.testnet","end_date":1648650445000000000,"goal":1000}' --accountId abc.testnet
# near view charity.example-acct.testnet get_campaign_info '{"campaign_id":1}'
# near call charity.example-acct.testnet vote_campaign '{"campaign_id":1, "vote_type":"Accept"}' --accountId example-acct.testnet  --deposit 1
# near call charity.example-acct.testnet confirm_campaign '{"campaign_id":1}' --accountId example-acct.testnet  --deposit 1.1
# near view charity.example-acct.testnet get_voting_info '{"campaign_id":1}'
# near call charity.example-acct.testnet donate_campaign '{"campaign_id":1}' --accountId example-acct.testnet  --deposit 5