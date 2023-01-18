#![doc(html_root_url = "https://docs.rs/wot_datfile_parser/0.3.0")]
//! A parser for `.dat` battle result files generated by the game World of Tanks

//! ## Simple Example
//! ```
//! use wot_datfile_parser::DatFileParser;
//!
//! let file = std::fs::read("input_files/WOT_1_19_1_0/19011713064132879.dat").unwrap();
//!
//! // You must construct the parser first as it needs to
//! // do some initialization to parse the datfiles.
//! // You can then use this same parser to parse any number of datfiles
//! let parser = DatFileParser::new();
//!
//! // The parser generates a Battle struct
//! let battle = parser.parse(&file).unwrap();
//!
//! assert_eq!(
//!    &battle.common["teamHealth"],
//!    &serde_json::json!({ "1": 13595, "2": 12985 })
//! );
//! assert_eq!(&battle.common["duration"], &serde_json::json!(407));
//!
//! // Battle implements serde::Serialize and serde::Deserialize.
//! // So, you can use other data formats as well.
//! // Here we will convert it to json and print it:
//! let battle_as_json = serde_json::to_string_pretty(&battle).unwrap();
//!
//! println!("{battle_as_json}");
//! ```
//!
//! ## Advanced Example
//! If you need to change how some `serde_pickle::Value` are converted to `serde_json::Value`, you can
//! intercept it and provide your own implementation:
//! ```
//! use wot_datfile_parser::{DatFileParser, Intercept};
//!
//! let file = std::fs::read("input_files/WOT_1_19_1_0/19011713064132879.dat").unwrap();
//!
//! let parser = DatFileParser::new();
//!
//! // We use the following closure to change how a serde_pickle::Value is
//! // converted to serde_json::Value. We can also use it to log any errors
//! // in the datfile_parser(by matching the Failed variant)
//! let intercept_fn = |intercept, _original_value| {
//!     use Intercept::*;
//!     match intercept {
//!         Success(field, _) | NotPresent(field, _) |
//!         ManuallyParsed(field, _) |  Failed(field, _, _) => {
//!             if field.name == "teamHealth" {
//!                 // Here we can inspect the original_value and provide our own impl
//!                 // for converting the serde_pickle::Value to serde_json::Value
//!                 // But for this example, we will just return the following:
//!                 serde_json::Value::String("My own parser for teamHealth".into())
//!             } else {
//!                 intercept.original_result()
//!             }
//!         },
//!     }
//! };
//!
//! // The parser generates a Battle struct
//! let battle = parser.parse_intercept(&file, intercept_fn).unwrap();
//!
//! assert_eq!(
//!    &battle.common["teamHealth"],
//!    &serde_json::json!("My own parser for teamHealth")
//! );
//! assert_eq!(&battle.common["duration"], &serde_json::json!(407));
//! ```

mod battle_results;
mod error;
mod fields;
mod manual_parser;
mod parser;

type Result<T> = core::result::Result<T, Error>;

pub(crate) type AnyErr = Box<dyn std::error::Error + Send + Sync + 'static>;

use std::collections::HashMap;

pub use battle_results::Field;
use error::Error;
pub use parser::DatFileParser;
pub use parser::Intercept;
use serde::Deserialize;
use serde::Serialize;
pub use serde_json;
pub use serde_pickle;
pub(crate) use serde_pickle::Value as PickleValue;


/// Intercept Function that allows you to manipulate how a particular value is parsed.
///
/// You only need to use this if you are not satisfied with how a pickle value from the datfile is converted
/// into `serde_json::Value`
pub type InterceptFn = fn(Intercept, serde_pickle::Value) -> serde_json::Value;


/// A `.dat` file is parsed into this data structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Battle {
    #[serde(rename(serialize = "arenaUniqueID"))]
    pub arena_unique_id: String,
    pub common:          serde_json::Value,
    pub player_info:     HashMap<String, serde_json::Value>,
    pub account_all:     HashMap<String, serde_json::Value>,
    pub vehicle_all:     HashMap<String, serde_json::Value>,
    pub vehicle_self:    HashMap<String, serde_json::Value>,
    pub account_self:    HashMap<String, serde_json::Value>,
}

/// `.dat` files pickles usually contain null values instead of the default
/// value. Therefore, we replace it with the default value.
/// For ex. `has_battle_pass = PickleValue::None` is replaced with
/// `has_battle_pass = PickleValue::Bool(false)`.
/// TODO: Explore making this like `#[serde(default)]`
fn to_default_if_none(identifier: &Field, value: PickleValue) -> PickleValue {
    if value == PickleValue::None {
        // Identifier knows what is the default value
        identifier.default.to_pickle_value()
    } else {
        value
    }
}
