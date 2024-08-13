//!
//! This library provides conversion functions between MEF and TMF schema where applicable.
//! 
//! # Supported Classes
//! - Geographic Site: TMF674 MEFW122
//! 


use tmflib::{tmf674::geographic_site_v4::GeographicSite as TMFSite, HasName};
use tmflib::common::related_party::RelatedParty;
use meflib::w122::geographic_site::GeographicSite as MEFSite;

use std::convert::From;

#[warn(missing_docs)]

/// Container class to hold either TMF or MEF site objects
#[derive(Clone)]
pub enum SiteVal {
    /// TMF Varient
    TMF(TMFSite),
    /// MEF Varient
    MEF(MEFSite),
}

impl From<MEFSite> for SiteVal {
    fn from(value: MEFSite) -> Self {
        SiteVal::MEF(value)
    }
}

impl From<TMFSite> for SiteVal {
    fn from(value: TMFSite) -> Self {
        SiteVal::TMF(value)
    }
}

pub struct MEFTMF {
    val : SiteVal,
}

impl From<SiteVal> for TMFSite {
    fn from(value: SiteVal) -> Self {
        match value {
            SiteVal::TMF(t) => t.clone(),
            SiteVal::MEF(m) => {
                // This is where we do the conversion from MEF into TMF
                let mut tmf = TMFSite::new(m.name);
                tmf.description = Some(m.description);
                tmf.related_party = Some(vec![]);
                m.related_contact_information.into_iter().for_each(|c| {
                    // Convert RelatedContactInformation into RelatedParty from TMF
                    
                    let related_party = RelatedParty {
                        name : Some(c.name.clone()),
                        base_type: Some("individual".to_string()),
                        ..Default::default()
                    };
                    tmf.related_party.as_mut().unwrap().push(related_party);
                });
                tmf
            }
        }
    }
}

impl From<SiteVal> for MEFSite {
    fn from(value: SiteVal) -> Self {
        match value {
            SiteVal::TMF(t) => {
                // This is where we do the conversion from TMF into MEF
                MEFSite {
                    name : t.get_name(),
                    description: t.description.unwrap_or("No TMF description".to_string()),
                    company_name: match t.related_party {
                        Some(v) => {
                            v.first().unwrap().clone().name.unwrap()
                        }
                        None => String::from("No name"),
                    },
                    ..MEFSite::default()
                }
            },
            SiteVal::MEF(m) => m.clone(),
        }    
    }
}

impl MEFTMF {
    pub fn mef(&self) -> MEFSite {
        MEFSite::from(self.val.clone())
    }
    pub fn tmf(&self) -> TMFSite {
        TMFSite::from(self.val.clone())
    }
}

impl From<MEFSite> for MEFTMF {
    fn from(value: MEFSite) -> Self {
        MEFTMF { 
            val : SiteVal::MEF(value),
        }
    }
}

/// Convert from MEF into TMF using intermedary Enum
/// ```
/// use meflib::w122::geographic_site::GeographicSite as MEFSite;
/// use mef2tmf::mef_to_tmf_site;
/// let mef = MEFSite::default();
/// let tmf = mef_to_tmf_site(mef);
/// ```
pub fn mef_to_tmf_site(mef : MEFSite) -> TMFSite {
    let siteval = SiteVal::from(mef);
    TMFSite::from(siteval)
}

/// Convert from TMF into MEF using intermedary Enum
/// ```
/// use mef2tmf::tmf_to_mef_site;
/// let tmf = tmflib::tmf674::geographic_site_v4::GeographicSite::new("A Site");
/// let mef = tmf_to_mef_site(tmf);
/// ```
pub fn tmf_to_mef_site(tmf : TMFSite) -> MEFSite {
    let siteval = SiteVal::from(tmf);
    MEFSite::from(siteval)
}

#[cfg(test)]
mod test {
    use tmflib::tmf674::geographic_site_v4::GeographicSite as TMFSite;
    use super::*;
    const TMF_SITE : &str = "TMF Site";
    #[test]
    fn test_tmf_to_mef() {
        let tmf = TMFSite::new(TMF_SITE);
        let mef = tmf_to_mef_site(tmf.clone());

        assert_eq!(tmf.get_name(),mef.name);
    }
}