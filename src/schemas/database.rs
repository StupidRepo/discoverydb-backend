use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use mongodb::sync::{Client as MongoClient, Collection};

use super::{Bot, Client, Plugin, Server, Theme};

#[derive(Serialize, Deserialize, Debug)]
pub enum ThemeClient {
    Android,
    Revite,
}

#[allow(non_camel_case_types)] // Piss off :3
#[derive(Serialize, Deserialize, Debug)]
pub enum ClientPlatform {
    Android,
    iOS,

    Web,
    Desktop,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ReportStatus {
    /// The report has been submitted, but not yet reviewed.
    Pending,
    /// The report is being reviewed by a moderator.
    Reviewing,

    /// The report has been reviewed and no action was taken.
    Ignored,
    /// The report has been reviewed and action has been taken.
    ActionDealt {
        /// The action that was taken
        action: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ReportType {
    DiscoveryDBUser,

    Bot,
    Server,

    Theme,
    Plugin,
    Client,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Report {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    pub reporter_id: String,

    pub reported_id: String,
    pub reported_type: ReportType,

    pub reason: String,
    pub status: ReportStatus,
}

pub struct Database {
    // ? Common things a user wants to see
    pub bot: Collection<Bot>,
    pub server: Collection<Server>,

    // ? Other shit a user may want
    // TODO: Figure out conversion for Revite -> Android and Android -> Revite themes.
    pub theme: Collection<Theme>,
    /// Custom Revolt clients.
    pub client: Collection<Client>,
    /// Plugins that modify supported clients.
    pub plugin: Collection<Plugin>,

    // ? Moderation stuff
    /// Reports for abusive content.
    pub report: Collection<Report>,
}

impl Database {
    pub fn init() -> Self {
        let client =
            MongoClient::with_uri_str(std::env::var("MONGO_URL").unwrap())
                .unwrap();
        let db = client.database("DiscoveryDB");

        Database {
            bot: db.collection("bot"),
            client: db.collection("client"),
            plugin: db.collection("plugin"),
            report: db.collection("report"),
            server: db.collection("server"),
            theme: db.collection("theme"),
        }
    }
}