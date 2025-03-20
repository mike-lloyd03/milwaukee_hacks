use anyhow::{bail, Result};
use serde::Deserialize;

use crate::types::Product;

const QUERY: &str = r#"query promotionProductsItems(
  $storeId: String
  $pageSize: Int
  $itemIds: [String]
) {
  searchModel(
    storeId: $storeId
    itemIds: $itemIds
  ) {
    products(pageSize: $pageSize) {
      itemId
      dataSources
      identifiers {
        brandName
        productType
        productLabel
        canonicalUrl
        itemId
        specialOrderSku
        storeSkuNumber
        __typename
      }
      media {
        images {
          url
          sizes
          type
          subType
          __typename
        }
        __typename
      }
      pricing(
        storeId: $storeId
      ) {
        value
        promotion {
          promotionTag
          type
          description {
            shortDesc
            longDesc
            __typename
          }
          dollarOff
          percentageOff
          savingsCenter
          savingsCenterPromos
          specialBuySavings
          specialBuyDollarOff
          specialBuyPercentageOff
          __typename
        }
        original
        message
        preferredPriceFlag
        specialBuy
        unitOfMeasure
        __typename
      }
      __typename
    }
    __typename
  }
}"#;

#[derive(Debug, Deserialize)]
struct Response {
    data: SearchModel,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SearchModel {
    search_model: Products,
}

#[derive(Debug, Deserialize)]
struct Products {
    products: Vec<Product>,
}

pub fn get_promo_items(item_ids: Vec<String>) -> Result<Vec<Product>> {
    let variables = serde_json::json!({
            "itemIds":item_ids,
            "storeId": null,
    });

    let body = serde_json::json!({
        "operationName": "promotionProductsItems",
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

    Ok(resp.data.search_model.products)
}
