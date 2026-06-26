use std::process::ExitCode;

use typeship::ir::{Decl, Field, TsType};
use typeship::{Arg, Bridge, Command};

fn build_bridge() -> Bridge {
    let task_status = Decl::alias(
        "TaskStatus",
        TsType::string_literals(["todo", "doing", "done"]),
    );

    let task_filter = Decl::interface(
        "TaskFilter",
        [
            Field::rust("project_id", TsType::string()).optional(),
            Field::rust("status", TsType::nullable(TsType::named("TaskStatus"))).optional(),
        ],
    );

    let task = Decl::interface(
        "Task",
        [
            Field::rust("id", TsType::string()),
            Field::rust("title", TsType::string()),
            Field::rust("status", TsType::named("TaskStatus")),
            Field::rust(
                "metadata",
                TsType::record(TsType::string(), TsType::unknown()),
            )
            .optional(),
        ],
    );

    Bridge::fetch()
        .decl(&task_status)
        .decl(&task_filter)
        .decl(&task)
        .command(
            Command::returning("list_tasks", TsType::array(TsType::named("Task")))
                .arg(Arg::rust("filter", TsType::named("TaskFilter")).optional()),
        )
        .command(
            Command::returning("set_task_done", TsType::void())
                .arg(Arg::rust("task_id", TsType::string())),
        )
}

fn main() -> ExitCode {
    typeship::cli::run(&build_bridge(), "samples/basic-ir/generated/api.ts")
}
