use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IncomingPacket {
    pub line: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionPacketOrigin {
    pub name: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type", content = "content")]
pub enum PacketOrigin {
    Direct,
    Broadcast,
    #[cfg(feature = "subscriptions")]
    Subscription(SubscriptionPacketOrigin),
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutgoingPacket {
    pub packet_origin: PacketOrigin,
    /// Lossy UTF-8 string representation of the sent bytes.
    pub value: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type", content = "content")]
pub enum PacketDirection {
    /// From the open serial port to the application.
    Incoming(IncomingPacket),
    /// From the application to the open serial port.
    Outgoing(OutgoingPacket),
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Packet {
    pub packet_direction: PacketDirection,
    pub port_name: String,
    pub timestamp_millis: u64,
}

mod core_impl {
    use super::*;
    #[cfg(feature = "subscriptions")]
    use crate::app::serial_state::model::CoreSubscriptionPacketOrigin;
    use crate::app::serial_state::model::{
        CoreIncomingPacket, CoreOutgoingPacket, CorePacket, CorePacketDirection, CorePacketOrigin,
    };

    impl From<CoreIncomingPacket> for IncomingPacket {
        fn from(value: CoreIncomingPacket) -> Self {
            Self {
                line: String::from_utf8_lossy(&value.line).to_string(),
            }
        }
    }

    #[cfg(feature = "subscriptions")]
    impl From<CoreSubscriptionPacketOrigin> for SubscriptionPacketOrigin {
        fn from(value: CoreSubscriptionPacketOrigin) -> Self {
            Self { name: value.name }
        }
    }

    impl From<CorePacketOrigin> for PacketOrigin {
        fn from(value: CorePacketOrigin) -> Self {
            match value {
                CorePacketOrigin::Direct => Self::Direct,
                CorePacketOrigin::Broadcast => Self::Broadcast,
                #[cfg(feature = "subscriptions")]
                CorePacketOrigin::Subscription(origin) => Self::Subscription(origin.into()),
            }
        }
    }

    impl From<CoreOutgoingPacket> for OutgoingPacket {
        fn from(value: CoreOutgoingPacket) -> Self {
            Self {
                packet_origin: value.packet_origin.into(),
                value: String::from_utf8_lossy(&value.bytes).to_string(),
            }
        }
    }

    impl From<CorePacketDirection> for PacketDirection {
        fn from(value: CorePacketDirection) -> Self {
            match value {
                CorePacketDirection::Incoming(packet) => Self::Incoming(packet.into()),
                CorePacketDirection::Outgoing(packet) => Self::Outgoing(packet.into()),
            }
        }
    }

    impl From<CorePacket> for Packet {
        fn from(value: CorePacket) -> Self {
            Self {
                packet_direction: value.packet_direction.into(),
                port_name: value.port_name,
                timestamp_millis: value.timestamp_millis,
            }
        }
    }
}
