use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use utoipa::ToSchema;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
/// The string encoding of a JSON object representing some `appData`.
pub enum OrderCreationAppData {
    /// Hash is inferred from full app data and validated against expectation.
    Both {
        #[serde(rename = "appData")]
        full: String,
        #[serde(rename = "appDataHash")]
        expected: String,
    },
    /// Backward compatible app data hash.
    Hash {
        #[serde(rename = "appData")]
        hash: String,
    },
    /// Hash is inferred from full app data.
    Full {
        #[serde(rename = "appData")]
        full: String,
    },
}

impl Default for OrderCreationAppData {
    fn default() -> Self {
        Self::Hash {
            hash: Default::default(),
        }
    }
}

/// An order as provided to the POST order endpoint.
#[serde_as]
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OrderCreation {
    /// Address of token sold.
    #[schema(value_type = String, example = "0x6810e776880c02933d47db1b9fc05908e5386b96")]
    pub sell_token: String,
    /// Address of token bought.
    #[schema(value_type = String, example = "0x6810e776880c02933d47db1b9fc05908e5386b96")]
    pub buy_token: String,
    /// The string encoding of a JSON object representing some `appData`.
    #[schema(value_type = String, example = "0x000000")]
    #[serde(flatten)]
    pub app_data: OrderCreationAppData,
}

use utoipa::OpenApi;
#[derive(OpenApi)]
#[openapi(components(schemas(OrderCreation)))]
struct ApiDoc;


fn main() {
    println!("{}", ApiDoc::openapi().to_yaml().unwrap());
}
