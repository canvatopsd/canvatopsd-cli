use clap::{Args, Parser, Subcommand};

pub const MAX_REQUEST_SECONDS: u64 = 3600;

fn ui_locales() -> clap::builder::PossibleValuesParser {
    let schema: serde_json::Value = serde_json::from_str(include_str!("ui-locale.schema.json"))
        .expect("Bundled UI locale schema");
    clap::builder::PossibleValuesParser::new(
        schema["enum"]
            .as_array()
            .expect("UI locale codes")
            .iter()
            .map(|value| value.as_str().expect("UI locale code").to_owned())
            .collect::<Vec<_>>(),
    )
}

#[derive(Parser)]
#[command(name = "canvatopsd", version = env!("CARGO_PKG_VERSION"), about = "Canva to PSD CLI for AI agents. Convert public Canva links to editable layered Photoshop files. Business results are JSON.", subcommand_help_heading = "Common commands", after_help = "Start: canvatopsd discover | canvatopsd skill show\nWorkflow: convert URL → preview TASK --out FILE → checkout TASK → user payment → download TASK --out FILE\nMulti-page: convert URL --pages 1,3; then use --page 3. No --json flag is needed.\nAdvanced: create, files, init, resolve, operator, request. Use COMMAND --help or discover for details.")]
pub struct Cli {
    #[command(flatten)]
    pub options: Options,
    #[command(subcommand)]
    pub command: Command,
}
#[derive(Args, Default)]
pub struct Options {
    /// Web endpoint ending /api, or direct API origin
    #[arg(long, global = true)]
    pub base_url: Option<String>,
    /// Origin header matching the configured Web origin
    #[arg(long, global = true, hide = true)]
    pub origin: Option<String>,
    /// Per-request timeout in seconds
    #[arg(long, global = true, default_value_t = 30, value_parser = clap::value_parser!(u64).range(1..=MAX_REQUEST_SECONDS))]
    pub timeout: u64,
}
#[derive(Subcommand)]
pub enum Command {
    /// Convert a Canva link and wait for previews; retain one task ID for subsequent commands
    Convert {
        #[command(flatten)]
        input: Submission,
        /// Design page numbers, e.g. 1,3; required for multi-page designs
        #[arg(long, value_name = "numbers")]
        pages: Option<String>,
        /// Maximum wait after submission; timeout does not cancel conversion
        #[arg(long, default_value_t = 300, value_parser = clap::value_parser!(u64).range(1..))]
        seconds: u64,
    },
    /// Submit a design and save task credentials; does not wait
    #[command(hide = true)]
    Create(Submission),
    /// Read conversion progress and selected page numbers
    Status {
        /// Existing task UUID returned by convert or create
        task: String,
    },
    /// Wait for all selected pages; stop at required input or deadline
    Wait {
        /// Existing task UUID returned by convert or create
        task: String,
        /// Maximum wait in seconds; polls every 5 seconds without cancelling work
        #[arg(long, default_value_t = 300, value_parser = clap::value_parser!(u64).range(1..))]
        seconds: u64,
    },
    /// Discover source pages; a single-page design may start automatically
    Pages {
        /// Existing task UUID returned by convert or create
        task: String,
        /// Request fresh source-page discovery
        #[arg(long)]
        refresh: bool,
    },
    /// Select design pages to convert; keep the same task ID
    Select {
        /// Existing task UUID returned by convert or create
        task: String,
        /// Comma-separated positive design page numbers, e.g. 1,3
        pages: String,
        /// Reuse the submission UUID after an uncertain response
        #[arg(long)]
        key: Option<String>,
        /// Inspect the command input without submitting this action
        #[arg(long)]
        dry_run: bool,
    },
    /// List previews, or save one with --out
    Preview {
        #[command(flatten)]
        selection: Selection,
        /// Write to a new output file; existing files are never overwritten
        #[arg(long)]
        out: Option<String>,
        /// Explicit internal artifact page ID; normally use --page instead
        #[arg(long, hide = true)]
        artifact_page: Option<String>,
        /// Preview artifact to list or save
        #[arg(long, hide = true, default_value = "preview.webp", value_parser = ["preview.webp", "preview-layers.webp", "preview-layers.json"])]
        file: String,
    },
    /// Synchronize payment and download a PSD or fonts; verify size and SHA-256
    Download {
        #[command(flatten)]
        selection: Selection,
        /// Write to a new output file; existing files are never overwritten
        #[arg(long)]
        out: String,
        /// Explicit internal artifact page ID; normally use --page instead
        #[arg(long, hide = true)]
        artifact_page: Option<String>,
        /// Paid PSD or font artifact to download
        #[arg(long, hide = true, default_value = "page-0001.psd", value_parser = ["page-0001.psd", "fonts.zip", "fonts.txt"])]
        file: String,
    },
    /// List paid PSD and font files
    #[command(hide = true)]
    Files(Selection),
    /// Create a hosted payment link; share checkoutUrl unchanged with the user
    Checkout {
        #[command(flatten)]
        selection: Selection,
        /// Inspect the command input without submitting this action
        #[arg(long)]
        dry_run: bool,
    },
    /// Read payment status or confirm an authorized payment
    Payment(PaymentArgs),
    /// Compatibility alias for payment confirm; may capture approved funds
    #[command(hide = true)]
    Confirm(Selection),
    /// Check API protocol and configuration; submits no conversion jobs
    Doctor,
    /// Parse a public Canva link or private task link locally; no network
    #[command(hide = true)]
    Resolve {
        /// Public Canva URL or private task URL to parse without networking
        url: String,
    },
    /// Save endpoint settings; defaults to prod
    #[command(hide = true)]
    Init {
        /// HTTPS service endpoint, or loopback HTTP; defaults to prod
        url: Option<String>,
    },
    /// Import, export or inspect local task credentials
    Auth {
        #[command(subcommand)]
        command: Auth,
    },
    /// Read, inspect or install bundled Agent instructions; offline
    Skill {
        #[command(subcommand)]
        command: Skill,
    },
    /// Direct API operator reads; requires CANVATOPSD_OPERATOR_TOKEN
    #[command(hide = true)]
    Operator {
        #[command(subcommand)]
        command: Operator,
    },
    /// Send an explicit POST command; side effects depend on the API action
    #[command(hide = true)]
    Request {
        /// POST command path such as /task/status
        path: String,
        /// File containing the JSON request body
        #[arg(long)]
        body: String,
        /// Use CANVATOPSD_OPERATOR_TOKEN authorization
        #[arg(long)]
        operator: bool,
        /// Write to a new output file; existing files are never overwritten
        #[arg(long)]
        out: Option<String>,
    },
    /// Offline JSON command index generated from actual command definitions
    Discover {
        /// Command path, e.g. skill install; quoted paths remain supported
        #[arg(num_args = 0..)]
        command: Vec<String>,
    },
}
#[derive(Args)]
pub struct Submission {
    /// Canva Share → Public view link, or copied canva.link share URL
    pub url: String,
    /// Optional notification email
    #[arg(long)]
    pub email: Option<String>,
    /// Task page and notification language; omission uses the API default
    #[arg(long, value_parser = ui_locales())]
    pub locale: Option<String>,
    /// Reuse this UUID after an uncertain response
    #[arg(long)]
    pub key: Option<String>,
    /// Use local layer names without model naming
    #[arg(long)]
    pub no_layer_naming: bool,
    /// Validate locally without submitting or saving credentials
    #[arg(long)]
    pub dry_run: bool,
}
#[derive(Args, Clone)]
pub struct Selection {
    /// Original task UUID returned by convert; reuse it for every selected page
    pub task: String,
    /// Design page number; required when multiple pages were converted
    #[arg(long, value_parser = clap::value_parser!(u32).range(1..))]
    pub page: Option<u32>,
}
#[derive(Subcommand)]
pub enum Auth {
    /// Import a credential JSON file or private task link file for this endpoint
    Import {
        /// Credential JSON or complete private task link file
        file: String,
    },
    /// Write one secret task credential backup; does not overwrite files
    Export {
        /// Task UUID with a direct local credential; export its parent when needed
        task: String,
        /// New secret credential backup file
        file: String,
    },
    /// Show credential availability without printing secrets
    Status,
    /// Remove endpoint credentials and retry tokens; does not revoke server grants
    Clear,
}
#[derive(Subcommand)]
pub enum Skill {
    /// Read bundled SKILL.md as JSON without installation
    Show,
    /// Compare installed Skill files with this binary's bundled instructions
    Status(SkillTarget),
    /// Install bundled Skill; --replace replaces the entire existing directory
    Install {
        #[command(flatten)]
        target: SkillTarget,
        /// Replace the entire installed Skill directory, including local edits
        #[arg(long)]
        replace: bool,
    },
}
#[derive(Args)]
pub struct SkillTarget {
    /// codex, claude-code, cursor, copilot or agents; use --dir for other targets
    #[arg(default_value = "codex")]
    pub agent: String,
    /// Custom skills parent directory, e.g. /project/.agents/skills
    #[arg(long)]
    pub dir: Option<String>,
}
#[derive(Subcommand)]
pub enum Operator {
    /// List tasks; server pagination is 100 tasks per page
    TaskList {
        /// Server task-list offset; each page contains at most 100 tasks
        #[arg(long, default_value_t = 0)]
        offset: u64,
    },
    /// Read task notification records
    NotificationStatus {
        /// Existing task UUID to inspect with operator authorization
        task: String,
    },
}

#[derive(Args)]
#[command(
    arg_required_else_help = true,
    subcommand_precedence_over_arg = true,
    args_conflicts_with_subcommands = true
)]
pub struct PaymentArgs {
    #[command(subcommand)]
    pub command: Option<Payment>,
    /// Compatibility form: payment TASK [--page N]; prefer payment status TASK
    pub task: Option<String>,
    /// Original design page number for the compatibility form
    #[arg(long, requires = "task", value_parser = clap::value_parser!(u32).range(1..))]
    pub page: Option<u32>,
}

#[derive(Subcommand)]
pub enum Payment {
    /// Read and synchronize payment state
    Status(Selection),
    /// Confirm an authorized payment; may capture approved funds
    Confirm(Selection),
}
