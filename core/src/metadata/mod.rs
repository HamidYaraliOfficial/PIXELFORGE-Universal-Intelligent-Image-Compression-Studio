use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MetadataSummary {
    pub orientation: Option<u16>,
    pub gps_present: bool,
    pub fields: Vec<String>,
}

pub fn inspect(path: &Path) -> Result<MetadataSummary> {
    let file = std::fs::File::open(path)?;
    let mut buf = std::io::BufReader::new(file);
    let exif = match exif::Reader::new().read_from_container(&mut buf) {
        Ok(v) => v,
        Err(_) => return Ok(MetadataSummary::default()),
    };
    let mut fields = Vec::new();
    let mut orientation = None;
    let mut gps = false;
    for f in exif.fields() {
        let name = f.tag.to_string();
        if name == "Orientation" {
            orientation = f.value.get_uint(0).map(|v| v as u16);
        }
        if name.starts_with("GPS") { gps = true; }
        fields.push(name);
    }
    Ok(MetadataSummary { orientation, gps_present: gps, fields })
}
