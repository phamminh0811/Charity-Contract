use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Campaign{
    pub title: String,
    pub details: String,
    pub receiving_account: AccountId,
    pub end_date: Timestamp,
    pub goal: u64,
    pub donated: Balance,
    pub is_on_voting: bool,
    pub is_open: bool,
    pub is_active: bool,
}

impl Campaign {

    pub fn new(title: String, details: String,receiving_account:AccountId, end_date: Timestamp,goal:u64) -> Self {
        Self {
            title: title,
            details: details,
            receiving_account: receiving_account,
            end_date: end_date,
            goal: goal,
            donated: 0,
            is_on_voting: true,
            is_open: false,
            is_active: true
        }
    }
}

