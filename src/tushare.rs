use std::collections::HashMap;

use paste::paste;
use polars::frame::DataFrame;

use crate::builder::*;

/// A struct to hold all tushare calls
pub struct Tushare {
    /// Internal string holds tushare webapi access token.
    /// Used in every call as a hidden parameter.
    pub token: String,
    /// This is actually a constant of "http://api.tushare.pro"
    pub api_endpoint: String,
}

/// Tushare struct methods implementation
impl Tushare {
    /// Only entry to create a tushare object
    /// # token
    /// The token is necessary for every call
    /// Apply it before you do any access
    pub fn new(token: &str) -> Self {
        Tushare {
            token: token.to_string(),
            api_endpoint: "http://api.tushare.pro".to_string(),
        }
    }

    /// Create a QueryBuilder to actually build and process the query
    /// # api_name:
    pub fn query_builder(self: &Self, api_name: &str) -> QueryBuilder {
        QueryBuilder::new(self, api_name)
    }
}

macro_rules! define_api {
    ($api_name:ident, $doc_link:literal $(, $param:ident)*) => {
        paste! {
            #[doc = concat!("Typed query builder for `", stringify!($api_name), "` API. \n\n")]
            #[doc = concat!("See <", $doc_link, "> for detailed documentation.")]
            pub struct [<$api_name:camel QueryBuilder>]<'a> {
                tushare: &'a Tushare,
                $(
                    $param: Option<&'a str>,
                )*
            }

            impl<'a> [<$api_name:camel QueryBuilder>]<'a> {
                pub fn new(tushare: &'a Tushare) -> Self {
                    Self {
                        tushare,
                        $($param: None,)*
                    }
                }

                $(
                    #[doc = concat!("Set `", stringify!($param), "` param for `", stringify!($api_name), "` API query.")]
                    pub fn $param(mut self, $param: &'a str) -> Self {
                        self.$param = Some($param);
                        self
                    }
                )*

                /// Consume the typed query builder to produce a [`QueryBuilder`].
                pub fn into_query_builder(self) -> QueryBuilder<'a> {
                    let mut dict = HashMap::new();
                    $(
                        if let Some($param) = self.$param {
                            dict.insert(stringify!($param).to_string(), $param.to_string());
                        }
                    )*
                    QueryBuilder::new(&self.tushare, stringify!($api_name)).params(dict)
                }

                /// Directly run the query without setting fields.
                pub fn query(self) -> Result<DataFrame, TushareError> {
                    self.into_query_builder().query()
                }
            }

            impl Tushare {
                pub fn $api_name<'a>(&'a self) -> [<$api_name:camel QueryBuilder>]<'a> {
                    [<$api_name:camel QueryBuilder>]::new(self)
                }
            }
        }
    };
}

define_api! {
    stock_basic,
    "https://tushare.pro/document/2?doc_id=25",
    ts_code,
    list_status
}
