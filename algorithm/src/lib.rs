mod promos;
mod types;

#[cfg(test)]
mod tests {
    use crate::promos::bogo::bogo;
    use crate::types::{Cart, ProductAlgo};

    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_bogo() {
        let mut src_products = vec![
            ProductAlgo {
                name: "product1".to_string(),
                price: 10.0,
            },
            ProductAlgo {
                name: "product2".to_string(),
                price: 20.0,
            },
            ProductAlgo {
                name: "product3".to_string(),
                price: 30.0,
            },
        ];

        let mut tgt_products = vec![
            ProductAlgo {
                name: "product4".to_string(),
                price: 40.0,
            },
            ProductAlgo {
                name: "product5".to_string(),
                price: 50.0,
            },
            ProductAlgo {
                name: "product6".to_string(),
                price: 60.0,
            },
        ];

        let mut required_product = "product5".to_string();

        let expect = Cart::new(vec![src_products[0].clone(), tgt_products[1].clone()], 60.0);

        let got = bogo(src_products.into(), tgt_products.into(), required_product);

        assert_eq!(expect, got);

        src_products = vec![
            ProductAlgo {
                name: "SAWZALL Metal Cutting Bi-Metal Reciprocating Blade Set (16 Piece)"
                    .to_string(),
                price: 27.97,
            },
            ProductAlgo {
                name:
                    "SAWZALL Wood and Metal Cutting Bi-Metal Reciprocating Saw Blade Set (10-Piece)"
                        .to_string(),
                price: 20.68,
            },
            ProductAlgo {
                name:
                    "SAWZALL Demolition Nail-Embedded Wood and Metal Cutting Bi-Metal Reciprocating Saw Blade Set (13 Piece)"
                        .to_string(),
                price: 25.97,
            },
            ProductAlgo {
                name: "M18 FUEL 18V Lith-Ion Brushless Cordless Combo Kit (5-Tool) w/ Two 5.0 Ah Batteries, 1 Charger, & SAWZALL Blade Set".to_string(),
                price: 669.0,
            },
            ProductAlgo {
                name: "M18 18-Volt Lith-Ion Cordless Combo Kit 9-Tool w/ 2-Batteries, Charger, & SAWZALL Blade Set".to_string(),
                price: 619.0,
            },
        ];

        tgt_products = vec![
            ProductAlgo {
                name: "6 in. 18 TPI Medium Metal Cutting SAWZALL Reciprocating Saw Blades (5-Pack)".to_string(),
                price: 13.47,
            },
            ProductAlgo {
                name: "6 in. 5 TPI Thin Kerf Wood Cutting BiMetal SAWZALL Reciprocating Saw Blades (5-Pack)".to_string(),
                price: 15.09,
            },
            ProductAlgo {
                name: "9 in. 10 TPI TORCH Thick Metal Cutting SAWZALL Reciprocating Saw Blades (5-Pack)".to_string(),
                price: 15.97,
            },
        ];

        required_product =
            "SAWZALL Metal Cutting Bi-Metal Reciprocating Blade Set (16 Piece)".to_string();

        let expect = Cart::new(
            vec![src_products[0].clone(), tgt_products[2].clone()],
            27.97 + 15.97,
        );

        let got = bogo(src_products.into(), tgt_products.into(), required_product);

        assert_eq!(expect, got);
    }
}
