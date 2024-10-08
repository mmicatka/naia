use naia_socket_shared::SocketConfig;

use super::{
    addr_cell::AddrCell, data_channel::DataChannel, data_port::DataPort,
    packet_receiver::PacketReceiverImpl, packet_sender::PacketSenderImpl,
};
use crate::{
    backends::socket::SocketTrait, conditioned_packet_receiver::ConditionedPacketReceiver,
    packet_receiver::PacketReceiver, packet_sender::PacketSender, IdentityReceiver,
};

/// A client-side socket which communicates with an underlying unordered &
/// unreliable protocol
pub struct Socket;

impl Socket {
    /// Connects to the given server address
    pub fn connect(
        server_session_url: &str,
        config: &SocketConfig,
    ) -> (
        Box<dyn IdentityReceiver>,
        Box<dyn PacketSender>,
        Box<dyn PacketReceiver>,
    ) {
        return Self::connect_inner(server_session_url, config, None, None);
    }

    /// Connects to the given server address with authentication
    pub fn connect_with_auth(
        server_session_url: &str,
        config: &SocketConfig,
        auth_bytes: Vec<u8>,
    ) -> (
        Box<dyn IdentityReceiver>,
        Box<dyn PacketSender>,
        Box<dyn PacketReceiver>,
    ) {
        return Self::connect_inner(server_session_url, config, Some(auth_bytes), None);
    }

    /// Connects to the given server address with authentication
    pub fn connect_with_auth_headers(
        server_session_url: &str,
        config: &SocketConfig,
        auth_headers: Vec<(String, String)>,
    ) -> (
        Box<dyn IdentityReceiver>,
        Box<dyn PacketSender>,
        Box<dyn PacketReceiver>,
    ) {
        return Self::connect_inner(server_session_url, config, None, Some(auth_headers));
    }

    /// Connects to the given server address with authentication
    pub fn connect_with_auth_and_headers(
        server_session_url: &str,
        config: &SocketConfig,
        auth_bytes: Vec<u8>,
        auth_headers: Vec<(String, String)>,
    ) -> (
        Box<dyn IdentityReceiver>,
        Box<dyn PacketSender>,
        Box<dyn PacketReceiver>,
    ) {
        return Self::connect_inner(
            server_session_url,
            config,
            Some(auth_bytes),
            Some(auth_headers),
        );
    }

    /// Connects to the given server address
    fn connect_inner(
        server_session_url: &str,
        config: &SocketConfig,
        auth_bytes_opt: Option<Vec<u8>>,
        auth_headers_opt: Option<Vec<(String, String)>>,
    ) -> (
        Box<dyn IdentityReceiver>,
        Box<dyn PacketSender>,
        Box<dyn PacketReceiver>,
    ) {
        let data_channel =
            DataChannel::new(config, server_session_url, auth_bytes_opt, auth_headers_opt);

        let data_port = data_channel.data_port();
        let addr_cell = data_channel.addr_cell();

        let (packet_sender, packet_receiver) = Socket::setup_io(config, &addr_cell, &data_port);

        // Setup Identity Receiver
        let id_receiver: Box<dyn IdentityReceiver> = Box::new(data_channel.id_receiver());

        data_channel.start();

        return (id_receiver, packet_sender, packet_receiver);
    }

    // Creates a Socket from an underlying DataPort.
    // This is for use in apps running within a Web Worker.
    pub fn connect_with_data_port(
        config: &SocketConfig,
        data_port: &DataPort,
    ) -> (Box<dyn PacketSender>, Box<dyn PacketReceiver>) {
        let addr_cell = AddrCell::new();
        return Socket::setup_io(config, &addr_cell, data_port);
    }

    fn setup_io(
        config: &SocketConfig,
        addr_cell: &AddrCell,
        data_port: &DataPort,
    ) -> (Box<dyn PacketSender>, Box<dyn PacketReceiver>) {
        // Setup Packet Sender
        let packet_sender_impl = PacketSenderImpl::new(&data_port, addr_cell);

        let packet_sender: Box<dyn PacketSender> = Box::new(packet_sender_impl);

        // Setup Packet Receiver
        let packet_receiver_impl = PacketReceiverImpl::new(&data_port, addr_cell);

        let packet_receiver: Box<dyn PacketReceiver> = {
            let inner_receiver = Box::new(packet_receiver_impl);
            if let Some(config) = &config.link_condition {
                Box::new(ConditionedPacketReceiver::new(inner_receiver, config))
            } else {
                inner_receiver
            }
        };

        return (packet_sender, packet_receiver);
    }
}

impl SocketTrait for Socket {
    /// Connects to the given server address
    fn connect(
        server_session_url: &str,
        config: &SocketConfig,
    ) -> (
        Box<dyn IdentityReceiver>,
        Box<dyn PacketSender>,
        Box<dyn PacketReceiver>,
    ) {
        return Self::connect(server_session_url, config);
    }
    /// Connects to the given server address with authentication
    fn connect_with_auth(
        server_session_url: &str,
        config: &SocketConfig,
        auth_bytes: Vec<u8>,
    ) -> (
        Box<dyn IdentityReceiver>,
        Box<dyn PacketSender>,
        Box<dyn PacketReceiver>,
    ) {
        return Self::connect_with_auth(server_session_url, config, auth_bytes);
    }
}
