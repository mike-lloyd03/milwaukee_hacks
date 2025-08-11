use crate::now_timestamp;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{types::Json, SqlitePool};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    #[serde(rename = "itemId")]
    pub id: String,
    pub identifiers: Identifiers,
    pub pricing: Pricing,
    pub media: Media,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Identifiers {
    pub item_id: String,
    pub brand_name: Option<String>,
    pub product_type: String,
    pub product_label: String,
    pub canonical_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pricing {
    pub value: f32,
    pub original: f32,
    pub promotion: ProductPromotion,
    pub conditional_promotions: Vec<ConditionalPromotion>,
    pub message: Option<String>,
    pub special_buy: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    images: Vec<Image>,
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub url: String,
    pub sizes: Vec<String>,
    pub sub_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductPromotion {
    pub promotion_tag: Option<String>,
    pub r#type: Option<String>,
    pub description: Option<String>,
    pub dollar_off: f32,
    pub percentage_off: f32,
    pub savings_center: Option<String>,
    pub savings_center_promos: Option<String>,
    pub special_buy_savings: Option<String>,
    pub special_buy_dollar_off: Option<f32>,
    pub special_buy_percentage_off: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConditionalPromotion {
    pub promotion_id: Option<u32>,
}

#[derive(Debug)]
pub struct ProductDB {
    pub item_id: String,
    pub brand_name: Option<String>,
    pub product_type: String,
    pub product_label: String,
    pub canonical_url: String,
    pub pricing: Json<Pricing>,
    pub image_primary_url: String,
    pub image_primary_sizes: Json<Vec<String>>,
    pub image_secondary_url: String,
    pub image_secondary_sizes: Json<Vec<String>>,
    pub updated_at: u32,
}

impl From<Product> for ProductDB {
    fn from(from_val: Product) -> Self {
        let mut primary_image = Image::default();
        let mut secondary_image = Image::default();

        for image in from_val.media.images {
            if image.sub_type == "PRIMARY" {
                primary_image = image
            } else if image.sub_type == "SECONDARY" {
                secondary_image = image
            }
        }

        Self {
            item_id: from_val.id,
            brand_name: from_val.identifiers.brand_name,
            product_type: from_val.identifiers.product_type,
            product_label: from_val.identifiers.product_label,
            canonical_url: from_val.identifiers.canonical_url,
            pricing: from_val.pricing.into(),
            image_primary_url: primary_image.url,
            image_primary_sizes: primary_image.sizes.into(),
            image_secondary_url: secondary_image.url,
            image_secondary_sizes: secondary_image.sizes.into(),
            updated_at: now_timestamp(),
        }
    }
}

impl ProductDB {
    pub async fn create(&self, pool: &SqlitePool) -> Result<()> {
        sqlx::query!(
            r#"insert into products (
                item_id,
                brand_name,
                product_type,
                product_label,
                canonical_url,
                pricing,
                image_primary_url,
                image_primary_sizes,
                image_secondary_url,
                image_secondary_sizes,
                updated_at
            ) values 
            ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            on conflict (item_id)
            do update set
                brand_name = $2,
                product_type = $3,
                product_label = $4,
                canonical_url = $5,
                pricing = $6,
                image_primary_url = $7,
                image_primary_sizes = $8,
                image_secondary_url = $9,
                image_secondary_sizes = $10,
                updated_at = $11
            where item_id = $1
            "#,
            self.item_id,
            self.brand_name,
            self.product_type,
            self.product_label,
            self.canonical_url,
            self.pricing,
            self.image_primary_url,
            self.image_primary_sizes,
            self.image_secondary_url,
            self.image_secondary_sizes,
            self.updated_at
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn delete_all_before(pool: &SqlitePool, timestamp: u32) -> Result<()> {
        sqlx::query!(
            "delete from products where updated_at is null or updated_at < ?",
            timestamp
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}
