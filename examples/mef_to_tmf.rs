//! Convert from MEF Site to TMF Site

use mef2tmf::mef_to_tmf_site;
use meflib::w122::geographic_site::GeographicSite as MEFSite;
use meflib::w122::geographic_site::{RelatedContactInformation,FieldedAddress};

fn main() {
    let contact_info = RelatedContactInformation {
        name : "John Q. Citizen".to_string(),
        ..Default::default()
    };

    let address = FieldedAddress::default();
    let mut mef = MEFSite::default();
    mef.name = "MEF Name".to_string();
    mef.description = "MEF Description".to_string();
    mef.postal_address = vec![address];
    mef.company_name = "Building Owner Name".to_string();
    mef.company_name = "Tenant Name".to_string();
    mef.related_contact_information = vec![contact_info];

    let tmf = mef_to_tmf_site(mef);

    dbg!(tmf);
}