use std::fs::File;
use std::io::{self, Write};
use serde_json;
use crate::models::FastingEvent;
use crate::errors::FastingAppError;

pub fn export_to_csv(events: &[FastingEvent], file_path: &str) -> Result<(), FastingAppError> {
    let mut file = File::create(file_path).map_err(|_| FastingAppError::FileError)?;
    writeln!(file, "start_time,stop_time,duration_minutes").map_err(|_| FastingAppError::FileError)?;
    for event in events {
        let duration = event
            .stop_time
            .unwrap_or(Utc::now().naive_utc())
            .signed_duration_since(event.start_time)
            .num_minutes();
        writeln!(file, "{},{},{}", event.start_time, event.stop_time.unwrap_or(Utc::now().naive_utc()), duration)
            .map_err(|_| FastingAppError::FileError)?;
    }
    Ok(())
}

pub fn export_to_json(events: &[FastingEvent]) -> Result<String, FastingAppError> {
    serde_json::to_string(events).map_err(|_| FastingAppError::SerializationError)
}
