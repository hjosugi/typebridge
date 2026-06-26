use std::process::ExitCode;

use ts_rs::TS;
use typeship::ir::TsType;
use typeship::{Arg, Bridge, Command};
use typeship_ts_rs::decl;

#[derive(TS)]
#[ts(rename_all = "lowercase")]
#[allow(dead_code)]
enum NoteKind {
    Text,
    Sql,
}

#[derive(TS)]
#[ts(rename_all = "lowercase")]
#[allow(dead_code)]
enum NoteStatus {
    Draft,
    Published,
    Archived,
}

#[derive(TS)]
#[ts(rename_all = "camelCase")]
#[allow(dead_code)]
struct Note {
    id: String,
    title: String,
    kind: NoteKind,
    status: NoteStatus,
    body: String,
    tags: Vec<String>,
    #[ts(optional)]
    source_url: Option<String>,
}

#[derive(TS)]
#[ts(rename_all = "camelCase")]
#[allow(dead_code)]
struct CreateNote {
    title: String,
    kind: NoteKind,
    body: String,
    tags: Vec<String>,
}

#[derive(TS)]
#[ts(rename_all = "camelCase")]
#[allow(dead_code)]
struct SearchNotesParams {
    query: String,
    #[ts(optional)]
    include_archived: Option<bool>,
}

fn build_bridge() -> Bridge {
    Bridge::tauri()
        .with_assert_never(true)
        .decl(&decl::<NoteKind>())
        .decl(&decl::<NoteStatus>())
        .decl(&decl::<Note>())
        .decl(&decl::<CreateNote>())
        .decl(&decl::<SearchNotesParams>())
        .command(
            Command::returning("note_search", TsType::array(TsType::named("Note")))
                .arg(Arg::rust("params", TsType::named("SearchNotesParams")).optional()),
        )
        .command(
            Command::new("note_create", "Note")
                .arg(Arg::rust("draft", TsType::named("CreateNote"))),
        )
        .command(
            Command::returning("note_archive", TsType::void())
                .arg(Arg::rust("note_id", TsType::string())),
        )
}

fn main() -> ExitCode {
    typeship::cli::run(&build_bridge(), "samples/tauri-ts-rs/generated/api.ts")
}
