use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Debug, Clone, FromPrimitive)]
pub enum WeenieReportType {
    Reportable,
    Inquirable,
    Ignored,
}
