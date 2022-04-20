use actix::{Actor, Handler, Message, MessageResponse, MessageResult};
use augmented_analytics::{AnalyticsClient, AnalyticsWorker};
use cacao::defaults::UserDefaults;

struct AnalyticsService {
    analytics_enabled: Option<bool>,
    // analytics: AnalyticsWorker,
}

impl Default for AnalyticsService {
    fn default() -> Self {
        Self {
            analytics_enabled: None,
            // analytics: AnalyticsWorker::new(
            //     Default::default(),
            //     Box::new(GoogleAnalyticsBackend::new(GoogleAnalyticsConfig::new(
            //     ))),
            //     ClientMetadata::new("1"), // <- this should be an anonymous client-id
            //     receiver,
            // ),
        }
    }
}

impl Actor for AnalyticsService {
    type Context = actix::Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        self.analytics_enabled = UserDefaults::standard()
            .get("analytics_enabled")
            .map(|value| value.as_bool())
            .flatten();
    }
}

#[derive(Message)]
#[rtype(result = "Option<bool>")]
pub struct GetAnalyticsEnabled;

impl Handler<GetAnalyticsEnabled> for AnalyticsService {
    type Result = Option<bool>;

    fn handle(&mut self, _msg: GetAnalyticsEnabled, _ctx: &mut Self::Context) -> Self::Result {
        self.analytics_enabled.into()
    }
}

#[derive(Message)]
#[rtype("()")]
pub struct SetAnalyticsEnabled(bool);

impl Handler<SetAnalyticsEnabled> for AnalyticsService {
    type Result = ();

    fn handle(&mut self, msg: SetAnalyticsEnabled, _ctx: &mut Self::Context) -> Self::Result {
        self.analytics_enabled = Some(msg.0);
        let mut user_defaults = UserDefaults::standard();
        user_defaults.insert("analytics_enabled", cacao::defaults::Value::Bool(msg.0));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[actix::test]
    async fn test_analytics_service_set_defaults() {
        let analytics_service = AnalyticsService::default().start();
        let result = analytics_service.send(GetAnalyticsEnabled).await.unwrap();
        assert_eq!(result, None);
    }

    #[actix::test]
    async fn test_analytics_service_starts() {
        let analytics_service = AnalyticsService::default().start();
        let result = analytics_service.send(GetAnalyticsEnabled).await.unwrap();
        assert_eq!(result, None);
    }
}
