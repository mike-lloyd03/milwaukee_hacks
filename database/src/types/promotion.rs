use anyhow::Result;
use serde::Deserialize;
use sqlx::SqlitePool;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Promotion {
    pub promotion_id: String,
    pub experience_tag: String,
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

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EligibilityCriterion {
    pub item_group: String,
    pub categories: Vec<String>,
    pub item_ids: Vec<String>,
    pub min_purchase_amount: Option<f32>,
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

#[derive(Debug, Default, Clone, Deserialize)]
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

#[derive(Debug)]
pub struct PromotionDB {
    pub promotion_id: String,
    pub experience_tag: String,
    pub sub_experience_tag: String,
    pub long_description: String,
    pub short_description: String,
    pub start_date: String,
    pub end_data: String,
    pub item_group: String,
    pub categories: sqlx::types::Json<Vec<String>>,
    pub item_ids: sqlx::types::Json<Vec<String>>,
    pub eligibility_min_purchase_amount: Option<f32>,
    pub eligibility_min_purchase_quantity: Option<f32>,
    pub max_allowed_reward_amount: Option<f32>,
    pub max_purchase_quantity: Option<f32>,
    pub min_purchase_amount: Option<f32>,
    pub min_purchase_quantity: Option<f32>,
    pub reward_amount_per_item: Option<f32>,
    pub reward_amount_per_order: Option<f32>,
    pub reward_fixed_price: Option<f32>,
    pub reward_percent: Option<f32>,
}

impl From<Promotion> for PromotionDB {
    fn from(from_val: Promotion) -> Self {
        let eligibility_criteria = from_val
            .eligibility_criteria
            .first()
            .cloned()
            .unwrap_or_default();

        let reward = from_val.reward.tiers.first().cloned().unwrap_or_default();

        Self {
            promotion_id: from_val.promotion_id,
            experience_tag: from_val.experience_tag,
            sub_experience_tag: from_val.sub_experience_tag,
            long_description: from_val.description.long_desc,
            short_description: from_val.description.short_desc,
            start_date: from_val.dates.start,
            end_data: from_val.dates.end,
            item_group: eligibility_criteria.item_group,
            categories: eligibility_criteria.categories.into(),
            item_ids: eligibility_criteria.item_ids.into(),
            eligibility_min_purchase_amount: eligibility_criteria.min_purchase_amount,
            eligibility_min_purchase_quantity: eligibility_criteria.min_purchase_quantity,
            max_allowed_reward_amount: reward.max_allowed_reward_amount,
            max_purchase_quantity: reward.max_purchase_quantity,
            min_purchase_amount: reward.min_purchase_amount,
            min_purchase_quantity: reward.min_purchase_quantity,
            reward_amount_per_item: reward.reward_amount_per_item,
            reward_amount_per_order: reward.reward_amount_per_order,
            reward_fixed_price: reward.reward_fixed_price,
            reward_percent: reward.reward_percent,
        }
    }
}

impl PromotionDB {
    pub async fn create(&self, pool: &SqlitePool) -> Result<()> {
        sqlx::query!(
            r#"insert into promotions (
                promotion_id,
                experience_tag,
                sub_experience_tag,
                long_description,
                short_description,
                start_date,
                end_data,
                item_group,
                categories,
                item_ids,
                eligibility_min_purchase_amount,
                eligibility_min_purchase_quantity,
                max_allowed_reward_amount,
                max_purchase_quantity,
                min_purchase_amount,
                min_purchase_quantity,
                reward_amount_per_item,
                reward_amount_per_order,
                reward_fixed_price,
                reward_percent
            )
            values
            ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20)
            on conflict (promotion_id)
            do update set
                experience_tag = $2,
                sub_experience_tag = $3,
                long_description = $4,
                short_description = $5,
                start_date = $6,
                end_data = $7,
                item_group = $8,
                categories = $9,
                item_ids = $10,
                eligibility_min_purchase_amount = $11,
                eligibility_min_purchase_quantity = $12,
                max_allowed_reward_amount = $13,
                max_purchase_quantity = $14,
                min_purchase_amount = $15,
                min_purchase_quantity = $16,
                reward_amount_per_item = $17,
                reward_amount_per_order = $18,
                reward_fixed_price = $19,
                reward_percent = $20
            where promotion_id = $1
            "#,
            self.promotion_id,
            self.experience_tag,
            self.sub_experience_tag,
            self.long_description,
            self.short_description,
            self.start_date,
            self.end_data,
            self.item_group,
            self.categories,
            self.item_ids,
            self.eligibility_min_purchase_amount,
            self.eligibility_min_purchase_quantity,
            self.max_allowed_reward_amount,
            self.max_purchase_quantity,
            self.min_purchase_amount,
            self.min_purchase_quantity,
            self.reward_amount_per_item,
            self.reward_amount_per_order,
            self.reward_fixed_price,
            self.reward_percent,
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}
