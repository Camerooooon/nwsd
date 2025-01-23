use notify_rust::{Notification, Urgency};
use std::fmt::Display;

use efcl::{color, Color};
use serde::Deserialize;

use crate::daemon::Config;

#[derive(Debug, Deserialize)]
pub struct GeoJson {
    features: Vec<Feature>,
}

#[derive(Debug, Deserialize)]
pub struct Feature {
    pub properties: AlertProperties,
}

#[derive(Debug, Deserialize)]
pub enum Severity {
    Extreme,
    Severe,
    Moderate,
    Minor,
    Unknown,
}

// Thanks GPT!
#[derive(Debug, Deserialize)]
pub enum Event {
    #[serde(rename = "Hazardous Weather Outlook")]
    HazardousWeatherOutlook,

    // Winter Weather/Cold Weather
    #[serde(rename = "Winter Storm Watch")]
    WinterStormWatch,
    #[serde(rename = "Blizzard Warning")]
    BlizzardWarning,
    #[serde(rename = "Winter Storm Warning")]
    WinterStormWarning,
    #[serde(rename = "Ice Storm Warning")]
    IceStormWarning,
    #[serde(rename = "Winter Weather Advisory")]
    WinterWeatherAdvisory,
    #[serde(rename = "Freeze Watch")]
    FreezeWatch,
    #[serde(rename = "Freeze Warning")]
    FreezeWarning,
    #[serde(rename = "Frost Advisory")]
    FrostAdvisory,
    #[serde(rename = "Cold Weather Advisory")]
    ColdWeatherAdvisory,
    #[serde(rename = "Extreme Cold Warning")]
    ExtremeColdWarning,

    // Fire Weather
    #[serde(rename = "Fire Weather Watch")]
    FireWeatherWatch,
    #[serde(rename = "Red Flag Warning")]
    RedFlagWarning,

    // Fog / Wind / Severe Weather
    #[serde(rename = "Dense Fog Advisory")]
    DenseFogAdvisory,
    #[serde(rename = "High Wind Watch")]
    HighWindWatch,
    #[serde(rename = "High Wind Warning")]
    HighWindWarning,
    #[serde(rename = "Wind Advisory")]
    WindAdvisory,
    #[serde(rename = "Severe Thunderstorm Watch")]
    SevereThunderstormWatch,
    #[serde(rename = "Severe Thunderstorm Warning")]
    SevereThunderstormWarning,
    #[serde(rename = "Tornado Watch")]
    TornadoWatch,
    #[serde(rename = "Tornado Warning")]
    TornadoWarning,
    #[serde(rename = "Extreme Wind Warning")]
    ExtremeWindWarning,

    // Marine
    #[serde(rename = "Small Craft Advisory")]
    SmallCraftAdvisory,
    #[serde(rename = "Gale Warning")]
    GaleWarning,
    #[serde(rename = "Storm Warning")]
    StormWarning,
    #[serde(rename = "Hurricane Force Wind Warning")]
    HurricaneForceWindWarning,
    #[serde(rename = "Special Marine Warning")]
    SpecialMarineWarning,

    // Flooding
    #[serde(rename = "Coastal Flood Watch")]
    CoastalFloodWatch,
    #[serde(rename = "Coastal Flood Warning")]
    CoastalFloodWarning,
    #[serde(rename = "Coastal Flood Advisory")]
    CoastalFloodAdvisory,
    #[serde(rename = "Flood Watch")]
    FloodWatch,
    #[serde(rename = "Flash Flood Warning")]
    FlashFloodWarning,
    #[serde(rename = "Flood Warning")]
    FloodWarning,
    #[serde(rename = "River Flood Watch")]
    RiverFloodWatch,
    #[serde(rename = "River Flood Warning")]
    RiverFloodWarning,

    // Excessive Heat
    #[serde(rename = "Excessive Heat Watch")]
    ExcessiveHeatWatch,
    #[serde(rename = "Excessive Heat Warning")]
    ExcessiveHeatWarning,
    #[serde(rename = "Heat Advisory")]
    HeatAdvisory,

    // Tropical
    #[serde(rename = "Tropical Storm Watch")]
    TropicalStormWatch,
    #[serde(rename = "Tropical Storm Warning")]
    TropicalStormWarning,
    #[serde(rename = "Hurricane Watch")]
    HurricaneWatch,
    #[serde(rename = "Hurricane Warning")]
    HurricaneWarning,

    // Fallback
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize)]
pub struct AlertProperties {
    pub headline: String,
    pub description: String,
    pub severity: Severity,
    pub id: String,
    pub event: Event,
}

pub fn extract_weather_features(json_data: String) -> Vec<Feature> {
    let geo_json: GeoJson = serde_json::from_str(&json_data).expect("Failed to deserialize JSON");
    geo_json.features
}

pub fn send_notification(feature: &Feature, config: &Config) {
    let icon_path = match &config.notification_icon_path {
        Some(a) => a,
        None => &format!(
            "/usr/share/icons/Papirus-Dark/symbolic/status/{}",
            get_icon_for_event(&feature.properties.event)
        ),
    };

    let body = match config.detailed_notification {
        true => &feature.properties.description,
        false => &feature.properties.headline,
    };

    let timeout = match feature.properties.severity {
        Severity::Extreme | Severity::Severe | Severity::Moderate | Severity::Unknown => 0, // Never timeout
        Severity::Minor => 120,
    };

    Notification::new()
        .summary(format!("{:?} Weather Alert", feature.properties.severity).as_str())
        .body(body)
        .icon(icon_path)
        .appname("National Weather Service Daemon")
        .urgency(get_notification_urgency_for_severity(
            &feature.properties.severity,
        ))
        .timeout(timeout)
        .show()
        .unwrap();
}

impl Display for Feature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let severity_color = match self.properties.severity {
            Severity::Extreme => Color::PURPLE,
            Severity::Severe => Color::RED,
            Severity::Moderate => Color::YELLOW,
            Severity::Minor => Color::GREEN,
            Severity::Unknown => Color::LIGHTGRAY,
        };

        write!(
            f,
            "{} {}",
            color!(
                severity_color,
                format!("{}:", self.properties.event).as_str()
            ),
            self.properties.headline.as_str()
        )
    }
}

pub fn get_notification_urgency_for_severity(severity: &Severity) -> Urgency {
    match severity {
        Severity::Extreme => Urgency::Critical,
        Severity::Severe => Urgency::Critical,
        Severity::Moderate => Urgency::Normal,
        Severity::Minor => Urgency::Low,
        Severity::Unknown => Urgency::Normal,
    }
}

pub fn get_icon_for_event(event: &Event) -> &'static str {
    match event {
        // Winter Weather / Cold Weather
        Event::WinterStormWatch
        | Event::WinterStormWarning
        | Event::BlizzardWarning
        | Event::IceStormWarning => "weather-snow-symbolic.svg",
        Event::WinterWeatherAdvisory
        | Event::FreezeWatch
        | Event::FreezeWarning
        | Event::FrostAdvisory => "weather-snow-symbolic.svg",
        Event::ExtremeColdWarning | Event::ColdWeatherAdvisory => "weather-snow-symbolic.svg",

        // Fire Weather
        Event::FireWeatherWatch | Event::RedFlagWarning => "weather-windy-symbolic.svg",

        // Fog / Wind / Severe Weather
        Event::DenseFogAdvisory => "weather-fog-symbolic.svg",
        Event::HighWindWatch | Event::HighWindWarning | Event::WindAdvisory => {
            "weather-windy-symbolic.svg"
        }
        Event::SevereThunderstormWatch | Event::SevereThunderstormWarning => {
            "weather-storm-symbolic.svg"
        }
        Event::TornadoWatch | Event::TornadoWarning | Event::ExtremeWindWarning => {
            "weather-tornado-symbolic.svg"
        }

        // Marine
        Event::SmallCraftAdvisory
        | Event::GaleWarning
        | Event::StormWarning
        | Event::HurricaneForceWindWarning => "weather-windy-symbolic.svg",
        Event::SpecialMarineWarning => "weather-storm-symbolic.svg",

        // Flooding
        Event::CoastalFloodWatch | Event::CoastalFloodWarning | Event::CoastalFloodAdvisory => {
            "weather-showers-symbolic.svg"
        }
        Event::FloodWatch
        | Event::FlashFloodWarning
        | Event::FloodWarning
        | Event::RiverFloodWatch
        | Event::RiverFloodWarning => "weather-showers-symbolic.svg",

        // Excessive Heat
        Event::ExcessiveHeatWatch | Event::ExcessiveHeatWarning | Event::HeatAdvisory => {
            "weather-clear-symbolic.svg"
        }

        // Tropical
        Event::TropicalStormWatch
        | Event::TropicalStormWarning
        | Event::HurricaneWatch
        | Event::HurricaneWarning => "weather-storm-symbolic.svg",

        // Fallback for unknown events
        Event::Unknown | Event::HazardousWeatherOutlook => "weather-severe-alert-symbolic.svg",
    }
}

// Thanks ChatGPT
impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let readable = match self {
            Event::HazardousWeatherOutlook => "Hazardous Weather Outlook",
            Event::WinterStormWatch => "Winter Storm Watch",
            Event::BlizzardWarning => "Blizzard Warning",
            Event::WinterStormWarning => "Winter Storm Warning",
            Event::IceStormWarning => "Ice Storm Warning",
            Event::WinterWeatherAdvisory => "Winter Weather Advisory",
            Event::FreezeWatch => "Freeze Watch",
            Event::FreezeWarning => "Freeze Warning",
            Event::FrostAdvisory => "Frost Advisory",
            Event::ColdWeatherAdvisory => "Cold Weather Advisory",
            Event::ExtremeColdWarning => "Extreme Cold Warning",
            Event::FireWeatherWatch => "Fire Weather Watch",
            Event::RedFlagWarning => "Red Flag Warning",
            Event::DenseFogAdvisory => "Dense Fog Advisory",
            Event::HighWindWatch => "High Wind Watch",
            Event::HighWindWarning => "High Wind Warning",
            Event::WindAdvisory => "Wind Advisory",
            Event::SevereThunderstormWatch => "Severe Thunderstorm Watch",
            Event::SevereThunderstormWarning => "Severe Thunderstorm Warning",
            Event::TornadoWatch => "Tornado Watch",
            Event::TornadoWarning => "Tornado Warning",
            Event::ExtremeWindWarning => "Extreme Wind Warning",
            Event::SmallCraftAdvisory => "Small Craft Advisory",
            Event::GaleWarning => "Gale Warning",
            Event::StormWarning => "Storm Warning",
            Event::HurricaneForceWindWarning => "Hurricane Force Wind Warning",
            Event::SpecialMarineWarning => "Special Marine Warning",
            Event::CoastalFloodWatch => "Coastal Flood Watch",
            Event::CoastalFloodWarning => "Coastal Flood Warning",
            Event::CoastalFloodAdvisory => "Coastal Flood Advisory",
            Event::FloodWatch => "Flood Watch",
            Event::FlashFloodWarning => "Flash Flood Warning",
            Event::FloodWarning => "Flood Warning",
            Event::RiverFloodWatch => "River Flood Watch",
            Event::RiverFloodWarning => "River Flood Warning",
            Event::ExcessiveHeatWatch => "Excessive Heat Watch",
            Event::ExcessiveHeatWarning => "Excessive Heat Warning",
            Event::HeatAdvisory => "Heat Advisory",
            Event::TropicalStormWatch => "Tropical Storm Watch",
            Event::TropicalStormWarning => "Tropical Storm Warning",
            Event::HurricaneWatch => "Hurricane Watch",
            Event::HurricaneWarning => "Hurricane Warning",
            Event::Unknown => "Unknown Event",
        };
        write!(f, "{}", readable)
    }
}
