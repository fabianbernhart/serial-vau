/// Defines if an open serial port is currently reading or stopped.
#[derive(Debug, Clone, Copy)]
pub enum ReadState {
    Read,
    Stop,
}

#[derive(Debug)]
pub struct OpenStatus {
    pub read_state: ReadState,
}

#[derive(Debug)]
pub enum Status {
    Closed,
    Open(OpenStatus),
}

impl ReadState {
    pub fn is_stop(&self) -> bool {
        matches!(self, Self::Stop)
    }

    pub fn toggle(self) -> Self {
        match self {
            Self::Read => Self::Stop,
            Self::Stop => Self::Read,
        }
    }
}

#[derive(Debug)]
pub struct ManagedSerialPort {
    pub name: String,
    pub status: Status,
    #[cfg(feature = "subscriptions")]
    pub subscriptions: Vec<String>,
    #[cfg(feature = "subscriptions")]
    pub subscribed_to: Vec<String>,
}

impl ManagedSerialPort {
    pub fn is_open(&self) -> bool {
        matches!(self.status, Status::Open(_))
    }

    pub fn is_closed(&self) -> bool {
        matches!(self.status, Status::Closed)
    }
}