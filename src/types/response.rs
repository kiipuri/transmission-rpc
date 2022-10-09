use serde::Deserialize;
#[derive(Deserialize, Debug)]
pub struct RpcResponse<T: RpcResponseArgument> {
    pub arguments: T,
    pub result: String,
}

impl<T: RpcResponseArgument> RpcResponse<T> {
    pub fn is_ok(&self) -> bool {
        self.result == "success"
    }
}
pub trait RpcResponseArgument {}

#[derive(Deserialize, Debug, Clone)]
pub struct SessionGet {
    #[serde(rename = "alt-speed-down")]
    pub alt_speed_down: i64,
    #[serde(rename = "alt-speed-enabled")]
    pub alt_speed_enabled: bool,
    #[serde(rename = "alt-speed-up")]
    pub alt_speed_up: i64,
    #[serde(rename = "blocklist-enabled")]
    pub blocklist_enabled: bool,
    #[serde(rename = "download-dir")]
    pub download_dir: String,
    pub encryption: String,
    #[serde(rename = "rpc-version-minimum")]
    pub rpc_version_minimum: i32,
    #[serde(rename = "rpc-version")]
    pub rpc_version: i32,
    pub version: String,
}
impl RpcResponseArgument for SessionGet {}

#[derive(Deserialize, Debug, Clone)]
pub struct SessionStats {
    #[serde(rename = "torrentCount")]
    pub torrent_count: i32,
    #[serde(rename = "activeTorrentCount")]
    pub active_torrent_count: i32,
    #[serde(rename = "pausedTorrentCount")]
    pub paused_torrent_count: i32,
    #[serde(rename = "downloadSpeed")]
    pub download_speed: i64,
    #[serde(rename = "uploadSpeed")]
    pub upload_speed: i64,
    #[serde(rename = "current-stats")]
    pub current_stats: Stats,
    #[serde(rename = "cumulative-stats")]
    pub cumulative_stats: Stats,
}
impl RpcResponseArgument for SessionStats {}

#[derive(Deserialize, Debug, Clone)]
pub struct SessionClose {}
impl RpcResponseArgument for SessionClose {}

#[derive(Deserialize, Debug, Clone)]
pub struct BlocklistUpdate {
    #[serde(rename = "blocklist-size")]
    pub blocklist_size: Option<i32>,
}
impl RpcResponseArgument for BlocklistUpdate {}

#[derive(Deserialize, Debug, Clone)]
pub struct FreeSpace {
    pub path: String,
    #[serde(rename = "size-bytes")]
    pub size_bytes: i64,
}
impl RpcResponseArgument for FreeSpace {}

#[derive(Deserialize, Debug, Clone)]
pub struct PortTest {
    #[serde(rename = "port-is-open")]
    pub port_is_open: bool,
}
impl RpcResponseArgument for PortTest {}

#[derive(Deserialize, Debug, Clone)]
pub struct Torrent {
    #[serde(rename = "activityDate")]
    pub activity_date: Option<i64>,
    #[serde(rename = "addedDate")]
    pub added_date: Option<i64>,
    // TODO: what
    // pub availability: Option<i64>,
    #[serde(rename = "bandwidthPriority")]
    pub bandwidth_priority: Option<i64>,
    pub comment: Option<String>,
    #[serde(rename = "corruptEver")]
    pub corrupt_ever: Option<i64>,
    pub creator: Option<String>,
    #[serde(rename = "dateCreated")]
    pub date_created: Option<i64>,
    #[serde(rename = "desiredAvailable")]
    pub desired_available: Option<i64>,
    #[serde(rename = "doneDate")]
    pub done_date: Option<i64>,
    #[serde(rename = "downloadDir")]
    pub download_dir: Option<String>,
    #[serde(rename = "downloadedEver")]
    pub downloaded_ever: Option<i64>,
    #[serde(rename = "downloadLimit")]
    pub download_limit: Option<i64>,
    #[serde(rename = "downloadLimited")]
    pub download_limited: Option<bool>,
    #[serde(rename = "editDate")]
    pub edit_date: Option<i64>,
    #[serde(rename = "error")]
    pub error: Option<i64>,
    #[serde(rename = "errorString")]
    pub error_string: Option<String>,
    pub eta: Option<i64>,
    #[serde(rename = "etaIdle")]
    pub eta_idle: Option<i64>,
    #[serde(rename = "fileCount")]
    pub file_count: Option<i64>,
    pub files: Option<Vec<File>>,
    #[serde(rename = "fileStats")]
    pub file_stats: Option<Vec<FileStat>>,
    // TODO: returns 0
    // pub group: Option<String>,
    #[serde(rename = "hashString")]
    pub hash_string: Option<String>,
    #[serde(rename = "haveUnchecked")]
    pub have_unchecked: Option<i64>,
    #[serde(rename = "haveValid")]
    pub have_valid: Option<i64>,
    #[serde(rename = "honorsSessionLimits")]
    pub honors_session_limits: Option<bool>,
    pub id: Option<i64>,
    #[serde(rename = "isFinished")]
    pub is_finished: Option<bool>,
    #[serde(rename = "isPrivate")]
    pub is_private: Option<bool>,
    #[serde(rename = "isStalled")]
    pub is_stalled: Option<bool>,
    // TODO: returns 0
    // pub labels: Option<Vec<String>>,
    #[serde(rename = "leftUntilDone")]
    pub left_until_done: Option<i64>,
    #[serde(rename = "magnetLink")]
    pub magnet_link: Option<String>,
    #[serde(rename = "manualAnnounceTime")]
    pub manual_announce_time: Option<i64>,
    #[serde(rename = "maxConnectedPeers")]
    pub max_connected_peers: Option<i64>,
    #[serde(rename = "metadataPercentComplete")]
    pub metadata_percent_complete: Option<f32>,
    pub name: Option<String>,
    #[serde(rename = "peerLimit")]
    pub peer_limit: Option<i64>,
    pub peers: Option<Vec<Peer>>,
    #[serde(rename = "peersConnected")]
    pub peers_connected: Option<i64>,
    #[serde(rename = "peersFrom")]
    pub peers_from: Option<PeersFrom>,
    #[serde(rename = "peersGettingFromUs")]
    pub peers_getting_from_us: Option<i64>,
    #[serde(rename = "peersSendingToUs")]
    pub peers_sending_to_us: Option<i64>,
    #[serde(rename = "percentComplete")]
    pub percent_complete: Option<f32>,
    #[serde(rename = "percentDone")]
    pub percent_done: Option<f32>,
    pub pieces: Option<String>,
    #[serde(rename = "pieceCount")]
    pub piece_count: Option<i64>,
    /// for each file in files, their download priority (low:-1,normal:0,high:1)
    pub priorities: Option<Vec<i8>>,
    #[serde(rename = "primary-mime-type")]
    pub primary_mime_type: Option<String>,
    #[serde(rename = "queuePosition")]
    pub queue_position: Option<i64>,
    #[serde(rename = "rateDownload")]
    pub rate_download: Option<i64>,
    #[serde(rename = "rateUpload")]
    pub rate_upload: Option<i64>,
    #[serde(rename = "recheckProgress")]
    pub recheck_progress: Option<f32>,
    #[serde(rename = "secondsDownloading")]
    pub seconds_downloading: Option<i64>,
    #[serde(rename = "secondsSeeding")]
    pub seconds_seeding: Option<i64>,
    #[serde(rename = "seedIdleLimit")]
    pub seed_idle_limit: Option<i64>,
    #[serde(rename = "seedIdleMode")]
    pub seed_idle_mode: Option<i64>,
    #[serde(rename = "seedRatioLimit")]
    pub seed_ratio_limit: Option<f32>,
    #[serde(rename = "seedRatioMode")]
    pub seed_ratio_mode: Option<i64>,
    #[serde(rename = "sizeWhenDone")]
    pub size_when_done: Option<i64>,
    #[serde(rename = "startDate")]
    pub start_date: Option<i64>,
    pub status: Option<i64>,
    pub trackers: Option<Vec<Trackers>>,
    // TODO: returns 0
    // #[serde(rename = "trackerList")]
    // pub tracker_list: Option<String>,
    #[serde(rename = "trackerStats")]
    pub tracker_stats: Option<Vec<TrackerStats>>,
    #[serde(rename = "totalSize")]
    pub total_size: Option<i64>,
    #[serde(rename = "torrentFile")]
    pub torrent_file: Option<String>,
    #[serde(rename = "uploadedEver")]
    pub uploaded_ever: Option<i64>,
    #[serde(rename = "uploadLimit")]
    pub upload_limit: Option<i64>,
    #[serde(rename = "uploadLimited")]
    pub upload_limited: Option<bool>,
    #[serde(rename = "uploadRatio")]
    pub upload_ratio: Option<f32>,
    /// for each file in files, whether or not they will be downloaded (0 or 1)
    pub wanted: Option<Vec<i8>>,
    pub webseeds: Option<Vec<String>>,
    #[serde(rename = "webseedsSendingToUs")]
    pub webseeds_sending_to_us: Option<i64>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Stats {
    #[serde(rename = "filesAdded")]
    pub files_added: i32,
    #[serde(rename = "downloadedBytes")]
    pub downloaded_bytes: i64,
    #[serde(rename = "uploadedBytes")]
    pub uploaded_bytes: i64,
    #[serde(rename = "secondsActive")]
    pub seconds_active: i64,
    #[serde(rename = "sessionCount")]
    pub session_count: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct Torrents<T> {
    pub torrents: Vec<T>,
}
impl RpcResponseArgument for Torrents<Torrent> {}

#[derive(Deserialize, Debug, Clone)]
pub struct Trackers {
    pub id: i32,
    pub announce: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TrackerStats {
    #[serde(rename = "announceState")]
    pub announce_state: Option<i64>,
    pub announce: Option<String>,
    #[serde(rename = "downloadCount")]
    pub download_count: Option<i64>,
    #[serde(rename = "hasAnnounced")]
    pub has_announced: Option<bool>,
    pub host: Option<String>,
    pub id: Option<i64>,
    #[serde(rename = "isBackup")]
    pub is_backup: Option<bool>,
    #[serde(rename = "lastAnnouncePeerCount")]
    pub last_announce_peer_count: Option<i64>,
    #[serde(rename = "lastAnnounceResult")]
    pub last_announce_result: Option<String>,
    #[serde(rename = "lastAnnounceStartTime")]
    pub last_announce_start_time: Option<i64>,
    #[serde(rename = "lastAnnounceSucceeded")]
    pub last_announce_succeeded: Option<bool>,
    #[serde(rename = "lastAnnounceTime")]
    pub last_announce_time: Option<i64>,
    #[serde(rename = "lastAnnounceTimedOut")]
    pub last_announce_timed_out: Option<bool>,
    #[serde(rename = "lastScrapeResult")]
    pub last_scrape_result: Option<String>,
    #[serde(rename = "lastScrapeStartTime")]
    pub last_scrape_start_time: Option<i64>,
    #[serde(rename = "lastScrapeSucceeded")]
    pub last_scrape_succeeded: Option<bool>,
    #[serde(rename = "lastScrapeTime")]
    pub last_scrape_time: Option<i64>,
    #[serde(rename = "lastScrapeTimedOut")]
    pub last_scrape_timed_out: Option<bool>,
    #[serde(rename = "leecherCount")]
    pub leecher_count: Option<i64>,
    #[serde(rename = "nextAnnounceTime")]
    pub next_announce_time: Option<i64>,
    #[serde(rename = "nextScrapeTime")]
    pub next_scrape_time: Option<i64>,
    #[serde(rename = "scrapeState")]
    pub scrape_state: Option<i64>,
    pub scrape: Option<String>,
    #[serde(rename = "seederCount")]
    pub seeder_count: Option<i64>,
    pub site_name: Option<String>,
    pub tier: Option<i64>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Peer {
    pub address: Option<String>,
    #[serde(rename = "clientName")]
    pub client_name: Option<String>,
    #[serde(rename = "clientIsChoked")]
    pub client_is_choked: Option<bool>,
    #[serde(rename = "clientIsInterested")]
    pub client_is_interested: Option<bool>,
    #[serde(rename = "flagStr")]
    pub flag_str: Option<String>,
    #[serde(rename = "isDownloadingFrom")]
    pub is_downloading_from: Option<bool>,
    #[serde(rename = "isEncrypted")]
    pub is_encrypted: Option<bool>,
    #[serde(rename = "isIncoming")]
    pub is_incoming: Option<bool>,
    #[serde(rename = "isUploadingTo")]
    pub is_uploading_to: Option<bool>,
    #[serde(rename = "isUTP")]
    pub is_utp: Option<bool>,
    #[serde(rename = "peerIsChoked")]
    pub peer_is_choked: Option<bool>,
    #[serde(rename = "peerIsInterested")]
    pub peer_is_interested: Option<bool>,
    pub port: Option<i64>,
    pub progress: Option<f32>,
    #[serde(rename = "rateToClient")]
    pub rate_to_client: Option<i64>,
    #[serde(rename = "rateToPeer")]
    pub rate_to_peer: Option<i64>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PeersFrom {
    #[serde(rename = "fromCache")]
    pub from_cache: Option<i64>,
    #[serde(rename = "fromDht")]
    pub from_dht: Option<i64>,
    #[serde(rename = "fromIncoming")]
    pub from_incoming: Option<i64>,
    #[serde(rename = "fromLpd")]
    pub from_lpd: Option<i64>,
    #[serde(rename = "fromLtep")]
    pub from_ltep: Option<i64>,
    #[serde(rename = "fromPex")]
    pub from_pex: Option<i64>,
    #[serde(rename = "fromTracker")]
    pub from_tracker: Option<i64>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct File {
    pub length: i64,
    #[serde(rename = "bytesCompleted")]
    pub bytes_completed: i64,
    pub name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FileStat {
    #[serde(rename = "bytesCompleted")]
    pub bytes_completed: i64,
    pub wanted: bool,
    /// low: -1, normal: 0, high: 1
    pub priority: i8,
}

#[derive(Deserialize, Debug)]
pub struct Nothing {}
impl RpcResponseArgument for Nothing {}

#[derive(Deserialize, Debug)]
pub struct TorrentAdded {
    #[serde(rename = "torrent-added")]
    pub torrent_added: Option<Torrent>,
}
impl RpcResponseArgument for TorrentAdded {}

#[derive(Deserialize, Debug)]
pub struct TorrentRenamePath {
    pub path: Option<String>,
    pub name: Option<String>,
    pub id: Option<i64>,
}
impl RpcResponseArgument for TorrentRenamePath {}
