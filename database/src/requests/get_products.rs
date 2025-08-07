use anyhow::{Context, Result};
use serde::Deserialize;

use crate::types::Product;

const QUERY: &str = r#"
query searchModel($startIndex: Int, $pageSize: Int, $orderBy: ProductSort, $filter: ProductFilter, $navParam: String) {
  searchModel(navParam: $navParam) {
    products(
      startIndex: $startIndex
      pageSize: $pageSize
      orderBy: $orderBy
      filter: $filter
    ) {
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
        }
      }
      pricing(storeId: null) {
        value
        original
        preferredPriceFlag
        promotion {
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
          }
          reward {
            tiers {
              minThresholdVal
              thresholdType
              rewardVal
              rewardType
              rewardLevel
              maxAllowedRewardAmount
            }
          }
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
    searchReport {
      totalProducts
      didYouMean
      correctedKeyword
      keyword
      pageSize
      searchUrl
      sortBy
      sortOrder
      startIndex
    }
  }
}"#;

// dept=Tools, brand=Milwaukee:                                     5yc1vZc 1xy Zzv
// dept=Tools, brand=Milwaukee, savingsCenter=BMSM:                 5yc1vZc 1xy Zzv Z1z10aav
// dept=Tools, brand=Milwaukee, savingsCenter=BOGO:                 5yc1vZc 1xy Zzv Z1z1xyom
// dept=Tools, brand=Milwaukee, savingsCenter=SpecialBuy:           5yc1vZc 1xy Zzv Z1z11ao3
// dept=Tools, savingsCenter=SpecialBuy:                            5yc1vZc 1xy     Z1z11ao3
// dept=Tools, savingsCenter=BMSM:                                  5yc1vZc 1xy     Z1z10aav
// dept=Tools, savingsCenter=BOGO:                                  5yc1vZc 1xy     Z1z1xyom
// dept=Tools, brand=Milwaukee, savingsCenter=BMSM,BOGO,SpecialBuy: 5yc1vZc 1xy Zzv Z1z10aav Z1z11ao3 Z1z1xyom
// dept=PowerTools, brand=Milwaukee, savingsCenter=BMSM, BOGO:      5yc1vZc 298 Zzv Z1z10aav Z1z1xyom
// dept=Tools, brand=None, savingsCenter=BMSM, BOGO:                5yc1vZc 1xy Z1z 10aavZ1z1xyom

#[derive(Debug)]
pub enum Brand {
    Milwaukee,
}

pub enum Department {
    Tools,
    PowerTools,
}

pub struct SearchParams {
    pub department: Option<Department>,
    pub brand: Option<Brand>,
    pub buy_more_save_more: bool,
    pub buy_one_get_one: bool,
    pub special_buy: bool,
}

impl SearchParams {
    pub fn to_nav_param(&self) -> String {
        let mut nav_param = String::from("5yc1vZc");

        match &self.department {
            Some(d) => match d {
                Department::Tools => nav_param += "1xy",
                Department::PowerTools => nav_param += "298",
            },
            None => nav_param += "1xy",
        }

        match &self.brand {
            Some(b) => match b {
                Brand::Milwaukee => nav_param += "Zzv",
            },
            None => nav_param += "Z1z",
        }

        if self.buy_more_save_more {
            nav_param += "Z1z10aav"
        }

        if self.buy_one_get_one {
            nav_param += "Z1z1xyom"
        }

        if self.special_buy {
            nav_param += "Z1z11ao3"
        }

        nav_param
    }
}

#[derive(Debug, Deserialize)]
struct Response {
    data: Data,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Data {
    search_model: Option<SearchModel>,
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SearchModel {
    products: Vec<Product>,
    search_report: SearchReport,
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SearchReport {
    pub total_products: u32,
    pub page_size: u32,
    pub start_index: u32,
}

pub fn get_products(search_params: SearchParams) -> Result<Vec<Product>> {
    let mut products: Vec<Product> = Vec::new();
    let page_size = 24;
    let mut index = 0;

    loop {
        let variables = serde_json::json!({
          "navParam":search_params.to_nav_param(),
          "orderBy": {
            "field": "PRICE",
            "order": "DESC"
          },
          "pageSize": page_size,
          "startIndex": index
        });

        let body = serde_json::json!({
            "operationName": "searchModel",
            "variables": variables,
            "query": QUERY,
        });

        let resp_str = ureq::post("https://apionline.homedepot.com/federation-gateway/graphql")
            .header("x-experience-name", "fusion-gm-pip-desktop")
            .header("content-type", "application/json")
            .send_json(body)?
            .body_mut()
            .read_to_string()?;

        let resp: Response =
            serde_json::from_str(&resp_str).context("Failed to parse get_products response")?;

        match resp.data.search_model {
            Some(mut search_model) if index < search_model.search_report.total_products => {
                products.append(&mut search_model.products);
                index += page_size
            }
            _ => break,
        }
    }

    Ok(products)
}
