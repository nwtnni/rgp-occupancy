use std::env;
use std::error;
use std::fs;
use std::io::Write as _;
use std::time;

use reqwest::blocking;

fn main() -> Result<(), Box<dyn error::Error>> {
    let path =
        env::var("RGP_OCCUPANCY_LOG").expect("Must set `RGP_OCCUPANCY_LOG` environment variable");
    let facility_id = env::var("RGP_OCCUPANCY_FACILITY_ID")
        .expect("Must set `RGP_OCCUPANCY_FACILITY_ID` environment variable");
    let facility_tag = env::var("RGP_OCCUPANCY_FACILITY_TAG")
        .expect("Must set `RGP_OCCUPANCY_FACILITY_TAG` environment variable");

    let client = blocking::Client::builder()
        .build()
        .expect("[INTERNAL ERROR]: invalid reqwest client");

    let mut log = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;

    let mut html = client
        .get(&format!(
            "https://portal.rockgympro.com/portal/public/{}/occupancy",
            facility_id
        ))
        .send()
        .and_then(blocking::Response::text)?;

    html.retain(|c| !c.is_whitespace());

    let count = regex::Regex::new(&format!(
        "'{}':\\{{'capacity':\\d+,'count':(\\d+),",
        facility_tag
    ))
    .expect("[INTERNAL ERROR]: invalid regex")
    .captures(&*html)
    .and_then(|captures| captures.get(1))
    .expect("[INTERNAL ERROR]: count regex returned nothing")
    .as_str();

    let time = time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .expect("[INTERNAL ERROR]: time went backwards");

    writeln!(&mut log, "{},{}", time.as_secs(), count)?;

    Ok(())
}
