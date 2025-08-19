use crate::now_timestamp;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Promotion {
    pub promotion_id: String,
    // This field is added
    pub name: Option<String>,
    // This field is the itemId used in the GraphQL query for fetching this promo. It is the
    // product ID
    pub item_id: Option<String>,
    pub experience_tag: String,
    pub sub_experience_tag: String,
    pub description: Description,
    pub eligibility_criteria: Vec<EligibilityCriterion>,
    pub dates: Dates,
    pub reward: Reward,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Description {
    pub long_desc: Option<String>,
    pub short_desc: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct EligibilityCriterion {
    pub item_group: String,
    pub categories: Vec<String>,
    pub item_ids: Vec<String>,
    pub min_purchase_amount: Option<f32>,
    pub min_purchase_quantity: Option<f32>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Dates {
    pub end: String,
    pub start: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Reward {
    pub tiers: Vec<RewardTier>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
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
    pub name: Option<String>,
    pub item_id: Option<String>,
    pub experience_tag: String,
    pub sub_experience_tag: String,
    pub long_description: Option<String>,
    pub short_description: String,
    pub start_date: String,
    pub end_date: String,
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
    pub reward_tiers: sqlx::types::Json<Vec<RewardTier>>,
    pub eligibility_criteria: sqlx::types::Json<Vec<EligibilityCriterion>>,
    pub updated_at: u32,
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
            name: from_val.name,
            item_id: from_val.item_id,
            experience_tag: from_val.experience_tag,
            sub_experience_tag: from_val.sub_experience_tag,
            long_description: from_val.description.long_desc,
            short_description: from_val.description.short_desc,
            start_date: from_val.dates.start,
            end_date: from_val.dates.end,
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
            reward_tiers: from_val.reward.tiers.into(),
            eligibility_criteria: from_val.eligibility_criteria.into(),
            updated_at: now_timestamp(),
        }
    }
}

impl PromotionDB {
    pub async fn create(&self, pool: &SqlitePool) -> Result<()> {
        sqlx::query!(
            r#"insert into promotions (
                promotion_id,
                name,
                item_id,
                experience_tag,
                sub_experience_tag,
                long_description,
                short_description,
                start_date,
                end_date,
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
                reward_percent,
                reward_tiers,
                eligibility_criteria,
                updated_at
            )
            values
            ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25)
            on conflict (promotion_id)
            do update set
                name = $2,
                item_id = $3,
                experience_tag = $4,
                sub_experience_tag = $5,
                long_description = $6,
                short_description = $7,
                start_date = $8,
                end_date = $9,
                item_group = $10,
                categories = $11,
                item_ids = $12,
                eligibility_min_purchase_amount = $13,
                eligibility_min_purchase_quantity = $14,
                max_allowed_reward_amount = $15,
                max_purchase_quantity = $16,
                min_purchase_amount = $17,
                min_purchase_quantity = $18,
                reward_amount_per_item = $19,
                reward_amount_per_order = $20,
                reward_fixed_price = $21,
                reward_percent = $22,
                reward_tiers = $23,
                eligibility_criteria = $24,
                updated_at = $25
            where promotion_id = $1
            "#,
            self.promotion_id,
            self.name,
            self.item_id,
            self.experience_tag,
            self.sub_experience_tag,
            self.long_description,
            self.short_description,
            self.start_date,
            self.end_date,
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
            self.reward_tiers,
            self.eligibility_criteria,
            self.updated_at
        )
            .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn delete_all_before(pool: &SqlitePool, timestamp: u32) -> Result<()> {
        sqlx::query!(
            "delete from promotions where updated_at is null or updated_at < ?",
            timestamp
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}
