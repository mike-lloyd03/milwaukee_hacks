use anyhow::{bail, Result};
use serde::Deserialize;

use crate::types::Promotion;

const QUERY: &str = r#"query promotionProducts($itemId: String!, $pageSize: Int) {
  promotionProducts(
    itemId: $itemId
    pageSize: $pageSize
  ) {
    promotions {
      experienceTag
      promotionId
      subExperienceTag
      description {
        longDesc
        shortDesc
        __typename
      }
      eligibilityCriteria {
        itemGroup
        categories {
          categoryId
          name
          maxPurchaseQuantity
          nvalues
          itemIds
          __typename
        }
        itemIds
        minPurchaseAmount
        minPurchaseQuantity
        searchReport {
          pageSize
          startIndex
          totalProducts
          __typename
        }
        __typename
      }
      dates {
        end
        start
        __typename
      }
      reward {
        tiers {
          maxAllowedRewardAmount
          maxPurchaseQuantity
          minPurchaseAmount
          minPurchaseQuantity
          rewardAmountPerItem
          rewardAmountPerOrder
          rewardFixedPrice
          rewardPercent
          __typename
        }
        __typename
      }
    }
    __typename
  }
}
"#;

#[derive(Debug, Deserialize)]
struct Response {
    data: PromotionProducts,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PromotionProducts {
    promotion_products: Promotions,
}

#[derive(Debug, Deserialize)]
struct Promotions {
    promotions: Vec<Promotion>,
}

pub fn get_promo(promo_id: u32) -> Result<Promotion> {
    let variables = serde_json::json!({
            "itemId":promo_id.to_string(),
            "pageSize": 48,
    });

    let body = serde_json::json!({
        "operationName": "promotionProducts",
        "variables": variables,
        "query": QUERY,
    });
    let resp = ureq::post(
        "https://apionline.homedepot.com/federation-gateway/graphql?opname=promotionProducts",
    )
    .header("x-experience-name", "fusion-gm-pip-desktop")
    .header("content-type", "application/json")
    .send_json(body)?
    .body_mut()
    .read_json::<Response>()?;

    match resp
        .data
        .promotion_products
        .promotions
        .into_iter()
        .take(1)
        .next()
    {
        Some(p) => Ok(p),
        None => bail!("Promotion not found"),
    }
}
