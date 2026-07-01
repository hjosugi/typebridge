//! Where encode/decode symmetry breaks.
//!
//! Typeship is only sound if the wire shape the Rust side *writes* is the
//! wire shape the TypeScript side is told to *read* — `decode . encode = id`. A
//! handful of serde attributes quietly break that symmetry: they change the JSON
//! without changing the type's nominal structure, so a naive renderer emits TS
//! that does not match the bytes.
//!
//! This module is the shared catalogue of those hazards. Backends and lints point
//! at the same [`Hazard`] values so the warning text never drifts between the
//! generator and the docs. (In the MVP this is a reference catalogue; a future
//! attribute-aware backend can attach [`Diagnostic`]s to specific fields.)

/// A serde attribute (or shape) that can desynchronise the wire format from the
/// nominal Rust type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hazard {
    /// `#[serde(untagged)]` — variants are distinguished structurally at runtime,
    /// with no discriminant tag. The generated union loses its tag, so a consumer
    /// `switch` cannot be made exhaustive on a tag field.
    Untagged,
    /// `#[serde(flatten)]` — a nested struct's fields are spliced into the parent
    /// object. The naive nested-interface rendering does not match the flat JSON.
    Flatten,
    /// `#[serde(skip_serializing_if = "…")]` — the key may be absent on the wire.
    /// The TS field must be optional (`name?:`), not merely `T | null`.
    SkipSerializingIf,
    /// `#[serde(transparent)]` — the wrapper disappears on the wire; the TS type
    /// must be the inner type, not a one-field object.
    Transparent,
    /// `#[serde(default)]` — the key may be absent; decoding fills a default. The
    /// TS field should be optional to match the producer that omits it.
    Default,
    /// A `rename` / `rename_all` whose target collides with another field's wire
    /// name, silently dropping data.
    RenameCollision,
}

impl Hazard {
    /// A short, stable explanation suitable for a warning or a doc table.
    pub fn note(self) -> &'static str {
        match self {
            Hazard::Untagged => {
                "untagged enum: no discriminant on the wire; the union cannot be \
                 exhaustively switched on a tag — narrow structurally instead"
            }
            Hazard::Flatten => {
                "flattened field: child keys are spliced into the parent object; \
                 render the merged shape, not a nested interface"
            }
            Hazard::SkipSerializingIf => {
                "skip_serializing_if: the key may be absent; emit `name?: T`, not \
                 `name: T | null`"
            }
            Hazard::Transparent => {
                "transparent newtype: the wrapper vanishes on the wire; emit the \
                 inner type directly"
            }
            Hazard::Default => {
                "default field: the key may be absent and is filled on decode; emit \
                 `name?: T` so producers may omit it"
            }
            Hazard::RenameCollision => {
                "rename collision: two fields map to the same wire name; one will \
                 silently overwrite the other"
            }
        }
    }
}

/// A hazard attached to a concrete location in the source schema.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Diagnostic {
    /// What is risky.
    pub hazard: Hazard,
    /// Where it occurs, e.g. `"WorkspaceSnapshot.connections"`.
    pub location: String,
}

impl Diagnostic {
    /// Build a diagnostic for `hazard` at `location`.
    pub fn new(hazard: Hazard, location: impl Into<String>) -> Self {
        Diagnostic {
            hazard,
            location: location.into(),
        }
    }

    /// A one-line, human-readable rendering.
    pub fn render(&self) -> String {
        format!(
            "warning[{:?}] at {}: {}",
            self.hazard,
            self.location,
            self.hazard.note()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_hazard_has_a_note() {
        for hazard in [
            Hazard::Untagged,
            Hazard::Flatten,
            Hazard::SkipSerializingIf,
            Hazard::Transparent,
            Hazard::Default,
            Hazard::RenameCollision,
        ] {
            assert!(!hazard.note().is_empty());
        }
    }

    #[test]
    fn diagnostic_renders_location_and_note() {
        let diag = Diagnostic::new(Hazard::Flatten, "WorkspaceSnapshot.meta");
        let text = diag.render();
        assert!(text.contains("WorkspaceSnapshot.meta"), "{text}");
        assert!(text.contains("flattened field"), "{text}");
    }
}
