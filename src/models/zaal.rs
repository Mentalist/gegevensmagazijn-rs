use serde::{Deserialize, Serialize};

use super::{Reservering, TkId, TkObject};

impl TkObject for Zaal {
    fn entity_set() -> &'static str {
        "Zaal"
    }
}

/// Zaal
///
/// - [Documentatie](https://opendata.tweedekamer.nl/documentatie/zaal)
/// - [XSD](https://github.com/TweedeKamerDerStaten-Generaal/OpenDataPortaal/blob/master/xsd/tkData-v1-0-zaal.xsd)
#[derive(
    Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct Zaal {
    #[serde(rename = "Id")]
    pub id: Option<TkId>,
    #[serde(rename = "Naam")]
    pub naam: Option<String>,
    #[serde(rename = "SysCode")]
    pub sys_code: Option<i64>,
    #[serde(rename = "GewijzigdOp")]
    pub gewijzigd_op: Option<String>,
    #[serde(rename = "ApiGewijzigdOp")]
    pub api_gewijzigd_op: Option<String>,
    #[serde(rename = "Verwijderd")]
    pub verwijderd: Option<bool>,
    #[serde(rename = "Reservering")]
    pub reservering: Option<Vec<Reservering>>,
}
