use near_sdk::{env,near_bindgen};
use near_sdk::{AccountId,Timestamp, Promise, Balance,PanicOnDefault};
use near_sdk::collections::{UnorderedMap, LookupMap};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};


pub type CampaignId = u64;
pub type VoteId = u64;

pub use crate::campaign::*;
pub use crate::voting::*;

mod campaign;
mod voting;

pub const THREE_DAY: Timestamp = 259_200_000_000_000;
pub const ONE_YOCTO_NEAR: u128 = 1_000_000_000_000_000_000_000_000;

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct CharityContract{
    owner_id: AccountId,
    campaign: LookupMap<CampaignId,Campaign>,
    voting: UnorderedMap<CampaignId,Voting>,
    current_campaign_id : CampaignId
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum StorageKey{
    CampaignKey,
    VotingKey,
}

#[near_bindgen]
impl CharityContract{

    #[init]
    pub fn new(owner_id: AccountId)-> Self{
        Self{
            owner_id: owner_id,
            campaign: LookupMap::new(StorageKey::CampaignKey.try_to_vec().unwrap()),
            voting: UnorderedMap::new(StorageKey::VotingKey.try_to_vec().unwrap()),
            current_campaign_id: 0
        }
    }

    pub fn create_campaign(&mut self,
                        title: String, 
                        details: String,
                        receiving_account:AccountId, 
                        end_date: Timestamp,
                        goal:u64 
                        ){
        let current_time = env::block_timestamp();
        env::log(format!("Current time: {}", current_time).as_bytes());
        assert!(current_time<end_date,"Close time must be after current time");

        let current_campaign_id = self.current_campaign_id +1;

        let campaign = Campaign::new(title, details,receiving_account,end_date,goal);
        self.campaign.insert(&current_campaign_id, &campaign);

        let voting = Voting::new(current_time+ THREE_DAY);
        self.voting.insert(&current_campaign_id, &voting);

        self.current_campaign_id = current_campaign_id;

    }

    #[payable]
    pub fn vote_campaign(&mut self,campaign_id:CampaignId, vote_type: VoteType){
        require_deposit_one_yocto_near();
        
        let campaign = self.get_campaign_info(campaign_id);
        assert!(campaign.is_on_voting, "Campaign isn't on voting time");
        assert!(campaign.is_active, "Campaign is deactivated");

        let current_account_id = env::current_account_id();
        let mut voting = self.voting.get(&campaign_id).unwrap();
        
        voting.total_voted += 1;
        let current_vote_id =voting.total_voted as usize;

        let mapping_vote_account = MappingVotingAccount{
            vote_id : voting.total_voted,
            account_id : current_account_id,
        }; 

        match vote_type {
            VoteType::Accept => {
                voting.accept_vote += 1;
                voting.saving_accept.insert(current_vote_id-1,mapping_vote_account);
                self.voting.insert(&campaign_id,&voting);
            },
            VoteType::Decline => {
                voting.saving_decline.insert(current_vote_id-1,mapping_vote_account);
                self.voting.insert(&campaign_id,&voting);
            }
        };

        env::log(b"Successfully vote this campaign");
    }

    #[payable]
    pub fn confirm_campaign(&mut self, campaign_id: u64){
        let mut campaign = self.get_campaign_info(campaign_id); 
        assert!(campaign.is_active, "Campaign is deactivated");
        assert_eq!(campaign.receiving_account, env::signer_account_id(), "Campaign must be confirm by campaign_id");
        // let current_time = env::block_timestamp();
        // assert!(current_time > voting.end_vote_time,"Vote time is not end");

        let voting = self.voting.get(&campaign_id).unwrap();
        assert!(voting.accept_vote/voting.total_voted > 1/2, "Decline vote is not enough");
        assert!(env::attached_deposit()>=(voting.accept_vote as u128 *ONE_YOCTO_NEAR*11/10),
                 "Attached deposit is not enough, must deposit {} NEAR", 
                 voting.accept_vote*11/10);

        for accept_info in voting.saving_accept.iter() {
                let accept_account = accept_info.account_id.clone();
                Promise::new(accept_account).transfer(ONE_YOCTO_NEAR*11/10);
        }

        campaign.is_on_voting = false;
        campaign.is_open = true;
        self.campaign.insert(&campaign_id,&campaign);
    }

    #[payable]
    pub fn deactivate_campaign(&mut self,campaign_id: u64) {
        let mut campaign = self.get_campaign_info(campaign_id); 
        assert!(campaign.is_active, "Campaign has already been deactivated");
        let owner_id =self.owner_id.clone();
        let campaign_receive_account = campaign.receiving_account.clone();
        if env::signer_account_id() == owner_id {
            env::log(b"Deactivated campaign");
            let voting = self.voting.get(&campaign_id).unwrap();
            // let current_time = env::block_timestamp();
            // assert!(current_time > voting.end_vote_time,"Vote time is not end");
            let decline_vote = voting.total_voted - voting.accept_vote;
            assert!(decline_vote/voting.total_voted > 1/2, "Decline vote is not enough");
            for decline_info in voting.saving_decline.iter() {
                let decline_account = decline_info.account_id.clone();
                Promise::new(decline_account).transfer(ONE_YOCTO_NEAR*11/10);
            }
                  
        } else if env::signer_account_id() ==campaign_receive_account {

            campaign.is_active = false;
            env::log(b"Deactivated campaign");
            self.campaign.insert(&campaign_id,&campaign);

        }else {
            panic!("Account in not permitted to deactivate campaign");
        }
    }
    pub fn get_campaign_info(&self, campaign_id: u64)-> Campaign{
        let campaign = self.campaign.get(&campaign_id).expect("No campaign found");
        campaign
    }
    pub fn get_voting_info(&self, campaign_id: u64)-> Voting{
        let voting = self.voting.get(&campaign_id).expect("No campaign found");
        voting
    }

    #[payable]
    pub fn donate_campaign(&mut self, campaign_id: u64){
        let deposit = env::attached_deposit(); 

        let mut campaign = self.get_campaign_info(campaign_id); 
        assert!(campaign.is_open, "Campaign not open");
        assert!(campaign.is_active, "Campaign is deactivated");

        let current_time = env::block_timestamp();
        assert!(current_time <= campaign.end_date, "Campaign is close");

        campaign.donated += deposit;
        self.campaign.insert(&campaign_id, &campaign);

    }

    #[payable]
    pub fn receive_donation(&mut self, campaign_id: u64){
        require_deposit_one_yocto_near();
        let mut campaign = self.get_campaign_info(campaign_id); 
        let receive_account = env::signer_account_id();
        assert_eq!(receive_account,campaign.receiving_account,"Receiving account not match"); 

        Promise::new(receive_account).transfer(campaign.donated);
        campaign.is_active = false;
        self.campaign.insert(&campaign_id,&campaign);
    }
}


fn require_deposit_one_yocto_near(){
    assert_eq!(env::attached_deposit()
                ,ONE_YOCTO_NEAR
                ,"Require deposit of exactly 1 yoctoNEAR")
}