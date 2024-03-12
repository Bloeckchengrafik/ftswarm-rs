use std::fmt::{Display, Formatter};
use std::time::Duration;

#[derive(Debug)]
pub struct WhoamiResponse {
    pub hostname: String,
    pub id: String,
    pub serial: Option<String>,
}

impl TryFrom<String> for  WhoamiResponse {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut parts = value.split('/');
        let id = parts.next().ok_or("No ID found")?;
        let hostname = parts.next().ok_or("No hostname found")?;

        let serial = parts.next().map(|s| s.to_string());

        Ok(WhoamiResponse {
            hostname: hostname.to_string(),
            id: id.to_string(),
            serial,
        })
    }
}

impl Display for WhoamiResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ID: {}, Hostname: {}, Serial: {:?}", self.id, self.hostname, self.serial)
    }
}

pub fn parse_uptime(value: String) -> Result<Duration, String> {
    let mut parts = value.split(':').last().ok_or("No uptime found")?;
    parts = parts.split(".").nth(0).ok_or("No uptime found")?.trim();
    let uptime: u64 = parts.parse().map_err(|_| "Failed to parse uptime")?;

    Ok(Duration::from_secs(uptime))
}