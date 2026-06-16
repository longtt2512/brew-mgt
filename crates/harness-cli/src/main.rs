//! harness-cli — durable operational layer for the brew-mgt harness.
//!
//! Stores intake classifications, story status, decisions, traces, and backlog
//! in a local SQLite database (`harness.db`). The database is per-clone and
//! git-ignored; the schema lives under `scripts/schema/`.

use std::path::PathBuf;
use std::process::Command as ProcessCommand;

use anyhow::{anyhow, bail, Context, Result};
use chrono::Utc;
use clap::{Args, Parser, Subcommand};
use rusqlite::{params, Connection};

const VALID_LANES: [&str; 3] = ["tiny", "normal", "high-risk"];
const VALID_OUTCOMES: [&str; 4] = ["success", "partial", "blocked", "reverted"];

#[derive(Parser)]
#[command(
    name = "harness-cli",
    version,
    about = "Durable operational layer for the brew-mgt harness."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create harness.db and apply scripts/schema/*.sql.
    Init,
    /// Record an intake classification.
    Intake(IntakeArgs),
    /// Manage story packets and proof status.
    Story {
        #[command(subcommand)]
        action: StoryAction,
    },
    /// Mirror feature_list.json into the durable layer.
    Feature {
        #[command(subcommand)]
        action: FeatureAction,
    },
    /// Manage durable decision records.
    Decision {
        #[command(subcommand)]
        action: DecisionAction,
    },
    /// Record an execution trace for a completed task.
    Trace(TraceArgs),
    /// Manage the harness growth backlog.
    Backlog {
        #[command(subcommand)]
        action: BacklogAction,
    },
    /// Read-only queries over the durable layer.
    Query {
        #[command(subcommand)]
        action: QueryAction,
    },
}

#[derive(Args)]
struct IntakeArgs {
    /// Input type: new-spec | spec-slice | change-request | new-initiative | maintenance | harness-improvement
    #[arg(long = "type")]
    kind: String,
    /// One-line summary of the request.
    #[arg(long)]
    summary: String,
    /// Risk lane: tiny | normal | high-risk
    #[arg(long)]
    lane: String,
}

#[derive(Subcommand)]
enum StoryAction {
    /// Add a new story.
    Add {
        #[arg(long)]
        id: String,
        #[arg(long)]
        title: String,
        #[arg(long)]
        lane: String,
        /// Optional mechanical proof command (run by `story verify`).
        #[arg(long)]
        verify: Option<String>,
    },
    /// Update status, proof booleans (1/0), or the verify command.
    Update {
        #[arg(long)]
        id: String,
        #[arg(long)]
        status: Option<String>,
        #[arg(long)]
        unit: Option<i64>,
        #[arg(long)]
        integration: Option<i64>,
        #[arg(long)]
        e2e: Option<i64>,
        #[arg(long)]
        platform: Option<i64>,
        #[arg(long)]
        verify: Option<String>,
    },
    /// Run a story's configured verify command. Exit 0 on pass, 1 on fail.
    Verify {
        id: String,
    },
    /// Run every configured story verify command. Exit 1 if any fail.
    VerifyAll,
}

#[derive(Subcommand)]
enum FeatureAction {
    /// Read feature_list.json and upsert each feature into the stories table.
    Sync {
        /// Path to the feature list (defaults to ./feature_list.json).
        #[arg(long, default_value = "feature_list.json")]
        file: String,
    },
}

#[derive(Subcommand)]
enum DecisionAction {
    /// Add or refresh a durable decision record.
    Add {
        #[arg(long)]
        id: String,
        #[arg(long)]
        title: String,
        #[arg(long)]
        doc: String,
        #[arg(long)]
        notes: Option<String>,
    },
}

#[derive(Args)]
struct TraceArgs {
    #[arg(long)]
    summary: String,
    /// Outcome: success | partial | blocked | reverted
    #[arg(long)]
    outcome: String,
    #[arg(long)]
    story: Option<String>,
    #[arg(long)]
    friction: Option<String>,
}

#[derive(Subcommand)]
enum BacklogAction {
    /// Add a backlog item born from friction.
    Add {
        #[arg(long)]
        title: String,
        #[arg(long)]
        pain: String,
        /// Risk lane: tiny | normal | high-risk
        #[arg(long, default_value = "normal")]
        risk: String,
        #[arg(long)]
        predicted: Option<String>,
    },
}

#[derive(Subcommand)]
enum QueryAction {
    /// Behavior-to-proof matrix from the stories table.
    Matrix {
        /// Print proof values as 1/0 instead of yes/no.
        #[arg(long)]
        numeric: bool,
    },
    /// List backlog items.
    Backlog {
        #[arg(long)]
        open: bool,
        #[arg(long)]
        closed: bool,
    },
    /// Counts across the durable layer.
    Stats,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Init => cmd_init(),
        Commands::Intake(a) => cmd_intake(a),
        Commands::Story { action } => match action {
            StoryAction::Add {
                id,
                title,
                lane,
                verify,
            } => cmd_story_add(id, title, lane, verify),
            StoryAction::Update {
                id,
                status,
                unit,
                integration,
                e2e,
                platform,
                verify,
            } => cmd_story_update(id, status, unit, integration, e2e, platform, verify),
            StoryAction::Verify { id } => cmd_story_verify(id),
            StoryAction::VerifyAll => cmd_story_verify_all(),
        },
        Commands::Feature { action } => match action {
            FeatureAction::Sync { file } => cmd_feature_sync(file),
        },
        Commands::Decision { action } => match action {
            DecisionAction::Add {
                id,
                title,
                doc,
                notes,
            } => cmd_decision_add(id, title, doc, notes),
        },
        Commands::Trace(a) => cmd_trace(a),
        Commands::Backlog { action } => match action {
            BacklogAction::Add {
                title,
                pain,
                risk,
                predicted,
            } => cmd_backlog_add(title, pain, risk, predicted),
        },
        Commands::Query { action } => match action {
            QueryAction::Matrix { numeric } => cmd_query_matrix(numeric),
            QueryAction::Backlog { open, closed } => cmd_query_backlog(open, closed),
            QueryAction::Stats => cmd_query_stats(),
        },
    }
}

// ---- paths & connection -----------------------------------------------------

fn db_path() -> PathBuf {
    PathBuf::from("harness.db")
}

/// Resolve scripts/schema next to the executable, falling back to ./scripts/schema.
fn schema_dir() -> PathBuf {
    if let Ok(exe) = std::env::current_exe() {
        // exe at scripts/bin/harness-cli -> scripts/schema
        if let Some(bin) = exe.parent() {
            if let Some(scripts) = bin.parent() {
                let candidate = scripts.join("schema");
                if candidate.is_dir() {
                    return candidate;
                }
            }
        }
    }
    PathBuf::from("scripts/schema")
}

fn open_db() -> Result<Connection> {
    let path = db_path();
    if !path.exists() {
        bail!("harness.db not found. Run `harness-cli init` first.");
    }
    Connection::open(&path).context("opening harness.db")
}

fn now() -> String {
    Utc::now().to_rfc3339()
}

fn validate_lane(lane: &str) -> Result<()> {
    if VALID_LANES.contains(&lane) {
        Ok(())
    } else {
        Err(anyhow!(
            "invalid lane '{lane}'. Use one of: {}",
            VALID_LANES.join(", ")
        ))
    }
}

fn validate_bool(name: &str, v: i64) -> Result<()> {
    if v == 0 || v == 1 {
        Ok(())
    } else {
        Err(anyhow!("{name} must be 1 or 0 (got {v})"))
    }
}

// ---- commands ---------------------------------------------------------------

fn cmd_init() -> Result<()> {
    let path = db_path();
    if path.exists() {
        bail!(
            "{} already exists. Delete it to re-initialize, or just use the existing DB.",
            path.display()
        );
    }
    let conn = Connection::open(&path).context("creating harness.db")?;
    let dir = schema_dir();
    let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)
        .with_context(|| format!("reading schema dir {}", dir.display()))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().map(|x| x == "sql").unwrap_or(false))
        .collect();
    files.sort();
    if files.is_empty() {
        bail!("no .sql files found in {}", dir.display());
    }
    for f in &files {
        let sql = std::fs::read_to_string(f)
            .with_context(|| format!("reading {}", f.display()))?;
        conn.execute_batch(&sql)
            .with_context(|| format!("applying {}", f.display()))?;
        println!("applied {}", f.display());
    }
    conn.execute(
        "INSERT INTO schema_version (version, applied_at) VALUES (?1, ?2)",
        params![files.len() as i64, now()],
    )?;
    println!("initialized {} (schema v{})", path.display(), files.len());
    Ok(())
}

fn cmd_feature_sync(file: String) -> Result<()> {
    let conn = open_db()?;
    let raw = std::fs::read_to_string(&file)
        .with_context(|| format!("reading {file}"))?;
    let doc: serde_json::Value =
        serde_json::from_str(&raw).with_context(|| format!("parsing {file}"))?;
    let features = doc
        .get("features")
        .and_then(|f| f.as_array())
        .ok_or_else(|| anyhow!("{file}: expected a top-level \"features\" array"))?;

    let mut synced = 0usize;
    for feat in features {
        let id = feat
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("a feature is missing a string \"id\""))?;
        let title = feat.get("title").and_then(|v| v.as_str()).unwrap_or("");
        let lane = feat.get("lane").and_then(|v| v.as_str()).unwrap_or("normal");
        validate_lane(lane)?;
        let status = feat
            .get("status")
            .and_then(|v| v.as_str())
            .unwrap_or("not_started");
        let area = feat.get("area").and_then(|v| v.as_str());
        let priority = feat.get("priority").and_then(|v| v.as_i64());
        let behavior = feat.get("user_visible_behavior").and_then(|v| v.as_str());
        let evidence = feat.get("evidence").and_then(|v| v.as_str());

        conn.execute(
            "INSERT INTO stories
                (id, title, lane, status, area, priority, user_visible_behavior, evidence, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
             ON CONFLICT(id) DO UPDATE SET
                title = excluded.title,
                lane = excluded.lane,
                status = excluded.status,
                area = excluded.area,
                priority = excluded.priority,
                user_visible_behavior = excluded.user_visible_behavior,
                evidence = excluded.evidence",
            params![id, title, lane, status, area, priority, behavior, evidence, now()],
        )?;
        synced += 1;
    }
    println!("synced {synced} feature(s) from {file} into harness.db");
    Ok(())
}

fn cmd_intake(a: IntakeArgs) -> Result<()> {
    validate_lane(&a.lane)?;
    let conn = open_db()?;
    conn.execute(
        "INSERT INTO intake (created_at, kind, summary, lane) VALUES (?1, ?2, ?3, ?4)",
        params![now(), a.kind, a.summary, a.lane],
    )?;
    println!("intake recorded: [{}] {} ({})", a.lane, a.summary, a.kind);
    Ok(())
}

fn cmd_story_add(id: String, title: String, lane: String, verify: Option<String>) -> Result<()> {
    validate_lane(&lane)?;
    let conn = open_db()?;
    conn.execute(
        "INSERT INTO stories (id, title, lane, status, verify_command, created_at)
         VALUES (?1, ?2, ?3, 'proposed', ?4, ?5)
         ON CONFLICT(id) DO UPDATE SET
            title = excluded.title,
            lane = excluded.lane,
            verify_command = COALESCE(excluded.verify_command, stories.verify_command)",
        params![id, title, lane, verify, now()],
    )?;
    println!("story {id} saved: {title} [{lane}]");
    Ok(())
}

fn cmd_story_update(
    id: String,
    status: Option<String>,
    unit: Option<i64>,
    integration: Option<i64>,
    e2e: Option<i64>,
    platform: Option<i64>,
    verify: Option<String>,
) -> Result<()> {
    let conn = open_db()?;
    if !story_exists(&conn, &id)? {
        bail!("story '{id}' not found. Add it first with `story add`.");
    }
    if let Some(s) = &status {
        conn.execute(
            "UPDATE stories SET status = ?1 WHERE id = ?2",
            params![s, id],
        )?;
    }
    for (name, val) in [
        ("unit", unit),
        ("integration", integration),
        ("e2e", e2e),
        ("platform", platform),
    ] {
        if let Some(v) = val {
            validate_bool(name, v)?;
            // Column name is a fixed literal from the loop, never user input.
            let sql = format!("UPDATE stories SET {name} = ?1 WHERE id = ?2");
            conn.execute(&sql, params![v, id])?;
        }
    }
    if let Some(cmd) = &verify {
        conn.execute(
            "UPDATE stories SET verify_command = ?1 WHERE id = ?2",
            params![cmd, id],
        )?;
    }
    println!("story {id} updated");
    Ok(())
}

fn story_exists(conn: &Connection, id: &str) -> Result<bool> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM stories WHERE id = ?1",
        params![id],
        |r| r.get(0),
    )?;
    Ok(count > 0)
}

fn cmd_story_verify(id: String) -> Result<()> {
    let conn = open_db()?;
    let cmd: Option<String> = conn
        .query_row(
            "SELECT verify_command FROM stories WHERE id = ?1",
            params![id],
            |r| r.get(0),
        )
        .map_err(|_| anyhow!("story '{id}' not found"))?;
    let Some(cmd) = cmd else {
        bail!("story '{id}' has no verify command. Set one with `story update --verify`.");
    };
    let passed = run_verify(&id, &cmd, &conn)?;
    if passed {
        Ok(())
    } else {
        std::process::exit(1);
    }
}

fn cmd_story_verify_all() -> Result<()> {
    let conn = open_db()?;
    let mut stmt = conn.prepare(
        "SELECT id, verify_command FROM stories WHERE verify_command IS NOT NULL ORDER BY id",
    )?;
    let rows: Vec<(String, String)> = stmt
        .query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)))?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    if rows.is_empty() {
        println!("no stories have a verify command configured.");
        return Ok(());
    }
    let mut any_fail = false;
    for (id, cmd) in rows {
        let passed = run_verify(&id, &cmd, &conn)?;
        if !passed {
            any_fail = true;
        }
    }
    if any_fail {
        std::process::exit(1);
    }
    Ok(())
}

/// Run a verify command from the repo root, record the result, return pass/fail.
fn run_verify(id: &str, cmd: &str, conn: &Connection) -> Result<bool> {
    println!("verifying {id}: {cmd}");
    let status = ProcessCommand::new("sh")
        .arg("-c")
        .arg(cmd)
        .status()
        .with_context(|| format!("spawning verify command for {id}"))?;
    let passed = status.success();
    let result = if passed { "pass" } else { "fail" };
    conn.execute(
        "UPDATE stories SET last_verified_at = ?1, last_verified_result = ?2 WHERE id = ?3",
        params![now(), result, id],
    )?;
    println!("  {id}: {result}");
    Ok(passed)
}

fn cmd_decision_add(id: String, title: String, doc: String, notes: Option<String>) -> Result<()> {
    let conn = open_db()?;
    conn.execute(
        "INSERT INTO decisions (id, title, doc, notes, created_at) VALUES (?1, ?2, ?3, ?4, ?5)
         ON CONFLICT(id) DO UPDATE SET
            title = excluded.title,
            doc = excluded.doc,
            notes = excluded.notes",
        params![id, title, doc, notes, now()],
    )?;
    println!("decision {id} saved: {title} -> {doc}");
    Ok(())
}

fn cmd_trace(a: TraceArgs) -> Result<()> {
    if !VALID_OUTCOMES.contains(&a.outcome.as_str()) {
        bail!(
            "invalid outcome '{}'. Use one of: {}",
            a.outcome,
            VALID_OUTCOMES.join(", ")
        );
    }
    let conn = open_db()?;
    // Advisory: warn if linked story's verify has never passed.
    if let Some(story) = &a.story {
        if story_exists(&conn, story)? {
            let last: Option<String> = conn.query_row(
                "SELECT last_verified_result FROM stories WHERE id = ?1",
                params![story],
                |r| r.get(0),
            )?;
            if last.as_deref() != Some("pass") {
                eprintln!("warning: story {story} has no passing verification on record.");
            }
        } else {
            eprintln!("warning: trace references unknown story {story}.");
        }
    }
    conn.execute(
        "INSERT INTO traces (created_at, summary, outcome, story, friction)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![now(), a.summary, a.outcome, a.story, a.friction],
    )?;
    let trace_id = conn.last_insert_rowid();
    println!("trace #{trace_id} recorded [{}]: {}", a.outcome, a.summary);
    Ok(())
}

fn cmd_backlog_add(
    title: String,
    pain: String,
    risk: String,
    predicted: Option<String>,
) -> Result<()> {
    validate_lane(&risk)?;
    let conn = open_db()?;
    conn.execute(
        "INSERT INTO backlog (created_at, title, pain, risk, predicted, status)
         VALUES (?1, ?2, ?3, ?4, ?5, 'open')",
        params![now(), title, pain, risk, predicted],
    )?;
    let id = conn.last_insert_rowid();
    println!("backlog #{id} added [{risk}]: {title}");
    Ok(())
}

fn cmd_query_matrix(numeric: bool) -> Result<()> {
    let conn = open_db()?;
    let mut stmt = conn.prepare(
        "SELECT id, lane, unit, integration, e2e, platform, status FROM stories ORDER BY id",
    )?;
    let rows = stmt.query_map([], |r| {
        Ok((
            r.get::<_, String>(0)?,
            r.get::<_, String>(1)?,
            r.get::<_, i64>(2)?,
            r.get::<_, i64>(3)?,
            r.get::<_, i64>(4)?,
            r.get::<_, i64>(5)?,
            r.get::<_, String>(6)?,
        ))
    })?;
    let fmt = |v: i64| -> String {
        if numeric {
            v.to_string()
        } else if v == 1 {
            "yes".to_string()
        } else {
            "no".to_string()
        }
    };
    println!(
        "{:<8} {:<10} {:<5} {:<11} {:<5} {:<8} {}",
        "story", "lane", "unit", "integration", "e2e", "platform", "status"
    );
    let mut any = false;
    for row in rows {
        let (id, lane, u, i, e, p, status) = row?;
        any = true;
        println!(
            "{:<8} {:<10} {:<5} {:<11} {:<5} {:<8} {}",
            id,
            lane,
            fmt(u),
            fmt(i),
            fmt(e),
            fmt(p),
            status
        );
    }
    if !any {
        println!("(no stories yet)");
    }
    Ok(())
}

fn cmd_query_backlog(open: bool, closed: bool) -> Result<()> {
    let conn = open_db()?;
    let filter = match (open, closed) {
        (true, false) => "WHERE status = 'open'",
        (false, true) => "WHERE status = 'closed'",
        _ => "",
    };
    let sql = format!(
        "SELECT id, risk, status, title, pain FROM backlog {filter} ORDER BY id"
    );
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map([], |r| {
        Ok((
            r.get::<_, i64>(0)?,
            r.get::<_, String>(1)?,
            r.get::<_, String>(2)?,
            r.get::<_, String>(3)?,
            r.get::<_, String>(4)?,
        ))
    })?;
    let mut any = false;
    for row in rows {
        let (id, risk, status, title, pain) = row?;
        any = true;
        println!("#{id} [{status}/{risk}] {title} — {pain}");
    }
    if !any {
        println!("(no backlog items)");
    }
    Ok(())
}

fn cmd_query_stats() -> Result<()> {
    let conn = open_db()?;
    let count = |table: &str| -> Result<i64> {
        let sql = format!("SELECT COUNT(*) FROM {table}");
        Ok(conn.query_row(&sql, [], |r| r.get(0))?)
    };
    println!("intake records : {}", count("intake")?);
    println!("stories        : {}", count("stories")?);
    println!("decisions      : {}", count("decisions")?);
    println!("traces         : {}", count("traces")?);
    println!("backlog items  : {}", count("backlog")?);
    let open_backlog: i64 = conn.query_row(
        "SELECT COUNT(*) FROM backlog WHERE status = 'open'",
        [],
        |r| r.get(0),
    )?;
    println!("  open backlog : {open_backlog}");
    Ok(())
}
