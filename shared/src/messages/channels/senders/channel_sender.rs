use naia_serde::BitWriter;
use naia_socket_shared::Instant;

use crate::messages::channels::senders::request_sender::LocalRequestId;
use crate::messages::request::GlobalRequestId;
use crate::{
    messages::{message_container::MessageContainer, message_kinds::MessageKinds},
    types::MessageIndex,
    LocalEntityAndGlobalEntityConverterMut, LocalResponseId,
};

pub trait ChannelSender<P>: Send + Sync {
    /// Queues a Message to be transmitted to the remote host into an internal buffer
    fn send_message(&mut self, message: P);
    /// For reliable channels, will collect any Messages that need to be resent
    fn collect_messages(&mut self, now: &Instant, rtt_millis: &f32);
    /// Returns true if there are queued Messages ready to be written
    fn has_messages(&self) -> bool;
    /// Called when it receives acknowledgement that a Message has been received
    fn notify_message_delivered(&mut self, message_index: &MessageIndex);
}

pub trait MessageChannelSender: ChannelSender<MessageContainer> {
    /// Gets Messages from the internal buffer and writes it to the BitWriter
    fn write_messages(
        &mut self,
        message_kinds: &MessageKinds,
        converter: &mut dyn LocalEntityAndGlobalEntityConverterMut,
        writer: &mut BitWriter,
        has_written: &mut bool,
    ) -> Option<Vec<MessageIndex>>;

    /// Queues a Request to be transmitted to the remote host into an internal buffer
    fn send_outgoing_request(
        &mut self,
        message_kinds: &MessageKinds,
        converter: &mut dyn LocalEntityAndGlobalEntityConverterMut,
        global_request_id: GlobalRequestId,
        request: MessageContainer,
    );

    /// Queues a Response to be transmitted to the remote host into an internal buffer
    fn send_outgoing_response(
        &mut self,
        message_kinds: &MessageKinds,
        converter: &mut dyn LocalEntityAndGlobalEntityConverterMut,
        local_response_id: LocalResponseId,
        response: MessageContainer,
    );

    /// Request is finished, so clean up the local request id and return the global request id
    fn process_incoming_response(
        &mut self,
        local_request_id: &LocalRequestId,
    ) -> Option<GlobalRequestId>;
}
