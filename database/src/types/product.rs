use serde::Deserialize;

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

#[derive(Debug, Deserialize)]
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
