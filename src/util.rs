//! 0ad model — game.

#[derive(Debug, Clone)]
pub struct Session {
    pub name: String,
    pub players: Vec<String>,
    pub running: bool,
    pub score: u32,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub sessions: Vec<Session>,
}

impl Default for Model {
    fn default() -> Self { Self { sessions: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn create(&mut self, name: &str) -> usize {
        let id = self.sessions.len();
        self.sessions.push(Session { name: name.into(), players: vec![], running: false, score: 0 });
        id
    }

    pub fn join(&mut self, session_id: usize, player: &str) -> bool {
        if let Some(s) = self.sessions.get_mut(session_id) {
            if !s.players.contains(&player.to_string()) {
                s.players.push(player.into());
            }
            true
        } else { false }
    }

    pub fn start(&mut self, session_id: usize) -> bool {
        if let Some(s) = self.sessions.get_mut(session_id) {
            s.running = true;
            true
        } else { false }
    }

    pub fn end(&mut self, session_id: usize, score: u32) -> bool {
        if let Some(s) = self.sessions.get_mut(session_id) {
            s.running = false;
            s.score = score;
            true
        } else { false }
    }

    pub fn list(&self) -> &[Session] { &self.sessions }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn session_lifecycle() {
        let mut m = Model::new();
        let id = m.create("match1");
        assert!(m.join(id, "player1"));
        assert!(m.start(id));
        assert!(m.end(id, 5000));
    }
}
