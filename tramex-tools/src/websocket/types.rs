use crate::{functions::extract_hexe, websocket::layer::Layer};

// deserialize the message
#[derive(serde::Deserialize, Debug)]
pub struct WebSocketLog {
    pub message: String,         // Same as request
    pub message_id: Option<u64>, //Any type, force as string // Same as in request.
    pub time: f64, // Number representing time in seconds since start of the process. // Usefull to send command with absolute time.
    pub utc: f64,  //Number representing UTC seconds.
    pub logs: Vec<OneLog>,
}

#[derive(Debug, PartialEq)]
pub enum LogLevel {
    ERROR = 1,
    WARN = 2,
    INFO = 3,
    DEBUG = 4,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub enum SourceLog {
    ENB,
    MME,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub enum Direction {
    UL,
    DL,
    FROM,
    TO,
}

#[derive(serde::Deserialize, Debug)]
pub struct OneLog {
    pub data: Vec<String>,      // Each item is a string representing a line of log.
    pub timestamp: u64,         // Milliseconds since January 1st 1970.
    pub layer: Layer,           // log layer
    pub level: LogLevel,        // Log level: error, warn, info or debug.
    pub dir: Option<Direction>, //  Log direction: UL, DL, FROM or TO.
    pub cell: Option<u64>,      // cell id
    pub channel: Option<String>, // channel
    pub src: SourceLog,
    pub idx: u64,
}

impl OneLog {
    pub fn extract_hexe(&self) -> Vec<u8> {
        return extract_hexe(&self.data);
    }

    pub fn extract_canal_msg(&self) -> Option<String> {
        if let Some(data_line) = self.data.first() {
            log::info!("{:?}", data_line);
            return Some(data_line.to_owned());
        }
        None
    }
}

impl<'de> serde::Deserialize<'de> for LogLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let a = u8::deserialize(deserializer)?;
        match a {
            1 => Ok(LogLevel::ERROR),
            2 => Ok(LogLevel::WARN),
            3 => Ok(LogLevel::INFO),
            4 => Ok(LogLevel::DEBUG),
            _ => Ok(LogLevel::INFO), // default
        }
    }
}