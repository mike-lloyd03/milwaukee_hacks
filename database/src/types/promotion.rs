use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Promotion {
    pub experience_tag: String,
    pub promotion_id: String,
    pub sub_experience_tag: String,
    pub description: Description,
    pub eligibility_criteria: Vec<EligibilityCriterion>,
    pub dates: Dates,
    pub reward: Reward,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Description {
    pub long_desc: String,
    pub short_desc: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EligibilityCriterion {
    pub item_group: String,
    pub categories: Vec<String>,
    pub item_ids: Vec<String>,
    pub min_purchase_amount: f32,
    pub min_purchase_quantity: Option<f32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dates {
    pub end: String,
    pub start: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reward {
    pub tiers: Vec<RewardTier>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RewardTier {
    pub max_allowed_reward_amount: Option<f32>,
    pub max_purchase_quantity: Option<f32>,
    pub min_purchase_amount: Option<f32>,
    pub min_purchase_quantity: Option<f32>,
    pub reward_amount_per_item: Option<f32>,
    pub reward_amount_per_order: Option<f32>,
    pub reward_fixed_price: Option<f32>,
    pub reward_percent: Option<f32>,
}
