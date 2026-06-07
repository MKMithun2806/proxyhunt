use serde::Serialize;
use std::net::IpAddr;
use maxminddb::Reader;
use std::path::Path;

#[derive(Debug, Clone, Serialize)]
pub struct Enrichment {
    pub country: Option<String>,
    pub city: Option<String>,
    pub asn: Option<u32>,
    pub isp: Option<String>,
}

pub struct Enricher {
    reader: Option<Reader<Vec<u8>>>,
}

impl Enricher {
    pub fn new<P: AsRef<Path>>(db_path: Option<P>) -> Self {
        let reader = db_path.and_then(|p| {
            maxminddb::Reader::open_readfile(p).ok()
        });
        Self { reader }
    }

    pub fn enrich(&self, ip_str: &str) -> Option<Enrichment> {
        let reader = self.reader.as_ref()?;
        let ip: IpAddr = ip_str.parse().ok()?;
        
        // This is a simplified version. Real MaxMind db has separate files for City and ASN.
        // For brevity, we just try to get whatever we can.
        let city: Option<maxminddb::geoip2::City> = reader.lookup(ip).ok();
        
        Some(Enrichment {
            country: city.as_ref().and_then(|c| c.country.as_ref()).and_then(|co| co.names.as_ref()).and_then(|n| n.get("en")).map(|s| s.to_string()),
            city: city.as_ref().and_then(|c| c.city.as_ref()).and_then(|ci| ci.names.as_ref()).and_then(|n| n.get("en")).map(|s| s.to_string()),
            asn: None, // Requires ASN database
            isp: None, // Requires ASN/ISP database
        })
    }
}
