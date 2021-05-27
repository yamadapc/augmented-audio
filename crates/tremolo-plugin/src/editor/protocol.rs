use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MessageWrapper<Message> {
    pub id: Option<String>,
    pub channel: String,
    pub message: Message,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ServerMessageInner {
    PublishParameters(PublishParametersMessage),
}
pub type ServerMessage = MessageWrapper<ServerMessageInner>;

#[derive(Serialize, Deserialize)]
pub struct PublishParametersMessage {
    pub parameters: Vec<ParameterDeclarationMessage>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterDeclarationMessage {
    pub id: String,
    pub name: String,
    pub label: String,
    pub text: String,
    pub value: f32,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessageInner {
    AppStarted(AppStartedMessage),
    SetParameter(SetParameterMessage),
    Log(LogMessage),
}
pub type ClientMessage = MessageWrapper<ClientMessageInner>;

#[derive(Serialize, Deserialize)]
pub struct AppStartedMessage;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetParameterMessage {
    parameter_id: String,
    value: f32,
}

#[derive(Serialize, Deserialize)]
pub struct LogMessage {
    level: String,
    message: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_serde_enum() {
        let client_message = ClientMessageInner::AppStarted(AppStartedMessage {});
        let result = serde_json::to_string(&client_message).unwrap();
        assert_eq!(result, "{\"type\":\"AppStarted\"}")
    }
}
