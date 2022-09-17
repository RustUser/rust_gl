use std::sync::*;
use std::sync::mpsc::{Receiver, Sender, SendError};
use std::thread::JoinHandle;
use serde_json::ser::State;
use steamworks::{Client, ClientManager, LobbyId, P2PSessionRequest, SteamId};

pub struct MenuState {
    lobby_input: String,
}

impl MenuState {
    pub fn lobby_input(&self) -> &String {
        &self.lobby_input
    }

    pub fn lobby_input_mut(&mut self) -> &mut String {
        &mut self.lobby_input
    }
}

pub struct ChatState {
    pub(crate) own_id: SteamId,
    pub(crate) current_lobby: LobbyId,
    pub(crate) is_host: bool,
    pub(crate) host_id: SteamId,
    pub(crate) peers: Vec<SteamId>,
    pub(crate) message: String,
    pub(crate) messages: Vec<String>,
}

impl ChatState {
    pub fn own_id(&self) -> SteamId {
        self.own_id
    }
    pub fn current_lobby(&self) -> LobbyId {
        self.current_lobby
    }
    pub fn is_host(&self) -> bool {
        self.is_host
    }
    pub fn host_id(&self) -> SteamId {
        self.host_id
    }
    pub fn peers(&self) -> &Vec<SteamId> {
        &self.peers
    }

    pub fn peers_mut(&mut self) -> &mut Vec<SteamId> {
        &mut self.peers
    }

    pub fn message(&self) -> &str {
        &self.message
    }
    pub fn message_mut(&mut self) -> &mut String {
        &mut self.message
    }
    pub fn messages(&self) -> &Vec<String> {
        &self.messages
    }
    pub fn messages_mut(&mut self) -> &mut Vec<String> {
        &mut self.messages
    }
}

pub enum States {
    Menu(MenuState),
    Chat(ChatState),
}

pub struct Channels<S, R> {
    sender: Sender<S>,
    receiver: Receiver<R>,
}

pub type CreateLobbyCallback = dyn Fn(LobbyId);
pub type JoinLobbyCallback = dyn Fn(LobbyId);

pub struct SteamMessaging {
    create_lobby: Channels<LobbyId, LobbyId>,
    join_lobby: Channels<LobbyId, LobbyId>,
    accept: Channels<SteamId, SteamId>,
    messages: Vec<String>,
    state: Arc<Mutex<States>>,
    pub(crate) create_lobby_callbacks: Vec<Box<CreateLobbyCallback>>,
    pub(crate) join_lobby_callbacks: Vec<Box<JoinLobbyCallback>>,
}

impl SteamMessaging {
    pub fn new(client: &Client<ClientManager>) -> SteamMessaging {
        let mut state = States::Menu(MenuState {
            lobby_input: "".to_string()
        });

        let (s_create_lobby, r_create_lobby) = mpsc::channel();
        let (s_join_lobby, r_join_lobby) = mpsc::channel();
        let (s_accept, r_accept) = mpsc::channel();

        let create_lobby: Channels<LobbyId, LobbyId> = Channels {
            sender: s_create_lobby,
            receiver: r_create_lobby,
        };
        let join_lobby = Channels {
            sender: s_join_lobby,
            receiver: r_join_lobby,
        };

        let accept = Channels {
            sender: s_accept,
            receiver: r_accept,
        };

        Self {
            create_lobby,
            join_lobby,
            accept,
            messages: vec![],
            state: Arc::new(Mutex::new(state)),
            create_lobby_callbacks: vec![],
            join_lobby_callbacks: vec![],
        }
    }

    pub fn register_join_lobby_callback(&mut self, c: Box<JoinLobbyCallback>) {
        self.join_lobby_callbacks.push(c);
    }

    pub fn local_sender_create_lobby(&self) -> Sender<LobbyId> {
        self.create_lobby.sender.clone()
    }

    pub fn local_sender_accept(&self) -> Sender<SteamId> {
        self.accept.sender.clone()
    }

    pub fn receive_create_lobby(&self) -> &Receiver<LobbyId> {
        &self.create_lobby.receiver
    }

    pub fn receive_join_lobby(&self) -> &Receiver<LobbyId> {
        &self.join_lobby.receiver
    }

    pub fn receive_accept(&self) -> &Receiver<SteamId> {
        &self.accept.receiver
    }

    pub fn local_sender_join_lobby(&self) -> Sender<LobbyId> {
        self.join_lobby.sender.clone()
    }

    pub fn state(&self) -> Arc<Mutex<States>> {
        self.state.clone()
    }

    pub fn on_create_lobby(&self, lobby: LobbyId) -> Result<(), SendError<LobbyId>> {
        self.create_lobby.sender.send(lobby)
    }
    pub fn messages(&self) -> &Vec<String> {
        &self.messages
    }
}