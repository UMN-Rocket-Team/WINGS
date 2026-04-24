use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ProductName {
    AltusMetrum,
    Rfd,
    Featherweight,
    Aim,
    Midwest,
}
