use anyhow::{Context, Result};
use serde::Deserialize;

use crate::{requests::STORE_ID, types::Product};

const QUERY: &str = r#"
query productClientOnlyProduct($itemId: String!, $storeId: String) {
  product(itemId: $itemId) {
    identifiers {
      storeSkuNumber
      specialOrderSku
      canonicalUrl
      brandName
      itemId
      productLabel
      productType
      parentId
      modelNumber
      isSuperSku
      sampleId
    }
    itemId
    dataSources
    media {
      images {
        url
        type
        subType
        sizes
        altText
      }
    }
    pricing(storeId: $storeId) {
      value
      original
      preferredPriceFlag
      promotion {
        dates {
          end
          start
        }
        description {
          shortDesc
          longDesc
        }
        experienceTag
        subExperienceTag
        type
        dollarOff
        percentageOff
        promotionTag
        savingsCenter
        savingsCenterPromos
        specialBuySavings
        specialBuyDollarOff
        specialBuyPercentageOff
      }
      conditionalPromotions {
        promotionId
        skuItemGroup
        promotionTags
        eligibilityCriteria {
          itemGroup
          minThresholdVal
          thresholdType
          minPurchaseAmount
          minPurchaseQuantity
          relatedSkusCount
          omsSkus
        }
        reward {
          tiers {
            minThresholdVal
            thresholdType
            rewardVal
            rewardType
            rewardLevel
            maxAllowedRewardAmount
            minPurchaseAmount
            minPurchaseQuantity
            rewardPercent
            rewardAmountPerOrder
            rewardAmountPerItem
            rewardFixedPrice
            maxPurchaseQuantity
          }
        }
        dates {
          start
          end
        }
        description {
          shortDesc
          longDesc
        }
        experienceTag
        subExperienceTag
        nvalues
        brandRefinementId
      }
      alternatePriceDisplay
      alternate {
        bulk {
          pricePerUnit
          thresholdQuantity
          value
        }
        unit {
          caseUnitOfMeasure
          unitsOriginalPrice
          unitsPerCase
          value
        }
      }
      mapAboveOriginalPrice
      mapDetail {
        percentageOff
        dollarOff
        mapPolicy
        mapOriginalPriceViolation
        mapSpecialPriceViolation
      }
      message
      specialBuy
      unitOfMeasure
      clearance {
        value
        dollarOff
        percentageOff
        unitsClearancePrice
      }
    }
  }
}
"#;

#[derive(Debug, Deserialize)]
struct Response {
    data: Data,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Data {
    product: Product,
}

pub async fn get_product(item_id: &str) -> Result<Product> {
    let client = reqwest::Client::new();

    let variables = serde_json::json!({
      "itemId": item_id,
      "storeId": STORE_ID
    });

    let body = serde_json::json!({
        "operationName": "productClientOnlyProduct",
        "variables": variables,
        "query": QUERY,
    });

    let resp_str = client
        .post("https://apionline.homedepot.com/federation-gateway/graphql")
        .header("x-experience-name", "fusion-gm-pip-desktop")
        .header("content-type", "application/json")
        .json(&body)
        .send()
        .await?
        .text()
        .await?;

    let resp: Response =
        serde_json::from_str(&resp_str).context("Failed to parse get_products response")?;

    Ok(resp.data.product)
}
