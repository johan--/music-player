#[cfg(test)]
mod tests;
use crate::simple_broker::SimpleBroker;
use anyhow::Error;
use async_graphql::Schema;
use futures_util::StreamExt;
use music_player_addons::Player;
use music_player_discovery::{discover, SERVICE_NAME, XBMC_SERVICE_NAME};
use music_player_entity::track as track_entity;
use music_player_playback::player::PlayerCommand;
use music_player_types::types::RemoteCoverUrl;
use music_player_types::types::RemoteTrackUrl;
use music_player_types::types::{Device, AIRPLAY_SERVICE_NAME, CHROMECAST_SERVICE_NAME};
use rand::seq::SliceRandom;
use schema::{Mutation, Query, Subscription};
use std::{
    sync::{Arc, Mutex},
    thread,
};
use tokio::sync::mpsc::UnboundedSender;
use upnp_client::discovery::discover_pnp_locations;
use url::Url;

pub mod schema;
pub mod simple_broker;

pub type MusicPlayerSchema = Schema<Query, Mutation, Subscription>;

const MEDIA_RENDERER: &str = "urn:schemas-upnp-org:device:MediaRenderer";
const MEDIA_SERVER: &str = "urn:schemas-upnp-org:device:MediaServer";

fn scan_mp_devices(mp_devices: Arc<Mutex<Vec<Device>>>) {
    thread::spawn(move || {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let services = discover(SERVICE_NAME);
            tokio::pin!(services);
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            while let Some(info) = services.next().await {
                let device = Device::from(info.clone());
                let mut mp_devices = mp_devices.lock().unwrap();
                if mp_devices
                    .iter()
                    .find(|d| d.id == device.id && d.service == device.service)
                    .is_none()
                {
                    mp_devices.push(device.clone());
                    SimpleBroker::<Device>::publish(device.clone());
                }
            }
        });
    });
}

fn scan_xbmc_devices(xbmc_devices: Arc<Mutex<Vec<Device>>>) {
    thread::spawn(move || {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let services = discover(XBMC_SERVICE_NAME);
            tokio::pin!(services);
            while let Some(info) = services.next().await {
                xbmc_devices
                    .lock()
                    .unwrap()
                    .push(Device::from(info.clone()));
                SimpleBroker::<Device>::publish(Device::from(info.clone()));
            }
        });
    });
}

fn scan_chromecast_devices(devices: Arc<Mutex<Vec<Device>>>) {
    thread::spawn(move || {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let services = discover(CHROMECAST_SERVICE_NAME);
            tokio::pin!(services);
            while let Some(info) = services.next().await {
                devices.lock().unwrap().push(Device::from(info.clone()));
                SimpleBroker::<Device>::publish(Device::from(info.clone()));
            }
        });
    });
}

fn scan_upnp_dlna_devices(devices: Arc<Mutex<Vec<Device>>>) {
    thread::spawn(move || {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            if let Ok(upnp_devices) = discover_pnp_locations().await {
                tokio::pin!(upnp_devices);

                while let Some(device) = upnp_devices.next().await {
                    if device.device_type.contains(MEDIA_RENDERER)
                        || device.device_type.contains(MEDIA_SERVER)
                    {
                        let mut devices = devices.lock().unwrap();
                        if devices.iter().find(|d| d.id == device.udn).is_none() {
                            devices.push(Device::from(device.clone()));
                            SimpleBroker::<Device>::publish(Device::from(device.clone()));
                        }
                    }
                }
            }
        });
    });
}

pub async fn scan_devices() -> Result<Arc<std::sync::Mutex<Vec<Device>>>, Box<dyn std::error::Error>>
{
    let devices: Arc<std::sync::Mutex<Vec<Device>>> = Arc::new(std::sync::Mutex::new(Vec::new()));
    let mp_devices = Arc::clone(&devices);
    let xbmc_devices = Arc::clone(&devices);
    let chromecast_devices = Arc::clone(&devices);
    let dlna_devices = Arc::clone(&devices);

    scan_mp_devices(mp_devices);

    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    scan_xbmc_devices(xbmc_devices);

    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    scan_chromecast_devices(chromecast_devices);

    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    scan_upnp_dlna_devices(dlna_devices);

    Ok(devices)
}

pub async fn load_tracks(
    player_cmd: &Arc<Mutex<UnboundedSender<PlayerCommand>>>,
    player: Option<&mut Box<dyn Player + Send>>,
    source_ip: Option<String>,
    mut tracks: Vec<track_entity::Model>,
    position: Option<u32>,
    shuffle: bool,
) -> Result<(), Error> {
    if shuffle {
        tracks.shuffle(&mut rand::thread_rng());
    }
    if let Some(player) = player {
        if player.device_type() == "chromecast" {
            if let Some(source_ip) = source_ip {
                tracks = tracks
                    .into_iter()
                    .map(|mut track| {
                        let url = Url::parse(track.uri.as_str()).unwrap();
                        let host = url.host_str().unwrap();
                        track.uri = track.uri.to_lowercase().replace(host, source_ip.as_str());
                        let cover = match track.clone().album.cover {
                            Some(cover) => Url::parse(cover.as_str()).ok().map(|url| {
                                let host = url.host_str().unwrap();
                                cover.to_lowercase().replace(host, source_ip.as_str())
                            }),
                            None => None,
                        };
                        track.album.cover = cover;
                        track
                    })
                    .collect();
            }
        }
        player
            .load_tracks(
                tracks.clone().into_iter().map(Into::into).collect(),
                Some(0),
            )
            .await?;
        return Ok(());
    }
    let player_cmd_tx = player_cmd.lock().unwrap();
    player_cmd_tx.send(PlayerCommand::Stop).unwrap();
    player_cmd_tx.send(PlayerCommand::Clear).unwrap();
    player_cmd_tx
        .send(PlayerCommand::LoadTracklist { tracks })
        .unwrap();
    player_cmd_tx
        .send(PlayerCommand::PlayTrackAt(position.unwrap_or(0) as usize))
        .unwrap();
    Ok(())
}

pub fn update_tracks_url<T: RemoteCoverUrl + RemoteTrackUrl>(
    devices: Vec<Device>,
    result: T,
    will_play_on_chromecast: bool,
) -> Result<T, Error> {
    let base_url = match devices
        .clone()
        .into_iter()
        .find(|device| device.is_current_device)
    {
        Some(device) => {
            let host = match will_play_on_chromecast {
                true => device.ip,
                false => device.host,
            };
            Some(format!("http://{}:{}", host, device.port))
        }
        None => None,
    };

    match base_url.is_some() {
        true => {
            let base_url = base_url.unwrap();
            Ok(result
                .with_remote_cover_url(&base_url)
                .with_remote_track_url(&base_url))
        }
        false => Err(Error::msg("Cannot find current device")),
    }
}

pub fn update_track_url<T: RemoteTrackUrl>(
    devices: Vec<Device>,
    result: T,
    will_play_on_chromecast: bool,
) -> Result<T, Error> {
    let base_url = match devices
        .clone()
        .into_iter()
        .find(|device| device.is_current_device)
    {
        Some(device) => {
            let host = match will_play_on_chromecast {
                true => device.ip,
                false => device.host,
            };
            Some(format!("http://{}:{}", host, device.port))
        }
        None => None,
    };

    match base_url.is_some() {
        true => {
            let base_url = base_url.unwrap();
            Ok(result.with_remote_track_url(&base_url))
        }
        false => Err(Error::msg("Cannot find current device")),
    }
}

pub fn update_cover_url<T: RemoteCoverUrl>(
    devices: Vec<Device>,
    result: T,
    will_play_on_chromecast: bool,
) -> Result<T, Error> {
    let base_url = match devices
        .clone()
        .into_iter()
        .find(|device| device.is_current_device)
    {
        Some(device) => {
            let host = match will_play_on_chromecast {
                true => device.ip,
                false => device.host,
            };
            Some(format!("http://{}:{}", host, device.port))
        }
        None => None,
    };

    match base_url.is_some() {
        true => {
            let base_url = base_url.unwrap();
            Ok(result.with_remote_cover_url(&base_url))
        }
        false => Err(Error::msg("Cannot find current device")),
    }
}
