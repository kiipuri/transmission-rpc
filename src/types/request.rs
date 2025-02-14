use enum_iterator::IntoEnumIterator;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct RpcRequest {
    method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    arguments: Option<Args>,
}

impl RpcRequest {
    pub fn session_get() -> RpcRequest {
        RpcRequest {
            method: String::from("session-get"),
            arguments: None,
        }
    }

    pub fn session_stats() -> RpcRequest {
        RpcRequest {
            method: String::from("session-stats"),
            arguments: None,
        }
    }

    pub fn session_close() -> RpcRequest {
        RpcRequest {
            method: String::from("session-close"),
            arguments: None,
        }
    }

    pub fn blocklist_update() -> RpcRequest {
        RpcRequest {
            method: String::from("blocklist-update"),
            arguments: None,
        }
    }

    pub fn free_space(path: String) -> RpcRequest {
        RpcRequest {
            method: String::from("free-space"),
            arguments: Some(Args::FreeSpaceArgs(FreeSpaceArgs { path })),
        }
    }

    pub fn port_test() -> RpcRequest {
        RpcRequest {
            method: String::from("port-test"),
            arguments: None,
        }
    }

    pub fn torrent_get(fields: Option<Vec<TorrentGetField>>, ids: Option<Vec<Id>>) -> RpcRequest {
        let string_fields = fields
            .unwrap_or(TorrentGetField::all())
            .iter()
            .map(|f| f.to_str())
            .collect();
        RpcRequest {
            method: String::from("torrent-get"),
            arguments: Some(Args::TorrentGetArgs(TorrentGetArgs {
                fields: Some(string_fields),
                ids,
            })),
        }
    }

    pub fn torrent_remove(ids: Vec<Id>, delete_local_data: bool) -> RpcRequest {
        RpcRequest {
            method: String::from("torrent-remove"),
            arguments: Some(Args::TorrentRemoveArgs(TorrentRemoveArgs {
                ids,
                delete_local_data,
            })),
        }
    }

    pub fn torrent_add(add: TorrentAddArgs) -> RpcRequest {
        RpcRequest {
            method: String::from("torrent-add"),
            arguments: Some(Args::TorrentAddArgs(add)),
        }
    }

    pub fn torrent_action(action: TorrentAction, ids: Vec<Id>) -> RpcRequest {
        RpcRequest {
            method: action.to_str(),
            arguments: Some(Args::TorrentActionArgs(TorrentActionArgs { ids })),
        }
    }

    pub fn torrent_set_location(
        ids: Vec<Id>,
        location: String,
        move_from: Option<bool>,
    ) -> RpcRequest {
        RpcRequest {
            method: String::from("torrent-set-location"),
            arguments: Some(Args::TorrentSetLocationArgs(TorrentSetLocationArgs {
                ids,
                location,
                move_from,
            })),
        }
    }

    pub fn torrent_rename_path(ids: Vec<Id>, path: String, name: String) -> RpcRequest {
        RpcRequest {
            method: String::from("torrent-rename-path"),
            arguments: Some(Args::TorrentRenamePathArgs(TorrentRenamePathArgs {
                ids,
                path,
                name,
            })),
        }
    }
}
pub trait ArgumentFields {}
impl ArgumentFields for TorrentGetField {}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum Args {
    FreeSpaceArgs(FreeSpaceArgs),
    TorrentGetArgs(TorrentGetArgs),
    TorrentActionArgs(TorrentActionArgs),
    TorrentRemoveArgs(TorrentRemoveArgs),
    TorrentAddArgs(TorrentAddArgs),
    TorrentSetLocationArgs(TorrentSetLocationArgs),
    TorrentRenamePathArgs(TorrentRenamePathArgs),
}

#[derive(Serialize, Debug, Clone)]
pub struct FreeSpaceArgs {
    path: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct TorrentGetArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    fields: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ids: Option<Vec<Id>>,
}

impl Default for TorrentGetArgs {
    fn default() -> Self {
        let all_fields = TorrentGetField::into_enum_iter()
            .map(|it| it.to_str())
            .collect();
        TorrentGetArgs {
            fields: Some(all_fields),
            ids: None,
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct TorrentActionArgs {
    ids: Vec<Id>,
}
#[derive(Serialize, Debug, Clone)]
pub struct TorrentRemoveArgs {
    ids: Vec<Id>,
    #[serde(rename = "delete-local-data")]
    delete_local_data: bool,
}

#[derive(Serialize, Debug, Clone)]
pub struct TorrentSetLocationArgs {
    ids: Vec<Id>,
    location: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "move")]
    move_from: Option<bool>,
}

#[derive(Serialize, Debug, Clone)]
pub struct TorrentRenamePathArgs {
    ids: Vec<Id>,
    path: String,
    name: String,
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum Id {
    Id(i64),
    Hash(String),
}

#[derive(Serialize, Debug, Clone)]
pub struct TorrentAddArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookies: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "download-dir")]
    pub download_dir: Option<String>,
    /// Either "filename" OR "metainfo" MUST be included
    /// semi-optional
    /// filename or URL of the .torrent file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    /// semi-optional
    /// base64-encoded .torrent content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metainfo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "peer-limit")]
    pub peer_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "bandwidthPriority")]
    pub bandwidth_priority: Option<i64>,
    /// list of indices of files to be downloaded
    /// to ignore some files, put their indices in files_unwanted, otherwise they will still be downloaded
    #[serde(skip_serializing_if = "Option::is_none", rename = "files-wanted")]
    pub files_wanted: Option<Vec<i32>>,
    /// list of indices of files not to download
    #[serde(skip_serializing_if = "Option::is_none", rename = "files-unwanted")]
    pub files_unwanted: Option<Vec<i32>>,
    /// list of indices of files to be downloaded with high priority
    #[serde(skip_serializing_if = "Option::is_none", rename = "priority-high")]
    pub priority_high: Option<Vec<i32>>,
    /// list of indices of files to be downloaded with low priority
    #[serde(skip_serializing_if = "Option::is_none", rename = "priority-low")]
    pub priority_low: Option<Vec<i32>>,
    /// list of indices of files to be downloaded with normal priority
    #[serde(skip_serializing_if = "Option::is_none", rename = "priority-normal")]
    pub priority_normal: Option<Vec<i32>>,
}

impl Default for TorrentAddArgs {
    fn default() -> Self {
        TorrentAddArgs {
            cookies: None,
            download_dir: None,
            filename: None,
            metainfo: None,
            paused: None,
            peer_limit: None,
            bandwidth_priority: None,
            files_wanted: None,
            files_unwanted: None,
            priority_high: None,
            priority_low: None,
            priority_normal: None,
        }
    }
}

#[derive(Clone, IntoEnumIterator)]
pub enum TorrentGetField {
    Activitydate,
    Addeddate,
    Availability,
    Bandwidthpriority,
    Comment,
    Corruptever,
    Creator,
    Datecreated,
    Desiredavailable,
    Donedate,
    Downloaddir,
    Downloadedever,
    Downloadlimit,
    Downloadlimited,
    Editdate,
    Error,
    Errorstring,
    Eta,
    Etaidle,
    Filecount,
    Files,
    Filestats,
    Group,
    HashString,
    Haveunchecked,
    Havevalid,
    Honorssessionlimits,
    Id,
    Isfinished,
    Isprivate,
    Isstalled,
    Labels,
    Leftuntildone,
    Magnetlink,
    Manualannouncetime,
    Maxconnectedpeers,
    Metadatapercentcomplete,
    Name,
    Peerlimit,
    Peers,
    Peersconnected,
    Peersfrom,
    Peersgettingfromus,
    Peerssendingtous,
    Percentcomplete,
    Percentdone,
    Pieces,
    Piececount,
    Piecesize,
    Priorities,
    Primarymimetype,
    Queueposition,
    Ratedownload,
    Rateupload,
    Recheckprogress,
    Secondsdownloading,
    Secondsseeding,
    Seedidlelimit,
    Seedidlemode,
    Seedratiolimit,
    Seedratiomode,
    Sizewhendone,
    Startdate,
    Status,
    Trackers,
    Trackerlist,
    Trackerstats,
    Totalsize,
    Torrentfile,
    Uploadedever,
    Uploadlimit,
    Uploadlimited,
    Uploadratio,
    Wanted,
    Webseeds,
    Webseedssendingtous,
}

impl TorrentGetField {
    pub fn all() -> Vec<TorrentGetField> {
        TorrentGetField::into_enum_iter().collect()
    }
}

impl TorrentGetField {
    pub fn to_str(&self) -> String {
        match self {
            TorrentGetField::Activitydate => "activityDate",
            TorrentGetField::Addeddate => "addedDate",
            TorrentGetField::Availability => "availability",
            TorrentGetField::Bandwidthpriority => "bandwidthPriority",
            TorrentGetField::Comment => "comment",
            TorrentGetField::Corruptever => "corruptEver",
            TorrentGetField::Creator => "creator",
            TorrentGetField::Datecreated => "dateCreated",
            TorrentGetField::Desiredavailable => "desiredAvailable",
            TorrentGetField::Donedate => "doneDate",
            TorrentGetField::Downloaddir => "downloadDir",
            TorrentGetField::Downloadedever => "downloadedEver",
            TorrentGetField::Downloadlimit => "downloadLimit",
            TorrentGetField::Downloadlimited => "downloadLimited",
            TorrentGetField::Editdate => "editDate",
            TorrentGetField::Error => "error",
            TorrentGetField::Errorstring => "errorString",
            TorrentGetField::Eta => "eta",
            TorrentGetField::Etaidle => "etaIdle",
            TorrentGetField::Filecount => "file-count",
            TorrentGetField::Files => "files",
            TorrentGetField::Filestats => "fileStats",
            TorrentGetField::Group => "group",
            TorrentGetField::HashString => "hashString",
            TorrentGetField::Haveunchecked => "haveUnchecked",
            TorrentGetField::Havevalid => "haveValid",
            TorrentGetField::Honorssessionlimits => "honorsSessionLimits",
            TorrentGetField::Id => "id",
            TorrentGetField::Isfinished => "isFinished",
            TorrentGetField::Isprivate => "isPrivate",
            TorrentGetField::Isstalled => "isStalled",
            TorrentGetField::Labels => "labels",
            TorrentGetField::Leftuntildone => "leftUntilDone",
            TorrentGetField::Magnetlink => "magnetLink",
            TorrentGetField::Manualannouncetime => "manualAnnounceTime",
            TorrentGetField::Maxconnectedpeers => "maxConnectedPeers",
            TorrentGetField::Metadatapercentcomplete => "metadataPercentComplete",
            TorrentGetField::Name => "name",
            TorrentGetField::Peerlimit => "peer-limit",
            TorrentGetField::Peers => "peers",
            TorrentGetField::Peersconnected => "peersConnected",
            TorrentGetField::Peersfrom => "peersFrom",
            TorrentGetField::Peersgettingfromus => "peersGettingFromUs",
            TorrentGetField::Peerssendingtous => "peersSendingToUs",
            TorrentGetField::Percentcomplete => "percentComplete",
            TorrentGetField::Percentdone => "percentDone",
            TorrentGetField::Pieces => "pieces",
            TorrentGetField::Piececount => "pieceCount",
            TorrentGetField::Piecesize => "pieceSize",
            TorrentGetField::Priorities => "priorities",
            TorrentGetField::Primarymimetype => "primaryMimeType",
            TorrentGetField::Queueposition => "queuePosition",
            TorrentGetField::Ratedownload => "rateDownload",
            TorrentGetField::Rateupload => "rateUpload",
            TorrentGetField::Recheckprogress => "recheckProgress",
            TorrentGetField::Secondsdownloading => "secondsDownloading",
            TorrentGetField::Secondsseeding => "secondsSeeding",
            TorrentGetField::Seedidlelimit => "seedIdleLimit",
            TorrentGetField::Seedidlemode => "seedIdleMode",
            TorrentGetField::Seedratiolimit => "seedRatioLimit",
            TorrentGetField::Seedratiomode => "seedRatioMode",
            TorrentGetField::Sizewhendone => "sizeWhenDone",
            TorrentGetField::Startdate => "startDate",
            TorrentGetField::Status => "status",
            TorrentGetField::Trackers => "trackers",
            TorrentGetField::Trackerlist => "trackerList",
            TorrentGetField::Trackerstats => "trackerStats",
            TorrentGetField::Totalsize => "totalSize",
            TorrentGetField::Torrentfile => "torrentFile",
            TorrentGetField::Uploadedever => "uploadedEver",
            TorrentGetField::Uploadlimit => "uploadLimit",
            TorrentGetField::Uploadlimited => "uploadLimited",
            TorrentGetField::Uploadratio => "uploadRatio",
            TorrentGetField::Wanted => "wanted",
            TorrentGetField::Webseeds => "webseeds",
            TorrentGetField::Webseedssendingtous => "webseedsSendingToUs",
        }
        .to_string()
    }
}

pub enum TorrentAction {
    Start,
    Stop,
    StartNow,
    Verify,
    Reannounce,
}

impl TorrentAction {
    pub fn to_str(&self) -> String {
        match self {
            TorrentAction::Start => "torrent-start",
            TorrentAction::Stop => "torrent-stop",
            TorrentAction::StartNow => "torrent-start-now",
            TorrentAction::Verify => "torrent-verify",
            TorrentAction::Reannounce => "torrent-reannounce",
        }
        .to_string()
    }
}
