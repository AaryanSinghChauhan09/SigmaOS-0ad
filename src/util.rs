//! 0ad model — 0 A.D. game with advanced features.

use std::collections::HashMap;
use std::time::{Duration, SystemTime};

#[derive(Debug, Clone, PartialEq)]
pub enum GameMode {
    Skirmish,
    RandomMap,
    Scenario,
    Multiplayer,
    Tutorial,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GameState {
    Lobby,
    Loading,
    Playing,
    Paused,
    Ended,
    Abandoned,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PlayerStatus {
    Connected,
    Disconnected,
    Ready,
    NotReady,
    Spectator,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Civilization {
    Athenian,
    Spartan,
    Persian,
    Seleucid,
    Ptolemaic,
    Carthaginian,
    Roman,
    Celtic,
    Iberian,
    Kushite,
    Han,
    Nomad,
    Achaemenid,
    Indian,
    Chinese,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub id: u32,
    pub name: String,
    pub civilization: Civilization,
    pub team: Option<u32>,
    pub status: PlayerStatus,
    pub resources: Resources,
    pub units: Vec<Unit>,
    pub buildings: Vec<Building>,
    pub score: u64,
    pub kills: u32,
    pub losses: u32,
    pub resources_gathered: u64,
    pub units_trained: u32,
    pub buildings_constructed: u32,
    pub is_ai: bool,
    pub difficulty: Option<AIDifficulty>,
}

#[derive(Debug, Clone)]
pub struct Resources {
    pub food: u64,
    pub wood: u64,
    pub stone: u64,
    pub metal: u64,
    pub population: u32,
    pub population_cap: u32,
}

#[derive(Debug, Clone)]
pub struct Unit {
    pub id: u64,
    pub type_: UnitType,
    pub position: (f64, f64),
    pub health: u32,
    pub max_health: u32,
    pub state: UnitState,
    pub owner: u32,
    pub target: Option<u64>,
    pub experience: u32,
    pub level: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnitType {
    CitizenSoldier,
    Infantry,
    Cavalry,
    Archer,
    Siege,
    Naval,
    Elephant,
    Hero,
    TradeCart,
    FishingBoat,
    FishingShip,
    TraderShip,
    FishingTrap,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnitState {
    Idle,
    Moving,
    Attacking,
    Gathering,
    Building,
    Training,
    Dead,
}

#[derive(Debug, Clone)]
pub struct Building {
    pub id: u64,
    pub type_: BuildingType,
    pub position: (f64, f64),
    pub health: u32,
    pub max_health: u32,
    pub state: BuildingState,
    pub owner: u32,
    pub progress: u32,
    pub queue: Vec<TrainingItem>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BuildingType {
    CivilCenter,
    House,
    Barracks,
    Stable,
    ArcheryRange,
    Fortress,
    Temple,
    Market,
    Dock,
    Farmstead,
    Storehouse,
    Wonder,
    Wall,
    Gate,
    Tower,
    Lighthouse,
    TradingPost,
    Field,
    Mine,
    LumberMill,
    WaterMill,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BuildingState {
    Constructing,
    Complete,
    Destroyed,
    Ruined,
}

#[derive(Debug, Clone)]
pub struct TrainingItem {
    pub unit_type: UnitType,
    pub count: u32,
    pub time_remaining: Duration,
    pub total_time: Duration,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AIDifficulty {
    VeryEasy,
    Easy,
    Medium,
    Hard,
    VeryHard,
}

#[derive(Debug, Clone)]
pub struct GameSession {
    pub id: u64,
    pub name: String,
    pub mode: GameMode,
    pub state: GameState,
    pub map_name: String,
    pub map_size: (u32, u32),
    pub players: Vec<Player>,
    pub spectator: bool,
    pub started_at: Option<SystemTime>,
    pub ended_at: Option<SystemTime>,
    pub duration: Duration,
    pub win_condition: WinCondition,
    pub victory_player: Option<u32>,
    pub is_host: bool,
    pub password: Option<String>,
    pub max_players: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WinCondition {
    Conquest,
    EconomicVictory,
    WonderVictory,
    TimeLimit,
    LastStanding,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub sessions: HashMap<u64, GameSession>,
    pub next_session_id: u64,
    pub next_player_id: u32,
    pub next_unit_id: u64,
    pub next_building_id: u64,
    pub current_session: Option<u64>,
    pub config: ModelConfig,
}

#[derive(Debug, Clone)]
pub struct ModelConfig {
    pub max_sessions: usize,
    pub max_players_per_session: u32,
    pub max_units_per_player: usize,
    pub max_buildings_per_player: usize,
    pub game_tick_rate: u32,
    pub resource_regeneration_rate: f64,
    pub unit_training_time_multiplier: f64,
    pub building_construction_time_multiplier: f64,
    pub enable_replay: bool,
    pub enable_spectator: bool,
    pub save_game_interval: Duration,
    pub auto_save: bool,
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            max_sessions: 100,
            max_players_per_session: 8,
            max_units_per_player: 500,
            max_buildings_per_player: 200,
            game_tick_rate: 20,
            resource_regeneration_rate: 1.0,
            unit_training_time_multiplier: 1.0,
            building_construction_time_multiplier: 1.0,
            enable_replay: true,
            enable_spectator: true,
            save_game_interval: Duration::from_secs(300), // 5 minutes
            auto_save: true,
        }
    }
}

impl Default for Model {
    fn default() -> Self {
        Self {
            sessions: HashMap::new(),
            next_session_id: 1,
            next_player_id: 1,
            next_unit_id: 1,
            next_building_id: 1,
            current_session: None,
            config: ModelConfig::default(),
        }
    }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn create_session(&mut self, name: &str, mode: GameMode, map_name: &str, map_size: (u32, u32)) -> u64 {
        let id = self.next_session_id;
        self.next_session_id += 1;
        
        let session = GameSession {
            id,
            name: name.into(),
            mode,
            state: GameState::Lobby,
            map_name: map_name.into(),
            map_size,
            players: vec![],
            spectator: false,
            started_at: None,
            ended_at: None,
            duration: Duration::ZERO,
            win_condition: WinCondition::Conquest,
            victory_player: None,
            is_host: true,
            password: None,
            max_players: self.config.max_players_per_session,
        };
        
        self.sessions.insert(id, session);
        self.current_session = Some(id);
        id
    }

    pub fn join_session(&mut self, session_id: u64, player_name: &str, civilization: Civilization) -> Option<u32> {
        let session = self.sessions.get_mut(&session_id)?;
        
        if session.players.len() >= session.max_players as usize {
            return None;
        }
        
        if session.state != GameState::Lobby {
            return None;
        }
        
        let player_id = self.next_player_id;
        self.next_player_id += 1;
        
        let player = Player {
            id: player_id,
            name: player_name.into(),
            civilization,
            team: None,
            status: PlayerStatus::Connected,
            resources: Resources {
                food: 200,
                wood: 200,
                stone: 100,
                metal: 100,
                population: 1,
                population_cap: 20,
            },
            units: vec![],
            buildings: vec![],
            score: 0,
            kills: 0,
            losses: 0,
            resources_gathered: 0,
            units_trained: 0,
            buildings_constructed: 0,
            is_ai: false,
            difficulty: None,
        };
        
        session.players.push(player);
        Some(player_id)
    }

    pub fn leave_session(&mut self, session_id: u64, player_id: u32) -> bool {
        if let Some(session) = self.sessions.get_mut(&session_id) {
            if let Some(index) = session.players.iter().position(|p| p.id == player_id) {
                session.players.remove(index);
                return true;
            }
        }
        false
    }

    pub fn start_game(&mut self, session_id: u64) -> bool {
        let session = match self.sessions.get_mut(&session_id) {
            Some(s) => s,
            None => return false,
        };
        
        if session.state != GameState::Lobby {
            return false;
        }
        
        if session.players.is_empty() {
            return false;
        }
        
        session.state = GameState::Playing;
        session.started_at = Some(SystemTime::now());
        true
    }

    pub fn end_game(&mut self, session_id: u64, winner: Option<u32>) -> bool {
        let session = match self.sessions.get_mut(&session_id) {
            Some(s) => s,
            None => return false,
        };
        
        if session.state != GameState::Playing {
            return false;
        }
        
        session.state = GameState::Ended;
        session.ended_at = Some(SystemTime::now());
        session.victory_player = winner;
        
        if let Some(start) = session.started_at {
            session.duration = session.ended_at.unwrap().duration_since(start).unwrap_or(Duration::ZERO);
        }
        
        true
    }

    pub fn pause_game(&mut self, session_id: u64) -> bool {
        let session = match self.sessions.get_mut(&session_id) {
            Some(s) => s,
            None => return false,
        };
        
        if session.state == GameState::Playing {
            session.state = GameState::Paused;
            return true;
        }
        false
    }

    pub fn resume_game(&mut self, session_id: u64) -> bool {
        let session = match self.sessions.get_mut(&session_id) {
            Some(s) => s,
            None => return false,
        };
        
        if session.state == GameState::Paused {
            session.state = GameState::Playing;
            return true;
        }
        false
    }

    pub fn add_unit(&mut self, session_id: u64, player_id: u32, unit_type: UnitType, position: (f64, f64)) -> Option<u64> {
        let session = self.sessions.get_mut(&session_id)?;
        let player = session.players.iter_mut().find(|p| p.id == player_id)?;
        
        if player.units.len() >= self.config.max_units_per_player {
            return None;
        }
        
        let unit_id = self.next_unit_id;
        self.next_unit_id += 1;
        
        let unit = Unit {
            id: unit_id,
            type_: unit_type,
            position,
            health: 100,
            max_health: 100,
            state: UnitState::Idle,
            owner: player_id,
            target: None,
            experience: 0,
            level: 1,
        };
        
        player.units.push(unit);
        Some(unit_id)
    }

    pub fn remove_unit(&mut self, session_id: u64, player_id: u32, unit_id: u64) -> bool {
        let session = match self.sessions.get_mut(&session_id) {
            Some(s) => s,
            None => return false,
        };
        
        let player = match session.players.iter_mut().find(|p| p.id == player_id) {
            Some(p) => p,
            None => return false,
        };
        
        if let Some(index) = player.units.iter().position(|u| u.id == unit_id) {
            player.units.remove(index);
            return true;
        }
        false
    }

    pub fn add_building(&mut self, session_id: u64, player_id: u32, building_type: BuildingType, position: (f64, f64)) -> Option<u64> {
        let session = self.sessions.get_mut(&session_id)?;
        let player = session.players.iter_mut().find(|p| p.id == player_id)?;
        
        if player.buildings.len() >= self.config.max_buildings_per_player {
            return None;
        }
        
        let building_id = self.next_building_id;
        self.next_building_id += 1;
        
        let building = Building {
            id: building_id,
            type_: building_type,
            position,
            health: 100,
            max_health: 100,
            state: BuildingState::Constructing,
            owner: player_id,
            progress: 0,
            queue: vec![],
        };
        
        player.buildings.push(building);
        Some(building_id)
    }

    pub fn remove_building(&mut self, session_id: u64, player_id: u32, building_id: u64) -> bool {
        let session = match self.sessions.get_mut(&session_id) {
            Some(s) => s,
            None => return false,
        };
        
        let player = match session.players.iter_mut().find(|p| p.id == player_id) {
            Some(p) => p,
            None => return false,
        };
        
        if let Some(index) = player.buildings.iter().position(|b| b.id == building_id) {
            player.buildings.remove(index);
            return true;
        }
        false
    }

    pub fn update_resources(&mut self, session_id: u64, player_id: u32, food: u64, wood: u64, stone: u64, metal: u64) -> bool {
        let session = match self.sessions.get_mut(&session_id) {
            Some(s) => s,
            None => return false,
        };
        
        let player = match session.players.iter_mut().find(|p| p.id == player_id) {
            Some(p) => p,
            None => return false,
        };
        
        player.resources.food += food;
        player.resources.wood += wood;
        player.resources.stone += stone;
        player.resources.metal += metal;
        player.resources_gathered += food + wood + stone + metal;
        true
    }

    pub fn update_population(&mut self, session_id: u64, player_id: u32, population: u32) -> bool {
        let session = match self.sessions.get_mut(&session_id) {
            Some(s) => s,
            None => return false,
        };
        
        let player = match session.players.iter_mut().find(|p| p.id == player_id) {
            Some(p) => p,
            None => return false,
        };
        
        if population <= player.resources.population_cap {
            player.resources.population = population;
            return true;
        }
        false
    }

    pub fn get_session(&self, session_id: u64) -> Option<&GameSession> {
        self.sessions.get(&session_id)
    }

    pub fn get_mut_session(&mut self, session_id: u64) -> Option<&mut GameSession> {
        self.sessions.get_mut(&session_id)
    }

    pub fn get_player(&self, session_id: u64, player_id: u32) -> Option<&Player> {
        self.sessions.get(&session_id)
            .and_then(|s| s.players.iter().find(|p| p.id == player_id))
    }

    pub fn get_mut_player(&mut self, session_id: u64, player_id: u32) -> Option<&mut Player> {
        self.sessions.get_mut(&session_id)
            .and_then(|s| s.players.iter_mut().find(|p| p.id == player_id))
    }

    pub fn get_unit(&self, session_id: u64, player_id: u32, unit_id: u64) -> Option<&Unit> {
        self.get_player(session_id, player_id)
            .and_then(|p| p.units.iter().find(|u| u.id == unit_id))
    }

    pub fn get_building(&self, session_id: u64, player_id: u32, building_id: u64) -> Option<&Building> {
        self.get_player(session_id, player_id)
            .and_then(|p| p.buildings.iter().find(|b| b.id == building_id))
    }

    pub fn set_win_condition(&mut self, session_id: u64, condition: WinCondition) -> bool {
        if let Some(session) = self.sessions.get_mut(&session_id) {
            session.win_condition = condition;
            return true;
        }
        false
    }

    pub fn set_password(&mut self, session_id: u64, password: Option<String>) -> bool {
        if let Some(session) = self.sessions.get_mut(&session_id) {
            session.password = password;
            return true;
        }
        false
    }

    pub fn set_max_players(&mut self, session_id: u64, max_players: u32) -> bool {
        if let Some(session) = self.sessions.get_mut(&session_id) {
            session.max_players = max_players;
            return true;
        }
        false
    }

    pub fn delete_session(&mut self, session_id: u64) -> bool {
        self.sessions.remove(&session_id).is_some()
    }

    pub fn get_active_sessions(&self) -> Vec<&GameSession> {
        self.sessions.values()
            .filter(|s| s.state == GameState::Playing || s.state == GameState::Lobby)
            .collect()
    }

    pub fn get_sessions_by_state(&self, state: &GameState) -> Vec<&GameSession> {
        self.sessions.values()
            .filter(|s| &s.state == state)
            .collect()
    }

    pub fn sessions(&self) -> &HashMap<u64, GameSession> { &self.sessions }
    pub fn next_session_id(&self) -> u64 { self.next_session_id }
    pub fn next_player_id(&self) -> u32 { self.next_player_id }
    pub fn next_unit_id(&self) -> u64 { self.next_unit_id }
    pub fn next_building_id(&self) -> u64 { self.next_building_id }
    pub fn current_session(&self) -> Option<u64> { self.current_session }
    pub fn config(&self) -> &ModelConfig { &self.config }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_ad_basic_operations() {
        let mut m = Model::new();
        assert!(m.sessions.is_empty());
        assert_eq!(m.current_session(), None);
    }

    #[test]
    fn zero_ad_session_management() {
        let mut m = Model::new();
        let session_id = m.create_session("Test Game", GameMode::Skirmish, "Maps/Random", (200, 200));
        assert!(m.sessions.contains_key(&session_id));
        assert_eq!(m.current_session(), Some(session_id));
        assert!(m.delete_session(session_id));
        assert!(!m.sessions.contains_key(&session_id));
    }

    #[test]
    fn zero_ad_player_management() {
        let mut m = Model::new();
        let session_id = m.create_session("Test Game", GameMode::Skirmish, "Maps/Random", (200, 200));
        let player_id = m.join_session(session_id, "Player1", Civilization::Athenian);
        assert!(player_id.is_some());
        assert!(m.get_player(session_id, player_id.unwrap()).is_some());
        assert!(m.leave_session(session_id, player_id.unwrap()));
        assert!(m.get_player(session_id, player_id.unwrap()).is_none());
    }

    #[test]
    fn zero_ad_game_lifecycle() {
        let mut m = Model::new();
        let session_id = m.create_session("Test Game", GameMode::Skirmish, "Maps/Random", (200, 200));
        let player_id = m.join_session(session_id, "Player1", Civilization::Athenian).unwrap();
        assert!(m.start_game(session_id));
        assert_eq!(m.get_session(session_id).unwrap().state, GameState::Playing);
        assert!(m.pause_game(session_id));
        assert_eq!(m.get_session(session_id).unwrap().state, GameState::Paused);
        assert!(m.resume_game(session_id));
        assert_eq!(m.get_session(session_id).unwrap().state, GameState::Playing);
        assert!(m.end_game(session_id, Some(player_id)));
        assert_eq!(m.get_session(session_id).unwrap().state, GameState::Ended);
    }

    #[test]
    fn zero_ad_unit_management() {
        let mut m = Model::new();
        let session_id = m.create_session("Test Game", GameMode::Skirmish, "Maps/Random", (200, 200));
        let player_id = m.join_session(session_id, "Player1", Civilization::Athenian).unwrap();
        let unit_id = m.add_unit(session_id, player_id, UnitType::CitizenSoldier, (100.0, 100.0));
        assert!(unit_id.is_some());
        assert!(m.get_unit(session_id, player_id, unit_id.unwrap()).is_some());
        assert!(m.remove_unit(session_id, player_id, unit_id.unwrap()));
        assert!(m.get_unit(session_id, player_id, unit_id.unwrap()).is_none());
    }

    #[test]
    fn zero_ad_building_management() {
        let mut m = Model::new();
        let session_id = m.create_session("Test Game", GameMode::Skirmish, "Maps/Random", (200, 200));
        let player_id = m.join_session(session_id, "Player1", Civilization::Athenian).unwrap();
        let building_id = m.add_building(session_id, player_id, BuildingType::CivilCenter, (50.0, 50.0));
        assert!(building_id.is_some());
        assert!(m.get_building(session_id, player_id, building_id.unwrap()).is_some());
        assert!(m.remove_building(session_id, player_id, building_id.unwrap()));
        assert!(m.get_building(session_id, player_id, building_id.unwrap()).is_none());
    }

    #[test]
    fn zero_ad_resource_management() {
        let mut m = Model::new();
        let session_id = m.create_session("Test Game", GameMode::Skirmish, "Maps/Random", (200, 200));
        let player_id = m.join_session(session_id, "Player1", Civilization::Athenian).unwrap();
        assert!(m.update_resources(session_id, player_id, 100, 50, 25, 10));
        let player = m.get_player(session_id, player_id).unwrap();
        assert_eq!(player.resources.food, 300); // 200 + 100
        assert_eq!(player.resources.wood, 250); // 200 + 50
        assert_eq!(player.resources.stone, 125); // 100 + 25
        assert_eq!(player.resources.metal, 110); // 100 + 10
    }

    #[test]
    fn zero_ad_population_management() {
        let mut m = Model::new();
        let session_id = m.create_session("Test Game", GameMode::Skirmish, "Maps/Random", (200, 200));
        let player_id = m.join_session(session_id, "Player1", Civilization::Athenian).unwrap();
        assert!(m.update_population(session_id, player_id, 15));
        let player = m.get_player(session_id, player_id).unwrap();
        assert_eq!(player.resources.population, 15);
        assert!(!m.update_population(session_id, player_id, 25)); // Exceeds cap
    }

    #[test]
    fn zero_ad_settings() {
        let mut m = Model::new();
        let session_id = m.create_session("Test Game", GameMode::Skirmish, "Maps/Random", (200, 200));
        assert!(m.set_win_condition(session_id, WinCondition::EconomicVictory));
        assert!(m.set_password(session_id, Some("secret".into())));
        assert!(m.set_max_players(session_id, 4));
        let session = m.get_session(session_id).unwrap();
        assert_eq!(session.win_condition, WinCondition::EconomicVictory);
        assert_eq!(session.password, Some("secret".into()));
        assert_eq!(session.max_players, 4);
    }
}
