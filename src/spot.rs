use serde::{Deserialize, Deserializer, Serialize};
use chrono::prelude::*;

use ham_rs::{Call,Grid,CountryInfo,Country,LogEntry,Mode};
use ham_rs::lotw::LoTWStatus;

use crate::{Receiver};

#[derive(Debug, Serialize, Deserialize)]
pub struct Spot {
    pub time: DateTime<Utc>,
    pub frequency: f32,
    #[serde(rename = "tunedfrequency")]
    pub tuned_frequency: f32,
    pub power: i32,
    pub drift: i32,
    pub snr: i32,
    pub dt: f32,
    pub msg: Option<String>,
    pub mode: Mode,
    pub distance: Option<f32>,
    #[serde(deserialize_with = "callsign_as_string")]
    pub call: Call,
    pub color: i32,
    pub locator: Option<Grid>,
    pub valid: bool
}

impl Spot {
    pub fn set_call(&mut self, call: Call) {
        self.call = call;
    }

    pub fn is_cq(&self) -> bool {
        match &self.msg {
            Some(msg) if msg.contains("CQ") => true,
            _ => false,
        }
    }

    pub fn current_rx(&self, rx: &Receiver) -> bool {
        if self.tuned_frequency == rx.frequency && self.mode == rx.mode {
            true
        } else {
            false
        }
    }

    pub fn new_state(&self, logs: &Vec<LogEntry>) -> bool {
        match (self.call.country(), self.call.state()) {
            (Ok(country), Some(state)) if country == Country::UnitedStates => {
                match logs.iter().position(|i| i.call.state() == Some(state.to_string())) {
                    Some(_) => false,
                    None => true,
                }
            },
            _ => false,
        }
    }

    pub fn new_country(&self, logs: &Vec<LogEntry>) -> bool {
        match self.call.country() {
            Ok(country) => {
                match logs.iter().position(|i| i.call.country() == Ok(country.clone())) {
                    Some(_) => false,
                    None => true,
                }
            },
            _ => false,
        }
    }

    pub fn uses_lotw(&self) -> bool {
        match self.call.lotw() {
            LoTWStatus::Registered | LoTWStatus::LastUpload(_) => true,
            _ => false
        }
    }
}

pub fn callsign_as_string<'de, D>(deserializer: D) -> Result<Call, D::Error>
    where D: Deserializer<'de>
{
    let v : String = Deserialize::deserialize(deserializer)?;
    Ok(Call::new(v))
}