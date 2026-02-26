#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Metadata {
    pub state: State,
    pub environment: Environment,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum State {
    Texas,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Environment {
    Stage,
}

impl Metadata {
    pub const fn user_service_url(&self) -> &'static str {
        match (self.state, self.environment) {
            (State::Texas, Environment::Stage) => {
                "https://texas-stage.tylertech.cloud/efm/EFMUserService.svc"
            }
        }
    }

    pub const fn court_record_service_url(&self) -> &'static str {
        match (self.state, self.environment) {
            (State::Texas, Environment::Stage) => {
                "https://texas-stage.tylertech.cloud/efm/v5/CourtRecordService.svc"
            }
        }
    }

    pub const fn court_policy_service_url(&self) -> &'static str {
        match (self.state, self.environment) {
            (State::Texas, Environment::Stage) => {
                "https://texas-stage.tylertech.cloud/efm/v5/CourtPolicyService.svc"
            }
        }
    }
}
