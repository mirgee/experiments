use std::fmt::Display;

use aries_vcx_agent::aries_vcx::messages::{connection::{
    problem_report::ProblemReport as ConnectionProblemReport, request::Request as ConnectionRequest,
    response::SignedResponse as ConnectionResponse,
}, a2a::A2AMessage};
use serde_json::Value;

pub enum MessagesCommand {
    ConnectionRequest(ConnectionRequest),
    ConnectionResponse(ConnectionResponse),
    ConnectionProblemReport(ConnectionProblemReport),
    Generic(Value),
    GoBack
}

impl From<A2AMessage> for MessagesCommand {
    fn from(message: A2AMessage) -> Self {
        match message {
            A2AMessage::ConnectionRequest(request) => Self::ConnectionRequest(request),
            A2AMessage::ConnectionResponse(response) => Self::ConnectionResponse(response),
            A2AMessage::ConnectionProblemReport(problem_report) => Self::ConnectionProblemReport(problem_report),
            _ => Self::Generic(serde_json::to_value(message).unwrap()),
        }
    }
}

impl Display for MessagesCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConnectionRequest(m) => f.write_fmt(format_args!("Connection Request: {:?}", m)),
            Self::ConnectionResponse(m) => f.write_fmt(format_args!("Connection Response: {:?}", m)),
            Self::ConnectionProblemReport(m) => f.write_fmt(format_args!("Connection Problem Report: {:?}", m)),
            Self::Generic(m) => f.write_fmt(format_args!("Generic: {:?}", m)),
            Self::GoBack => f.write_str("Back"),
        }
    }
}

pub fn get_messages() -> Vec<&'static MessagesCommand> {
    todo!()
}
