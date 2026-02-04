use crate::pane::Pane;
use crate::renderable::StableCursorPosition;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine as _;
use crossbeam::channel::{bounded, Receiver, Sender, TrySendError};
use serde::Serialize;
use std::convert::TryFrom;
use std::io::Write;
use std::sync::{Arc, OnceLock};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
use wezterm_uds::UnixStream;

const EVENT_QUEUE_CAPACITY: usize = 1024;
const MAX_OUTPUT_BYTES: usize = 64 * 1024;

/// Trait for receiving native events from WezTerm.
///
/// Implementations must be non-blocking and thread-safe.
pub trait WaEventSink: Send + Sync + 'static {
    /// Called when new output is received for a pane.
    fn on_pane_output(&self, pane_id: u64, data: &[u8]);

    /// Called when pane state changes (title, dimensions, alt-screen, cursor).
    fn on_pane_state_change(&self, pane_id: u64, state: &WaPaneState);

    /// Called when a user-var (OSC 1337) is set.
    fn on_user_var_changed(&self, pane_id: u64, name: &str, value: &str);

    /// Called when a new pane is created.
    fn on_pane_created(&self, pane_id: u64, domain: &str, cwd: Option<&str>);

    /// Called when a pane is destroyed.
    fn on_pane_destroyed(&self, pane_id: u64);
}

/// Snapshot of pane state for state change events.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WaPaneState {
    pub title: String,
    pub rows: u16,
    pub cols: u16,
    pub is_alt_screen: bool,
    pub cursor_row: u32,
    pub cursor_col: u32,
}

/// Build a pane state snapshot from a Pane.
#[must_use]
pub fn pane_state_from_pane(pane: &dyn Pane) -> WaPaneState {
    let dimensions = pane.get_dimensions();
    let cursor = pane.get_cursor_position();

    WaPaneState {
        title: pane.get_title(),
        rows: clamp_u16(dimensions.viewport_rows),
        cols: clamp_u16(dimensions.cols),
        is_alt_screen: pane.is_alt_screen_active(),
        cursor_row: clamp_cursor_row(cursor),
        cursor_col: clamp_cursor_col(cursor),
    }
}

fn clamp_u16(value: usize) -> u16 {
    u16::try_from(value).unwrap_or(u16::MAX)
}

fn clamp_cursor_row(cursor: StableCursorPosition) -> u32 {
    u32::try_from(cursor.y).unwrap_or(0)
}

fn clamp_cursor_col(cursor: StableCursorPosition) -> u32 {
    u32::try_from(cursor.x).unwrap_or(0)
}

static WA_EVENT_SINK: OnceLock<Arc<dyn WaEventSink>> = OnceLock::new();

/// Register the global wa event sink.
pub fn register_wa_event_sink(sink: Arc<dyn WaEventSink>) {
    let _ = WA_EVENT_SINK.set(sink);
}

/// Emit a wa event if a sink is registered.
pub(crate) fn emit_wa_event(event: impl FnOnce(&dyn WaEventSink)) {
    if let Some(sink) = WA_EVENT_SINK.get() {
        event(sink.as_ref());
    }
}

/// Initialize the wa integration from the WEZTERM_WA_SOCKET environment variable.
pub fn init_wa_integration_from_env() {
    if let Ok(socket_path) = std::env::var("WEZTERM_WA_SOCKET") {
        match WaSocketSink::connect(&socket_path) {
            Ok(sink) => register_wa_event_sink(sink),
            Err(err) => log::warn!("Failed to connect to wa socket {}: {:#}", socket_path, err),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum WaEvent {
    Hello {
        proto: u32,
        wezterm_version: String,
        ts: u64,
    },
    PaneOutput {
        pane_id: u64,
        data_b64: String,
        ts: u64,
    },
    StateChange {
        pane_id: u64,
        state: WaPaneState,
        ts: u64,
    },
    UserVar {
        pane_id: u64,
        name: String,
        value: String,
        ts: u64,
    },
    PaneCreated {
        pane_id: u64,
        domain: String,
        cwd: Option<String>,
        ts: u64,
    },
    PaneDestroyed {
        pane_id: u64,
        ts: u64,
    },
}

/// Socket-backed event sink that emits newline-delimited JSON.
pub struct WaSocketSink {
    tx: Sender<WaEvent>,
}

impl WaSocketSink {
    pub fn connect(path: &str) -> anyhow::Result<Arc<dyn WaEventSink>> {
        let stream = UnixStream::connect(path)?;
        let (tx, rx) = bounded(EVENT_QUEUE_CAPACITY);

        thread::Builder::new()
            .name("wa-event-sink".to_string())
            .spawn(move || writer_loop(stream, rx))
            .expect("spawn wa event sink thread");

        // Best-effort hello.
        let _ = tx.try_send(WaEvent::Hello {
            proto: 1,
            wezterm_version: config::wezterm_version().to_string(),
            ts: now_ms(),
        });

        Ok(Arc::new(Self { tx }))
    }

    fn enqueue(&self, event: WaEvent) {
        match self.tx.try_send(event) {
            Ok(()) => {}
            Err(TrySendError::Full(_)) => {
                // Drop when saturated.
            }
            Err(TrySendError::Disconnected(_)) => {
                // Writer thread exited.
            }
        }
    }
}

impl WaEventSink for WaSocketSink {
    fn on_pane_output(&self, pane_id: u64, data: &[u8]) {
        let bounded = if data.len() > MAX_OUTPUT_BYTES {
            &data[..MAX_OUTPUT_BYTES]
        } else {
            data
        };
        let data_b64 = BASE64_STANDARD.encode(bounded);
        self.enqueue(WaEvent::PaneOutput {
            pane_id,
            data_b64,
            ts: now_ms(),
        });
    }

    fn on_pane_state_change(&self, pane_id: u64, state: &WaPaneState) {
        self.enqueue(WaEvent::StateChange {
            pane_id,
            state: state.clone(),
            ts: now_ms(),
        });
    }

    fn on_user_var_changed(&self, pane_id: u64, name: &str, value: &str) {
        self.enqueue(WaEvent::UserVar {
            pane_id,
            name: name.to_string(),
            value: value.to_string(),
            ts: now_ms(),
        });
    }

    fn on_pane_created(&self, pane_id: u64, domain: &str, cwd: Option<&str>) {
        self.enqueue(WaEvent::PaneCreated {
            pane_id,
            domain: domain.to_string(),
            cwd: cwd.map(str::to_string),
            ts: now_ms(),
        });
    }

    fn on_pane_destroyed(&self, pane_id: u64) {
        self.enqueue(WaEvent::PaneDestroyed {
            pane_id,
            ts: now_ms(),
        });
    }
}

fn writer_loop(mut stream: UnixStream, rx: Receiver<WaEvent>) {
    for event in rx.iter() {
        match serde_json::to_string(&event) {
            Ok(line) => {
                if stream.write_all(line.as_bytes()).is_err() {
                    break;
                }
                if stream.write_all(b"\n").is_err() {
                    break;
                }
            }
            Err(err) => {
                log::warn!("Failed to serialize wa event: {:#}", err);
            }
        }
    }
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}
