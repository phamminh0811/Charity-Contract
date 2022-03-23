use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Voting{
    pub end_vote_time: Timestamp,
    pub total_voted: u64,
    pub accept_vote: u64,
    pub saving_accept :Vec<MappingVotingAccount>,
    pub saving_decline :Vec<MappingVotingAccount>
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct MappingVotingAccount{
    pub vote_id: VoteId,
    pub account_id: AccountId,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize,PartialEq, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum VoteType {
    Accept,
    Decline
}


impl Voting{
    pub fn new(end_vote_time:Timestamp)-> Self{
        Self{
            end_vote_time: end_vote_time,
            total_voted: 0,
            accept_vote: 0,
            saving_accept: vec![],
            saving_decline: vec![],
        }
    }
}