use tokio_util::bytes::Bytes;

use crate::core::state::{
    open_serial_port::{CoreOutgoingPacket, CorePacketOrigin},
    CoreSerialState,
};

pub async fn send_to_all_serial_ports_intern(bytes: Bytes, state: &CoreSerialState) {
    tracing::info!("Sending to all serial ports");

    let packet = CoreOutgoingPacket {
        bytes,
        packet_origin: CorePacketOrigin::Broadcast,
    };

    state.send_to_all_open_serial_ports(packet).await
}
