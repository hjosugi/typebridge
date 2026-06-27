/// How a [`crate::command::Command`] reaches the backend.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Transport {
    /// Tauri `invoke("cmd", args)` from `@tauri-apps/api/core`.
    Tauri,
    /// A generic async `request<T>("cmd", args)` helper the consumer supplies — a
    /// seam for HTTP / WebSocket transports without committing typeship to one.
    Fetch,
}

impl Transport {
    /// The import line a [`crate::bridge::Bridge`] should emit for this transport,
    /// if any.
    pub(crate) fn import_line(self) -> Option<&'static str> {
        match self {
            Transport::Tauri => Some("import { invoke } from \"@tauri-apps/api/core\";"),
            // The consumer wires `request` to their own client; no fixed import.
            Transport::Fetch => None,
        }
    }
}
