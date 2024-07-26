use tmflib::{tmf674::geographic_site_v4::GeographicSite as TMFSite, HasName};
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
                TMFSite::new(m.name)
            }
        }
    }
}

impl From<SiteVal> for MEFSite {
    fn from(value: SiteVal) -> Self {
        match value {
            SiteVal::TMF(t) => {
                let mut mef = MEFSite::default();
                mef.name = t.get_name();
                mef
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