use std::time::Duration;
use steamworks::{AppId, Callback, CallbackHandle, Client, ClientManager, Friend, FriendFlags, Friends, Manager, SingleClient, SteamId, Utils};

pub mod default_steam_manager;
pub mod steam_ext;
pub mod steam_messaging;
pub mod steam_bundle;

pub trait SteamManager {
    fn app_id(&self) -> &AppId;
    fn client(&self) -> &Client<ClientManager>;
    fn utils(&self) -> &Utils<ClientManager>;

    fn app_installed(&self, app_id: AppId) -> bool {
        self.client().apps().is_subscribed_app(app_id)
    }
    fn install_dir(&self, app_id: AppId) -> String {
        self.client().apps().app_install_dir(app_id)
    }
    fn friends(&self) -> Friends<ClientManager> {
        self.client().friends()
    }
    fn get_friends(&self, flags: FriendFlags) -> Vec<Friend<ClientManager>> {
        self.friends().get_friends(flags)
    }
    fn get_friend(&self, steam_id: SteamId) -> Friend<ClientManager> {
        self.friends().get_friend(steam_id)
    }
    fn immediate_friends(&self) -> Vec<Friend<ClientManager>> {
        self.friends().get_friends(FriendFlags::IMMEDIATE)
    }
    fn stop(&mut self);
    fn start(&mut self, delay: Duration);
    fn run_callbacks(&self);
}