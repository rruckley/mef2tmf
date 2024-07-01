use tmflib::tmf674::geographic_site_v4::GeographicSite as TMFSite;
use meflib::w122::geographic_site::GeographicSite as MEFSite;

use std::convert::From;

impl From<MEFSite> for TMFSite {
    fn from(value: MEFSite) -> Self {
        let tmf = TMFSite {
            id :  Some(value.id.clone()),
            name : Some(value.name),
            description : Some(value.description),
            ..Default::default()
        };
        //tmf.generate_href();
        tmf
    }
}