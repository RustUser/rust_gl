use std::time::Duration;
use steamworks::{AppId, CallbackHandle, LobbyId, P2PSessionRequest};
use crate::steam_manager::default_steam_manager::DefaultSteamManager;
use crate::steam_manager::steam_messaging::{ChatState, JoinLobbyCallback, States, SteamMessaging};
use crate::steam_manager::SteamManager;

pub struct SteamBundle {
    steam_manager: DefaultSteamManager,
    messaging: SteamMessaging,
    request_callback: CallbackHandle
}

impl SteamBundle {
    pub fn new(app_id: AppId) -> SteamBundle {
        let steam = DefaultSteamManager::new(app_id, Duration::default(), false);
        let mut messaging = SteamMessaging::new(steam.client());

        let sender_accept = messaging.local_sender_accept();

        let request_callback = steam.client().register_callback(move |request: P2PSessionRequest| {
            println!("Peer accepted!");
            sender_accept.send(request.remote).unwrap();
        });

        Self {
            steam_manager: steam,
            messaging,
            request_callback
        }
    }

    pub fn update(&self, join_lobby_cb: Box<JoinLobbyCallback>) {
        self.steam_manager.run_callbacks();
        let recv_create_lobby = self.messaging.receive_create_lobby();
        let state = self.messaging.state();
        let steam_id = self.steam_manager.client().user().steam_id();

        if let Ok(recv) = recv_create_lobby.try_recv() {
            *state.lock().unwrap() = States::Chat(ChatState {
                own_id: steam_id,
                current_lobby: recv,
                is_host: true,
                host_id: steam_id,
                peers: vec![steam_id],
                message: "".to_string(),
                messages: vec![]
            });
            for callback in &self.messaging.create_lobby_callbacks {
                (*callback)(recv);
            }
        }
        let recv_join_lobby = self.messaging.receive_join_lobby();
        if let Ok(recv) = recv_join_lobby.try_recv() {
            let host_id = self.steam_manager.client().matchmaking().lobby_owner(recv);
            *state.lock().unwrap() = States::Chat(ChatState {
                own_id: steam_id,
                current_lobby: recv,
                is_host: false,
                host_id,
                peers: vec![],
                message: "".to_string(),
                messages: vec![]
            });
            join_lobby_cb(recv);
            for callback in &self.messaging.join_lobby_callbacks {
                (*callback)(recv);
            }
        }
    }

    pub fn register_join_callback(&mut self, c: Box<JoinLobbyCallback>) {
        self.messaging.register_join_lobby_callback(c);
    }

    pub fn steam_manager(&self) -> &DefaultSteamManager {
        &self.steam_manager
    }
    pub fn messaging(&self) -> &SteamMessaging {
        &self.messaging
    }
}