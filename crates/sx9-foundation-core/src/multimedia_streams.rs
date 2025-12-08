//! Multimedia Streaming - Voice, VOIP, Video, Audio for CTAS Tactical Communications
//! Real-time multimedia streaming with tactical protocols and encryption

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rand::{thread_rng, Rng};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultimediaStreamEngine {
    pub active_streams: HashMap<String, MultimediaStream>,
    pub voice_channels: HashMap<String, VoiceChannel>,
    pub video_feeds: HashMap<String, VideoFeed>,
    pub audio_sources: HashMap<String, AudioSource>,
    #[serde(skip)]
    pub stream_broadcaster: broadcast::Sender<MultimediaEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultimediaStream {
    pub stream_id: String,
    pub stream_type: StreamType,
    pub source: StreamSource,
    pub destination: StreamDestination,
    pub codec: CodecInfo,
    pub quality: QualitySettings,
    pub encryption: EncryptionSettings,
    pub tactical_metadata: TacticalMetadata,
    pub bandwidth_usage: f64, // Mbps
    pub latency_ms: f64,
    pub jitter_ms: f64,
    pub packet_loss: f64,
    pub started_at: DateTime<Utc>,
    pub status: StreamStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StreamType {
    Voice {
        push_to_talk: bool,
        noise_suppression: bool,
        echo_cancellation: bool
    },
    Video {
        resolution: String,
        frame_rate: u32,
        infrared: bool,
        night_vision: bool
    },
    Audio {
        stereo: bool,
        directional: bool,
        frequency_range: (u32, u32)
    },
    VOIP {
        conference: bool,
        recording: bool,
        transcription: bool
    },
    DataStream {
        protocol: String,
        compressed: bool
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StreamSource {
    Agent { agent_id: String, persona_type: String },
    Human { user_id: String, role: String, clearance: String },
    Sensor { sensor_id: String, sensor_type: String, location: (f64, f64) },
    Drone { drone_id: String, altitude: f64, coordinates: (f64, f64) },
    Satellite { satellite_id: String, orbit_type: String },
    GroundStation { station_id: String, frequency_band: String },
    Synthetic { generator_id: String, scenario_type: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StreamDestination {
    TacticalChannel { channel_id: String, priority: u8 },
    CommandCenter { center_id: String },
    FieldUnit { unit_id: String, location: (f64, f64) },
    Agent { agent_id: String },
    RecordingSystem { system_id: String, classification: String },
    ExternalSystem { system_id: String, protocol: String },
    Broadcast { audience: String, encryption_level: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodecInfo {
    pub audio_codec: String, // "OPUS", "G.722", "MELP", "SILK"
    pub video_codec: String, // "H.264", "H.265", "AV1", "VP9"
    pub bitrate_kbps: u32,
    pub sample_rate: u32,
    pub channels: u8,
    pub compression_ratio: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualitySettings {
    pub resolution: String, // "4K", "1080p", "720p", "480p", "240p"
    pub frame_rate: u32,
    pub audio_quality: String, // "Studio", "High", "Standard", "Low", "Tactical"
    pub adaptive_bitrate: bool,
    pub error_correction: bool,
    pub redundancy_level: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionSettings {
    pub algorithm: String, // "AES-256", "ChaCha20", "SRTP", "ZRTP"
    pub key_exchange: String, // "ECDH", "RSA", "DH"
    pub authentication: String, // "HMAC-SHA256", "Poly1305"
    pub forward_secrecy: bool,
    pub classification_level: String, // "UNCLASSIFIED", "CONFIDENTIAL", "SECRET", "TOP_SECRET"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TacticalMetadata {
    pub call_sign: String,
    pub unit_designation: String,
    pub operation_code: String,
    pub grid_coordinates: Option<(f64, f64)>,
    pub frequency_allocation: String,
    pub priority_level: u8, // 1-5, 5 = FLASH OVERRIDE
    pub time_sensitivity: String, // "IMMEDIATE", "PRIORITY", "ROUTINE"
    pub distribution_list: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StreamStatus {
    Initializing,
    Active,
    Buffering,
    Degraded,
    Reconnecting,
    Terminated,
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceChannel {
    pub channel_id: String,
    pub channel_name: String,
    pub frequency: f64, // MHz
    pub participants: Vec<Participant>,
    pub push_to_talk: bool,
    pub radio_discipline: bool,
    pub encryption_enabled: bool,
    pub noise_floor: f64, // dB
    pub signal_strength: f64, // dBm
    pub voice_activation: bool,
    pub recording_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participant {
    pub participant_id: String,
    pub call_sign: String,
    pub role: String,
    pub is_transmitting: bool,
    pub signal_quality: f64,
    pub last_transmission: Option<DateTime<Utc>>,
    pub location: Option<(f64, f64)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoFeed {
    pub feed_id: String,
    pub feed_name: String,
    pub source_type: VideoSourceType,
    pub resolution: String,
    pub frame_rate: u32,
    pub field_of_view: f64, // degrees
    pub zoom_level: f64,
    pub infrared_enabled: bool,
    pub thermal_overlay: bool,
    pub target_tracking: bool,
    pub motion_detection: bool,
    pub recording_enabled: bool,
    pub stream_quality: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VideoSourceType {
    FixedCamera { location: (f64, f64), orientation: f64 },
    PTZCamera { pan: f64, tilt: f64, zoom: f64 },
    DroneCamera { altitude: f64, gimbal_stabilized: bool },
    HelmetCamera { wearer_id: String, head_tracking: bool },
    VehicleCamera { vehicle_id: String, camera_position: String },
    SatelliteImagery { satellite_id: String, resolution_cm: f64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioSource {
    pub source_id: String,
    pub source_name: String,
    pub audio_type: AudioType,
    pub frequency_range: (u32, u32), // Hz
    pub directional: bool,
    pub noise_suppression: bool,
    pub ambient_filtering: bool,
    pub voice_enhancement: bool,
    pub translation_enabled: bool,
    pub target_language: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AudioType {
    VoiceCommunication,
    EnvironmentalAudio,
    SignalIntelligence,
    MusicMorale,
    AlertTones,
    BackgroundNoise,
    WeaponSounds,
    VehicleNoise,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultimediaEvent {
    pub event_id: String,
    pub event_type: MultimediaEventType,
    pub stream_id: String,
    pub timestamp: DateTime<Utc>,
    pub data: serde_json::Value,
    pub quality_metrics: QualityMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MultimediaEventType {
    StreamStarted,
    StreamEnded,
    QualityChanged,
    ParticipantJoined,
    ParticipantLeft,
    TransmissionBegin,
    TransmissionEnd,
    ErrorOccurred,
    EncryptionKeyRotated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMetrics {
    pub bitrate_kbps: u32,
    pub packet_loss_percent: f64,
    pub jitter_ms: f64,
    pub latency_ms: f64,
    pub mos_score: f64, // Mean Opinion Score 1-5
    pub signal_to_noise_ratio: f64,
}

impl MultimediaStreamEngine {
    pub async fn new() -> Self {
        let (stream_broadcaster, _) = broadcast::channel(5000);

        Self {
            active_streams: HashMap::new(),
            voice_channels: HashMap::new(),
            video_feeds: HashMap::new(),
            audio_sources: HashMap::new(),
            stream_broadcaster,
        }
    }

    pub async fn create_voice_stream(&mut self, participants: Vec<String>, tactical_net: String) -> String {
        let stream_id = Uuid::new_v4().to_string();
        let channel_id = format!("VOICE-{}", Uuid::new_v4());

        let voice_channel = VoiceChannel {
            channel_id: channel_id.clone(),
            channel_name: format!("Tactical Net {}", tactical_net),
            frequency: thread_rng().gen_range(30.0..512.0), // VHF/UHF tactical frequencies
            participants: participants.iter().map(|p| {
                let roles = ["COMMANDER", "OPERATOR", "OBSERVER", "SPECIALIST"];
                let selected_role = roles[thread_rng().gen_range(0..4)];
                Participant {
                    participant_id: p.clone(),
                    call_sign: format!("CALL-{}", thread_rng().gen_range(100..999)),
                    role: selected_role.to_string(),
                    is_transmitting: false,
                    signal_quality: thread_rng().gen_range(0.7..1.0),
                    last_transmission: None,
                    location: Some((thread_rng().gen_range(-90.0..90.0), thread_rng().gen_range(-180.0..180.0))),
                }
            }).collect(),
            push_to_talk: true,
            radio_discipline: true,
            encryption_enabled: true,
            noise_floor: thread_rng().gen_range(-120.0..-80.0),
            signal_strength: thread_rng().gen_range(-60.0..-30.0),
            voice_activation: false,
            recording_enabled: true,
        };

        let multimedia_stream = MultimediaStream {
            stream_id: stream_id.clone(),
            stream_type: StreamType::Voice {
                push_to_talk: true,
                noise_suppression: true,
                echo_cancellation: true,
            },
            source: StreamSource::Human {
                user_id: participants[0].clone(),
                role: "TACTICAL_OPERATOR".to_string(),
                clearance: "SECRET".to_string(),
            },
            destination: StreamDestination::TacticalChannel {
                channel_id: channel_id.clone(),
                priority: 3,
            },
            codec: CodecInfo {
                audio_codec: "OPUS".to_string(),
                video_codec: "N/A".to_string(),
                bitrate_kbps: 32,
                sample_rate: 48000,
                channels: 1,
                compression_ratio: 0.8,
            },
            quality: QualitySettings {
                resolution: "N/A".to_string(),
                frame_rate: 0,
                audio_quality: "Tactical".to_string(),
                adaptive_bitrate: true,
                error_correction: true,
                redundancy_level: 2,
            },
            encryption: EncryptionSettings {
                algorithm: "AES-256".to_string(),
                key_exchange: "ECDH".to_string(),
                authentication: "HMAC-SHA256".to_string(),
                forward_secrecy: true,
                classification_level: "SECRET".to_string(),
            },
            tactical_metadata: TacticalMetadata {
                call_sign: format!("NET-{}", tactical_net),
                unit_designation: "CTAS-7".to_string(),
                operation_code: "OPERATION_STREAMING".to_string(),
                grid_coordinates: Some((thread_rng().gen_range(-90.0..90.0), thread_rng().gen_range(-180.0..180.0))),
                frequency_allocation: format!("{:.3} MHz", voice_channel.frequency),
                priority_level: 3,
                time_sensitivity: "PRIORITY".to_string(),
                distribution_list: participants.clone(),
            },
            bandwidth_usage: 0.032, // 32 kbps
            latency_ms: thread_rng().gen_range(20.0..150.0),
            jitter_ms: thread_rng().gen_range(1.0..20.0),
            packet_loss: thread_rng().gen_range(0.0..2.0),
            started_at: Utc::now(),
            status: StreamStatus::Active,
        };

        self.voice_channels.insert(channel_id, voice_channel);
        self.active_streams.insert(stream_id.clone(), multimedia_stream);

        // Start voice simulation
        self.start_voice_simulation(&stream_id).await;

        stream_id
    }

    pub async fn create_video_feed(&mut self, source_type: VideoSourceType, resolution: String) -> String {
        let feed_id = Uuid::new_v4().to_string();
        let stream_id = Uuid::new_v4().to_string();

        let video_feed = VideoFeed {
            feed_id: feed_id.clone(),
            feed_name: match &source_type {
                VideoSourceType::DroneCamera { .. } => "Drone Surveillance",
                VideoSourceType::FixedCamera { .. } => "Perimeter Camera",
                VideoSourceType::PTZCamera { .. } => "Tactical PTZ",
                VideoSourceType::HelmetCamera { .. } => "Operator POV",
                VideoSourceType::VehicleCamera { .. } => "Vehicle Mounted",
                VideoSourceType::SatelliteImagery { .. } => "Satellite Intel",
            }.to_string(),
            source_type: source_type.clone(),
            resolution: resolution.clone(),
            frame_rate: match resolution.as_str() {
                "4K" => 30,
                "1080p" => 60,
                "720p" => 60,
                _ => 30,
            },
            field_of_view: thread_rng().gen_range(45.0..120.0),
            zoom_level: 1.0,
            infrared_enabled: thread_rng().gen_bool(0.3),
            thermal_overlay: thread_rng().gen_bool(0.2),
            target_tracking: thread_rng().gen_bool(0.4),
            motion_detection: true,
            recording_enabled: true,
            stream_quality: thread_rng().gen_range(0.8..1.0),
        };

        let bandwidth = match resolution.as_str() {
            "4K" => 25.0,
            "1080p" => 8.0,
            "720p" => 4.0,
            "480p" => 2.0,
            _ => 1.0,
        };

        let multimedia_stream = MultimediaStream {
            stream_id: stream_id.clone(),
            stream_type: StreamType::Video {
                resolution: resolution.clone(),
                frame_rate: video_feed.frame_rate,
                infrared: video_feed.infrared_enabled,
                night_vision: thread_rng().gen_bool(0.3),
            },
            source: match source_type {
                VideoSourceType::DroneCamera { altitude, .. } => StreamSource::Drone {
                    drone_id: format!("DRONE-{}", thread_rng().gen_range(100..999)),
                    altitude,
                    coordinates: (thread_rng().gen_range(-90.0..90.0), thread_rng().gen_range(-180.0..180.0)),
                },
                _ => StreamSource::Sensor {
                    sensor_id: format!("CAM-{}", thread_rng().gen_range(100..999)),
                    sensor_type: "VIDEO_CAMERA".to_string(),
                    location: (thread_rng().gen_range(-90.0..90.0), thread_rng().gen_range(-180.0..180.0)),
                },
            },
            destination: StreamDestination::CommandCenter {
                center_id: "CTAS-COMMAND".to_string(),
            },
            codec: CodecInfo {
                audio_codec: "N/A".to_string(),
                video_codec: "H.265".to_string(),
                bitrate_kbps: (bandwidth * 1000.0) as u32,
                sample_rate: 0,
                channels: 0,
                compression_ratio: 0.6,
            },
            quality: QualitySettings {
                resolution,
                frame_rate: video_feed.frame_rate,
                audio_quality: "N/A".to_string(),
                adaptive_bitrate: true,
                error_correction: true,
                redundancy_level: 1,
            },
            encryption: EncryptionSettings {
                algorithm: "AES-256".to_string(),
                key_exchange: "ECDH".to_string(),
                authentication: "HMAC-SHA256".to_string(),
                forward_secrecy: true,
                classification_level: "CONFIDENTIAL".to_string(),
            },
            tactical_metadata: TacticalMetadata {
                call_sign: format!("VIDEO-{}", thread_rng().gen_range(100..999)),
                unit_designation: "SURVEILLANCE".to_string(),
                operation_code: "VIDEO_INTEL".to_string(),
                grid_coordinates: Some((thread_rng().gen_range(-90.0..90.0), thread_rng().gen_range(-180.0..180.0))),
                frequency_allocation: "DIGITAL".to_string(),
                priority_level: 2,
                time_sensitivity: "ROUTINE".to_string(),
                distribution_list: vec!["COMMAND".to_string(), "INTEL".to_string()],
            },
            bandwidth_usage: bandwidth,
            latency_ms: thread_rng().gen_range(50.0..300.0),
            jitter_ms: thread_rng().gen_range(5.0..50.0),
            packet_loss: thread_rng().gen_range(0.0..5.0),
            started_at: Utc::now(),
            status: StreamStatus::Active,
        };

        self.video_feeds.insert(feed_id, video_feed);
        self.active_streams.insert(stream_id.clone(), multimedia_stream);

        // Start video simulation
        self.start_video_simulation(&stream_id).await;

        stream_id
    }

    pub async fn create_audio_stream(&mut self, audio_type: AudioType, directional: bool) -> String {
        let source_id = Uuid::new_v4().to_string();
        let stream_id = Uuid::new_v4().to_string();

        let audio_source = AudioSource {
            source_id: source_id.clone(),
            source_name: match &audio_type {
                AudioType::VoiceCommunication => "Voice Comms",
                AudioType::EnvironmentalAudio => "Environmental",
                AudioType::SignalIntelligence => "SIGINT Audio",
                AudioType::MusicMorale => "Morale Music",
                AudioType::AlertTones => "Alert System",
                AudioType::BackgroundNoise => "Background",
                AudioType::WeaponSounds => "Weapon Audio",
                AudioType::VehicleNoise => "Vehicle Audio",
            }.to_string(),
            audio_type: audio_type.clone(),
            frequency_range: match audio_type {
                AudioType::VoiceCommunication => (300, 3400),
                AudioType::MusicMorale => (20, 20000),
                AudioType::SignalIntelligence => (100, 10000),
                _ => (100, 8000),
            },
            directional,
            noise_suppression: matches!(audio_type, AudioType::VoiceCommunication),
            ambient_filtering: true,
            voice_enhancement: matches!(audio_type, AudioType::VoiceCommunication | AudioType::SignalIntelligence),
            translation_enabled: matches!(audio_type, AudioType::VoiceCommunication | AudioType::SignalIntelligence),
            target_language: if matches!(audio_type, AudioType::VoiceCommunication) {
                Some("EN".to_string())
            } else {
                None
            },
        };

        let multimedia_stream = MultimediaStream {
            stream_id: stream_id.clone(),
            stream_type: StreamType::Audio {
                stereo: !directional,
                directional,
                frequency_range: audio_source.frequency_range,
            },
            source: StreamSource::Sensor {
                sensor_id: format!("AUDIO-{}", thread_rng().gen_range(100..999)),
                sensor_type: "AUDIO_SENSOR".to_string(),
                location: (thread_rng().gen_range(-90.0..90.0), thread_rng().gen_range(-180.0..180.0)),
            },
            destination: StreamDestination::CommandCenter {
                center_id: "CTAS-AUDIO".to_string(),
            },
            codec: CodecInfo {
                audio_codec: "OPUS".to_string(),
                video_codec: "N/A".to_string(),
                bitrate_kbps: match audio_type {
                    AudioType::MusicMorale => 320,
                    AudioType::VoiceCommunication => 64,
                    _ => 128,
                },
                sample_rate: match audio_type {
                    AudioType::MusicMorale => 48000,
                    _ => 16000,
                },
                channels: if directional { 1 } else { 2 },
                compression_ratio: 0.7,
            },
            quality: QualitySettings {
                resolution: "N/A".to_string(),
                frame_rate: 0,
                audio_quality: match audio_type {
                    AudioType::MusicMorale => "Studio".to_string(),
                    AudioType::VoiceCommunication => "High".to_string(),
                    _ => "Standard".to_string(),
                },
                adaptive_bitrate: true,
                error_correction: true,
                redundancy_level: 1,
            },
            encryption: EncryptionSettings {
                algorithm: "AES-256".to_string(),
                key_exchange: "ECDH".to_string(),
                authentication: "HMAC-SHA256".to_string(),
                forward_secrecy: true,
                classification_level: match audio_type {
                    AudioType::SignalIntelligence => "SECRET".to_string(),
                    _ => "CONFIDENTIAL".to_string(),
                },
            },
            tactical_metadata: TacticalMetadata {
                call_sign: format!("AUDIO-{}", thread_rng().gen_range(100..999)),
                unit_designation: "AUDIO_INTEL".to_string(),
                operation_code: "AUDIO_STREAM".to_string(),
                grid_coordinates: Some((thread_rng().gen_range(-90.0..90.0), thread_rng().gen_range(-180.0..180.0))),
                frequency_allocation: format!("{}-{} Hz", audio_source.frequency_range.0, audio_source.frequency_range.1),
                priority_level: match audio_type {
                    AudioType::AlertTones => 5,
                    AudioType::VoiceCommunication => 3,
                    _ => 2,
                },
                time_sensitivity: "ROUTINE".to_string(),
                distribution_list: vec!["AUDIO_TEAM".to_string()],
            },
            bandwidth_usage: (audio_source.codec.bitrate_kbps as f64) / 1000.0,
            latency_ms: thread_rng().gen_range(10.0..100.0),
            jitter_ms: thread_rng().gen_range(1.0..10.0),
            packet_loss: thread_rng().gen_range(0.0..1.0),
            started_at: Utc::now(),
            status: StreamStatus::Active,
        };

        self.audio_sources.insert(source_id, audio_source);
        self.active_streams.insert(stream_id.clone(), multimedia_stream);

        // Start audio simulation
        self.start_audio_simulation(&stream_id).await;

        stream_id
    }

    async fn start_voice_simulation(&self, stream_id: &str) {
        let stream_id_clone = stream_id.to_string();
        let broadcaster = self.stream_broadcaster.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(5));

            loop {
                interval.tick().await;

                let event = MultimediaEvent {
                    event_id: Uuid::new_v4().to_string(),
                    event_type: if thread_rng().gen_bool(0.3) {
                        MultimediaEventType::TransmissionBegin
                    } else {
                        MultimediaEventType::TransmissionEnd
                    },
                    stream_id: stream_id_clone.clone(),
                    timestamp: Utc::now(),
                    data: {
                        let message_types = ["STATUS", "REQUEST", "RESPONSE", "ALERT"];
                        let selected_type = message_types[thread_rng().gen_range(0..4)];
                        serde_json::json!({
                            "speaker": format!("OPERATOR-{}", thread_rng().gen_range(1..10)),
                            "duration_seconds": thread_rng().gen_range(2.0..30.0),
                            "message_type": selected_type,
                            "audio_level_db": thread_rng().gen_range(-40.0..-10.0)
                        })
                    },
                    quality_metrics: QualityMetrics {
                        bitrate_kbps: 32,
                        packet_loss_percent: thread_rng().gen_range(0.0..2.0),
                        jitter_ms: thread_rng().gen_range(1.0..20.0),
                        latency_ms: thread_rng().gen_range(20.0..150.0),
                        mos_score: thread_rng().gen_range(3.5..5.0),
                        signal_to_noise_ratio: thread_rng().gen_range(15.0..40.0),
                    },
                };

                let _ = broadcaster.send(event);
            }
        });
    }

    async fn start_video_simulation(&self, stream_id: &str) {
        let stream_id_clone = stream_id.to_string();
        let broadcaster = self.stream_broadcaster.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(10));

            loop {
                interval.tick().await;

                let event = MultimediaEvent {
                    event_id: Uuid::new_v4().to_string(),
                    event_type: MultimediaEventType::QualityChanged,
                    stream_id: stream_id_clone.clone(),
                    timestamp: Utc::now(),
                    data: {
                        let resolutions = ["4K", "1080p", "720p", "480p"];
                        let selected_resolution = resolutions[thread_rng().gen_range(0..4)];
                        serde_json::json!({
                            "frame_rate": thread_rng().gen_range(25..60),
                            "resolution_actual": selected_resolution,
                            "motion_detected": thread_rng().gen_bool(0.2),
                            "target_tracking": thread_rng().gen_bool(0.1),
                            "infrared_active": thread_rng().gen_bool(0.3),
                            "zoom_level": thread_rng().gen_range(1.0..10.0)
                        })
                    },
                    quality_metrics: QualityMetrics {
                        bitrate_kbps: thread_rng().gen_range(1000..25000),
                        packet_loss_percent: thread_rng().gen_range(0.0..5.0),
                        jitter_ms: thread_rng().gen_range(5.0..50.0),
                        latency_ms: thread_rng().gen_range(50.0..300.0),
                        mos_score: thread_rng().gen_range(3.0..5.0),
                        signal_to_noise_ratio: thread_rng().gen_range(20.0..50.0),
                    },
                };

                let _ = broadcaster.send(event);
            }
        });
    }

    async fn start_audio_simulation(&self, stream_id: &str) {
        let stream_id_clone = stream_id.to_string();
        let broadcaster = self.stream_broadcaster.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(3));

            loop {
                interval.tick().await;

                let event = MultimediaEvent {
                    event_id: Uuid::new_v4().to_string(),
                    event_type: MultimediaEventType::QualityChanged,
                    stream_id: stream_id_clone.clone(),
                    timestamp: Utc::now(),
                    data: {
                        let languages = ["EN", "RU", "AR", "ES", "FR"];
                        let detected_language = languages[thread_rng().gen_range(0..5)];
                        serde_json::json!({
                            "audio_level_db": thread_rng().gen_range(-60.0..0.0),
                            "frequency_peak": thread_rng().gen_range(100..8000),
                            "noise_floor_db": thread_rng().gen_range(-80.0..-40.0),
                            "voice_detected": thread_rng().gen_bool(0.4),
                            "language_detected": detected_language,
                            "transcription_confidence": thread_rng().gen_range(0.6..1.0)
                        })
                    },
                    quality_metrics: QualityMetrics {
                        bitrate_kbps: thread_rng().gen_range(32..320),
                        packet_loss_percent: thread_rng().gen_range(0.0..1.0),
                        jitter_ms: thread_rng().gen_range(1.0..10.0),
                        latency_ms: thread_rng().gen_range(10.0..100.0),
                        mos_score: thread_rng().gen_range(3.5..5.0),
                        signal_to_noise_ratio: thread_rng().gen_range(15.0..45.0),
                    },
                };

                let _ = broadcaster.send(event);
            }
        });
    }

    pub async fn get_all_streams(&self) -> Vec<MultimediaStream> {
        self.active_streams.values().cloned().collect()
    }

    pub async fn get_bandwidth_usage(&self) -> f64 {
        self.active_streams.values().map(|s| s.bandwidth_usage).sum()
    }

    pub fn subscribe_to_events(&self) -> broadcast::Receiver<MultimediaEvent> {
        self.stream_broadcaster.subscribe()
    }
}

// REST API endpoints
pub async fn create_voice_stream_endpoint(
    axum::Json(request): axum::Json<serde_json::Value>,
) -> axum::Json<serde_json::Value> {
    let mut engine = MultimediaStreamEngine::new().await;

    let participants: Vec<String> = request["participants"]
        .as_array()
        .unwrap_or(&Vec::new())
        .iter()
        .filter_map(|v| v.as_str().map(|s| s.to_string()))
        .collect();

    let tactical_net = request["tactical_net"].as_str().unwrap_or("ALPHA").to_string();

    let stream_id = engine.create_voice_stream(participants, tactical_net).await;

    axum::Json(serde_json::json!({
        "status": "created",
        "stream_id": stream_id,
        "stream_type": "voice",
        "endpoints": {
            "status": format!("/multimedia/stream/{}", stream_id),
            "quality": format!("/multimedia/stream/{}/quality", stream_id)
        }
    }))
}

pub async fn create_video_stream_endpoint(
    axum::Json(request): axum::Json<serde_json::Value>,
) -> axum::Json<serde_json::Value> {
    let mut engine = MultimediaStreamEngine::new().await;

    let source_type = VideoSourceType::DroneCamera {
        altitude: request["altitude"].as_f64().unwrap_or(100.0),
        gimbal_stabilized: true,
    };

    let resolution = request["resolution"].as_str().unwrap_or("1080p").to_string();

    let stream_id = engine.create_video_feed(source_type, resolution).await;

    axum::Json(serde_json::json!({
        "status": "created",
        "stream_id": stream_id,
        "stream_type": "video"
    }))
}

pub async fn create_audio_stream_endpoint(
    axum::Json(request): axum::Json<serde_json::Value>,
) -> axum::Json<serde_json::Value> {
    let mut engine = MultimediaStreamEngine::new().await;

    let audio_type = match request["audio_type"].as_str().unwrap_or("voice") {
        "voice" => AudioType::VoiceCommunication,
        "music" => AudioType::MusicMorale,
        "sigint" => AudioType::SignalIntelligence,
        "environmental" => AudioType::EnvironmentalAudio,
        _ => AudioType::VoiceCommunication,
    };

    let directional = request["directional"].as_bool().unwrap_or(false);

    let stream_id = engine.create_audio_stream(audio_type, directional).await;

    axum::Json(serde_json::json!({
        "status": "created",
        "stream_id": stream_id,
        "stream_type": "audio"
    }))
}

pub async fn get_multimedia_status() -> axum::Json<serde_json::Value> {
    let engine = MultimediaStreamEngine::new().await;
    let total_bandwidth = engine.get_bandwidth_usage().await;
    let active_streams = engine.get_all_streams().await;

    axum::Json(serde_json::json!({
        "total_streams": active_streams.len(),
        "total_bandwidth_mbps": total_bandwidth,
        "stream_types": {
            "voice": active_streams.iter().filter(|s| matches!(s.stream_type, StreamType::Voice { .. })).count(),
            "video": active_streams.iter().filter(|s| matches!(s.stream_type, StreamType::Video { .. })).count(),
            "audio": active_streams.iter().filter(|s| matches!(s.stream_type, StreamType::Audio { .. })).count(),
            "voip": active_streams.iter().filter(|s| matches!(s.stream_type, StreamType::VOIP { .. })).count(),
        },
        "quality_summary": {
            "average_latency": active_streams.iter().map(|s| s.latency_ms).sum::<f64>() / active_streams.len() as f64,
            "average_packet_loss": active_streams.iter().map(|s| s.packet_loss).sum::<f64>() / active_streams.len() as f64,
        }
    }))
}

pub async fn start_multimedia_stream() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "status": "starting",
        "message": "Multimedia stream engine initializing",
        "endpoints": {
            "voice": "/multimedia/voice/create",
            "video": "/multimedia/video/create",
            "audio": "/multimedia/audio/create",
            "status": "/multimedia/status"
        }
    }))
}