//! Provides an interface for building activities to send
//! to Discord via [`DiscordIpc::set_activity`](crate::DiscordIpc::set_activity).
use serde_derive::Serialize;
use serde_repr::{Serialize_repr, Deserialize_repr};

/// A struct representing a Discord rich presence activity
///
/// Note that all methods return `Self`, and can be chained
/// for fluency
#[derive(Serialize, Clone)]
pub struct Activity<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_url: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub details_url: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamps: Option<Timestamps>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub party: Option<Party<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<Assets<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Secrets<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub buttons: Option<Vec<Button<'a>>>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub activity_type: Option<ActivityType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_display_type: Option<StatusDisplayType>,
}

/// A struct representing an `Activity`'s timestamps
///
/// Note that all methods return `Self`, and can be chained
/// for fluency
#[derive(Serialize, Clone)]
pub struct Timestamps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<i64>,
}

/// A struct representing an `Activity`'s game party
///
/// Note that all methods return `Self`, and can be chained
/// for fluency
#[derive(Serialize, Clone)]
pub struct Party<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<[i32; 2]>,
}

/// A struct representing the art assets and hover text
/// used by an `Activity`
///
/// Note that all methods return `Self`, and can be chained
/// for fluency
#[derive(Serialize, Clone)]
pub struct Assets<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_image: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_text: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_image: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_text: Option<&'a str>,
}

/// A struct representing the secrets used by an
/// `Activity`
///
/// Note that all methods return `Self`, and can be chained
/// for fluency
#[derive(Serialize, Clone)]
pub struct Secrets<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub spectate: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#match: Option<&'a str>,
}

/// A struct representing the buttons that are
/// attached to an `Activity`
///
/// An activity may have a maximum of 2 buttons
#[derive(Serialize, Clone)]
pub struct Button<'a> {
    pub label: &'a str,
    pub url: &'a str,
}

/// A struct to set the Activity Type of the `Activity`
#[derive(Serialize_repr,Deserialize_repr, Clone, Debug, Hash, Eq, PartialEq)]
#[repr(u8)]
pub enum ActivityType {
    /// Activity type "Playing X"
    Playing = 0,
    /// Activity type "Listening to X"
    Listening = 2,
    /// Activity type "Watching X"
    Watching = 3,
    /// Activity type "Competing in X"
    Competing = 5,
}

impl std::fmt::Display for ActivityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            Self::Playing => "Playing",
            Self::Listening => "Listening",
            Self::Watching => "Watching",
            Self::Competing => "Competing",
        };

        write!(f, "{}", out)
    }
}

/// A struct to set the Status Display Type of the `Activity`
/// Controls which field is displayed in the user's status text in the member list
#[derive(Serialize_repr, Clone)]
#[repr(u8)]
pub enum StatusDisplayType {
    /// "Listening to Spotify"
    Name = 0,
    /// "Listening to Rick Astley"
    State = 1,
    /// "Listening to Never Gonna Give You Up"
    Details = 2
}

impl<'a> Activity<'a> {
    /// Creates a new `Activity`
    pub fn new() -> Self {
        Activity {
            state: None,
            state_url: None,
            details: None,
            details_url: None,
            assets: None,
            buttons: None,
            party: None,
            secrets: None,
            timestamps: None,
            activity_type: None,
            status_display_type: None,
        }
    }
}

impl<'a> Default for Activity<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl Timestamps {
    /// Creates a new `Timestamps`
    pub fn new() -> Self {
        Timestamps {
            start: None,
            end: None,
        }
    }
}

impl Default for Timestamps {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Party<'a> {
    /// Creates a new `Party`
    pub fn new() -> Self {
        Party {
            id: None,
            size: None,
        }
    }
}

impl<'a> Default for Party<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Assets<'a> {
    /// Creates a new `Assets`
    pub fn new() -> Self {
        Assets {
            large_image: None,
            large_text: None,
            small_image: None,
            small_text: None,
        }
    }
}

impl<'a> Default for Assets<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Secrets<'a> {
    /// Creates a new `Secrets`
    pub fn new() -> Self {
        Secrets {
            join: None,
            spectate: None,
            r#match: None,
        }
    }
}

impl<'a> Default for Secrets<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Button<'a> {
    /// Creates a new `Button` with the given label and
    /// URL
    ///
    /// The label must be 1-32 characters long
    ///
    /// The URL must be 1-512 characters long
    pub fn new(label: &'a str, url: &'a str) -> Self {
        Button { label, url }
    }
}
