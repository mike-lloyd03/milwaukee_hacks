use anyhow::Result;
use serde::Deserialize;
use sqlx::{types::Json, SqlitePool};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    #[serde(rename = "itemId")]
    pub id: String,
    pub identifiers: Identifiers,
    pub pricing: Pricing,
    pub media: Media,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Identifiers {
    pub item_id: String,
    pub brand_name: String,
    pub product_type: String,
    pub product_label: String,
    pub canonical_url: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pricing {
    pub value: f32,
    pub original: f32,
    pub promotion: ProductPromotion,
    pub message: Option<String>,
    pub special_buy: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    images: Vec<Image>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub url: String,
    pub sizes: Vec<String>,
    pub sub_type: String,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug)]
pub struct ProductDB {
    pub item_id: String,
    pub brand_name: String,
    pub product_type: String,
    pub product_label: String,
    pub canonical_url: String,
    pub pricing_value: f32,
    pub pricing_original: f32,
    pub promotion_tag: Option<String>,
    pub promotion_type: Option<String>,
    pub promotion_description: Option<String>,
    pub promotion_dollar_off: f32,
    pub promotion_percentage_off: f32,
    pub promotion_savings_center: Option<String>,
    pub promotion_savings_center_promos: Option<String>,
    pub promotion_special_buy_savings: Option<String>,
    pub promotion_special_buy_dollar_off: Option<f32>,
    pub promotion_special_buy_percentage_off: Option<f32>,
    pub pricing_message: Option<String>,
    pub special_buy: Option<String>,
    pub image_primary_url: String,
    pub image_primary_sizes: Json<Vec<String>>,
    pub image_secondary_url: String,
    pub image_secondary_sizes: Json<Vec<String>>,
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
            pricing_value: from_val.pricing.value,
            pricing_original: from_val.pricing.original,
            promotion_tag: from_val.pricing.promotion.promotion_tag,
            promotion_type: from_val.pricing.promotion.r#type,
            promotion_description: from_val.pricing.promotion.description,
            promotion_dollar_off: from_val.pricing.promotion.dollar_off,
            promotion_percentage_off: from_val.pricing.promotion.percentage_off,
            promotion_savings_center: from_val.pricing.promotion.savings_center,
            promotion_savings_center_promos: from_val.pricing.promotion.savings_center_promos,
            promotion_special_buy_savings: from_val.pricing.promotion.special_buy_savings,
            promotion_special_buy_dollar_off: from_val.pricing.promotion.special_buy_dollar_off,
            promotion_special_buy_percentage_off: from_val
                .pricing
                .promotion
                .special_buy_percentage_off,
            pricing_message: from_val.pricing.message,
            special_buy: from_val.pricing.special_buy,
            image_primary_url: primary_image.url,
            image_primary_sizes: primary_image.sizes.into(),
            image_secondary_url: secondary_image.url,
            image_secondary_sizes: secondary_image.sizes.into(),
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
                pricing_value,
                pricing_original,
                promotion_tag,
                promotion_type,
                promotion_description,
                promotion_dollar_off,
                promotion_percentage_off,
                promotion_savings_center,
                promotion_savings_center_promos,
                promotion_special_buy_savings,
                promotion_special_buy_dollar_off,
                promotion_special_buy_percentage_off,
                pricing_message,
                special_buy,
                image_primary_url,
                image_primary_sizes,
                image_secondary_url,
                image_secondary_sizes
            ) values 
            ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23)
            on conflict (item_id)
            do update set
                brand_name = $2,
                product_type = $3,
                product_label = $4,
                canonical_url = $5,
                pricing_value = $6,
                pricing_original = $7,
                promotion_tag = $8,
                promotion_type = $9,
                promotion_description = $10,
                promotion_dollar_off = $11,
                promotion_percentage_off = $12,
                promotion_savings_center = $13,
                promotion_savings_center_promos = $14,
                promotion_special_buy_savings = $15,
                promotion_special_buy_dollar_off = $16,
                promotion_special_buy_percentage_off = $17,
                pricing_message = $18,
                special_buy = $19,
                image_primary_url = $20,
                image_primary_sizes = $21,
                image_secondary_url = $22,
                image_secondary_sizes = $23
            where item_id = $1
            "#,
            self.item_id,
            self.brand_name,
            self.product_type,
            self.product_label,
            self.canonical_url,
            self.pricing_value,
            self.pricing_original,
            self.promotion_tag,
            self.promotion_type,
            self.promotion_description,
            self.promotion_dollar_off,
            self.promotion_percentage_off,
            self.promotion_savings_center,
            self.promotion_savings_center_promos,
            self.promotion_special_buy_savings,
            self.promotion_special_buy_dollar_off,
            self.promotion_special_buy_percentage_off,
            self.pricing_message,
            self.special_buy,
            self.image_primary_url,
            self.image_primary_sizes,
            self.image_secondary_url,
            self.image_secondary_sizes
        ).execute(pool).await?;
        Ok(())
    }
}
