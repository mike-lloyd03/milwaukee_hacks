use anyhow::{bail, Result};
use serde::Deserialize;

use crate::types::Promotion;

const QUERY: &str = r#"query promotionProducts($itemId: String!, $pageSize: Int) {
  promotionProducts(itemId: $itemId, pageSize: $pageSize) {
    promotions {
      experienceTag
      promotionId
      subExperienceTag
      description {
        longDesc
        shortDesc
      }
      eligibilityCriteria {
        itemGroup
        categories {
          categoryId
          name
          maxPurchaseQuantity
          nvalues
          itemIds
        }
        itemIds
        minPurchaseAmount
        minPurchaseQuantity
        searchReport {
          pageSize
          startIndex
          totalProducts
        }
      }
      dates {
        end
        start
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
        }
      }
    }
  }
}"#;

#[derive(Debug, Deserialize)]
struct Response {
    data: Data,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Data {
    promotion_products: Option<PromotionProducts>,
}

#[derive(Debug, Deserialize)]
struct PromotionProducts {
    promotions: Vec<Promotion>,
}

/// Fetches any promotions for the given product item_id
pub fn get_promos_for_product(item_id: &str) -> Result<Vec<Promotion>> {
    let variables = serde_json::json!({
            "itemId":item_id,
            "pageSize": 48,
    });

    let body = serde_json::json!({
        "operationName": "promotionProducts",
        "variables": variables,
        "query": QUERY,
    });

    let resp: Response = ureq::post(
        "https://apionline.homedepot.com/federation-gateway/graphql?opname=promotionProducts",
    )
    .header("x-experience-name", "fusion-gm-pip-desktop")
    .header("content-type", "application/json")
    .send_json(body)?
    .body_mut()
    .read_json()?;

    if let Some(promotion_products) = resp.data.promotion_products {
        Ok(promotion_products.promotions)
    } else {
        bail!("No promotions for product")
    }
}
