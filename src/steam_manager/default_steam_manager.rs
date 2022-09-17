use std::borrow::Borrow;
use std::sync::{Arc, LockResult, Mutex};
use std::thread::{JoinHandle, sleep, spawn};
use std::time::Duration;
use steamworks::{AppId, Client, ClientManager, SingleClient, Utils};
use crate::steam_manager::SteamManager;

pub struct DefaultSteamManager {
    app_id: AppId,
    client: Client<ClientManager>,
    single: Arc<Mutex<SingleClient<ClientManager>>>,
    utils: Utils<ClientManager>,
    running: Arc<Mutex<bool>>,
    handler: Option<JoinHandle<()>>
}

impl DefaultSteamManager {
    pub fn new(app_id: AppId, delay: Duration, start_on_create: bool) -> DefaultSteamManager {
        let (client, single) = Client::init_app(app_id).unwrap();
        let single = Arc::new(Mutex::new(single));
        let utils = client.utils();
        let running = Arc::new(Mutex::new(true));

        let mut app = Self {
            app_id,
            client,
            single,
            utils,
            running,
            handler: None
        };
        if start_on_create {
            app.start(delay);
        }
        app
    }
}

impl Drop for DefaultSteamManager {
    fn drop(&mut self) {
        *self.running.lock().unwrap() = false;
    }
}

impl SteamManager for DefaultSteamManager {
    fn app_id(&self) -> &AppId {
        &self.app_id
    }

    fn client(&self) -> &Client<ClientManager> {
        &self.client
    }

    fn utils(&self) -> &Utils<ClientManager> {
        &self.utils
    }

    fn stop(&mut self) {
        *self.running.lock().unwrap() = false;
    }

    fn start(&mut self, delay: Duration) {
        if *self.running.lock().unwrap() {
            return;
        }

        *self.running.lock().unwrap() = true;

        let handler = {
            let running = self.running.clone();
            let single = self.single.clone();
            let handler = spawn(move || {
                while *running.lock().unwrap() {
                    println!("Listening.");
                    single.lock().unwrap().run_callbacks();
                    sleep(delay);
                }
            });
            handler
        };
        self.handler = Some(handler);
    }

    fn run_callbacks(&self) {
        self.single.lock().unwrap().run_callbacks();
    }
}