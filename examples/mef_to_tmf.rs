//! Convert from MEF Site to TMF Site

use mef2tmf::mef_to_tmf_site;
use meflib::w122::geographic_site::GeographicSite as MEFSite;

fn main() {
    let mut mef = MEFSite::default();
    mef.name = "MEF Name".to_string();

    let tmf = mef_to_tmf_site(mef);

    dbg!(tmf);
}