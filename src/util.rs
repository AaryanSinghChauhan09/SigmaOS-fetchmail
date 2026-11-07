//! fetchmail model — mail retrieval and forwarding.

#[derive(Debug, Clone)]
pub struct Server {
    pub proto: String,
    pub host: String,
    pub user: String,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub servers: Vec<Server>,
}

impl Default for Model {
    fn default() -> Self { Self { servers: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn add_server(&mut self, proto: &str, host: &str, user: &str) {
        self.servers.push(Server { proto: proto.into(), host: host.into(), user: user.into() });
    }

    pub fn list(&self) -> &[Server] { &self.servers }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_server() {
        let mut f = Model::new();
        f.add_server("pop3", "mail.example.com", "alice");
        assert_eq!(f.list().len(), 1);
    }
}
