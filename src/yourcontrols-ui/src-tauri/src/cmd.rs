use serde::{Deserialize, Serialize};
use semver::Version;

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
    TestNetwork {
        port: i64
    },
    UiReady,
    InstallAircraft {
        names: Vec<String>,
    },
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Aircraft {
    pub newest_version: Option<Version>,
    pub installed_version: Option<Version>,
    pub install_locked: bool,
    pub name: String,
    pub author: String
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged, rename_all = "camelCase")]
pub enum UIEvents {
    StartUpText {
        text: String,
    },
    InitData {
        version: String,
        aircraft: Vec<Aircraft>
    },
    LoadingComplete,
    NetworkTestResult {
        test: TestNetworkResult,
        status: ResultStatus,
    },
}
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum TestNetworkResult {
    CloudServer,
    CloudServerP2P,
    UPnP,
    Direct,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ResultStatus {
    Pending {},
    Error { reason: String },
    Success {},
}


#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", tag="cmd")]
pub enum GameUiPayloads {
    Host {
        port: Option<u16>,
        username: String,
    },
    Join {
        port: Option<u16>,
        session_code: Option<String>,
        server_ip: Option<String>,
        username: String,
    },
    NetworkStatistics {
        ping: usize,
        upload: usize,
        download: usize,
    },
    LobbyInfo {
        session_code: Option<String>,
        server_ip: Option<String>,
        clients: Option<Vec<String>>,
    },
    LobbySettings {
        new_connection_as_obs: Option<bool>,
        co_pilot_throttle_control: Option<bool>,
        co_pilot_flight_surfaces_control: Option<bool>,
    },
    ChatMessage {
        uuid: Option<String>,
        client: String,
        message: String,
        time: String,
        pinned: Option<bool>
    },
    Connected,
    Disconnected
}