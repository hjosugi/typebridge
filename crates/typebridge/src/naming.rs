//! Naming as an isomorphism.
//!
//! Rust fields are idiomatic `snake_case`; JSON and TypeScript are idiomatic
//! `lowerCamelCase`. Treating the conversion as a (partial) isomorphism — rather
//! than two independent string manglers — is what keeps encode and decode in sync.
//!
//! The iso holds on the subset of *well-formed identifiers*: a `snake_case`
//! identifier with no leading/trailing/double underscores round-trips exactly. Type
//! names (`PascalCase`) are intentionally **not** transformed; they pass through the
//! renderers unchanged.

/// `snake_case` (or `snake_case_id`) to `lowerCamelCase`.
///
/// ```
/// use typebridge::naming::to_camel_case;
/// assert_eq!(to_camel_case("active_connection_id"), "activeConnectionId");
/// assert_eq!(to_camel_case("latency_ms"), "latencyMs");
/// assert_eq!(to_camel_case("id"), "id");
/// ```
pub fn to_camel_case(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut upper_next = false;
    let mut seen_first = false;
    for ch in input.chars() {
        if ch == '_' {
            // Leading underscores are dropped; interior ones uppercase the next char.
            if seen_first {
                upper_next = true;
            }
            continue;
        }
        if upper_next {
            out.extend(ch.to_uppercase());
            upper_next = false;
        } else {
            out.push(ch);
        }
        seen_first = true;
    }
    out
}

/// `lowerCamelCase` to `snake_case`.
///
/// ```
/// use typebridge::naming::to_snake_case;
/// assert_eq!(to_snake_case("activeConnectionId"), "active_connection_id");
/// assert_eq!(to_snake_case("latencyMs"), "latency_ms");
/// ```
pub fn to_snake_case(input: &str) -> String {
    let mut out = String::with_capacity(input.len() + 4);
    for (i, ch) in input.chars().enumerate() {
        if ch.is_uppercase() {
            if i != 0 {
                out.push('_');
            }
            out.extend(ch.to_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camel_matches_irodori_fields() {
        assert_eq!(to_camel_case("latency_ms"), "latencyMs");
        assert_eq!(to_camel_case("active_connection_id"), "activeConnectionId");
        assert_eq!(to_camel_case("workspace_snapshot"), "workspaceSnapshot");
    }

    #[test]
    fn snake_round_trips_with_camel() {
        for id in ["id", "latency_ms", "active_connection_id", "rows"] {
            assert_eq!(to_snake_case(&to_camel_case(id)), id, "iso failed for {id}");
        }
    }

    #[test]
    fn degenerate_inputs_do_not_panic() {
        assert_eq!(to_camel_case(""), "");
        assert_eq!(to_camel_case("_leading"), "leading");
        assert_eq!(to_camel_case("trailing_"), "trailing");
        assert_eq!(to_camel_case("double__under"), "doubleUnder");
    }
}
