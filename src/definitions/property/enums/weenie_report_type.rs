use num_derive::FromPrimitive;

#[derive(Hash, Eq, PartialEq, Debug, Clone, FromPrimitive)]
pub enum WeenieReportType {
    Reportable,
    Inquirable,
    Ignored,
}
