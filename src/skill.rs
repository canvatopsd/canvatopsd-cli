use crate::{
    args::{Skill, SkillTarget},
    config,
    error::{CommandError, invalid},
};
use anyhow::Result;
use serde_json::{Value, json};
use std::path::PathBuf;
const FILES: &[(&str, &str)] = &[
    ("SKILL.md", include_str!("../skills/canvatopsd/SKILL.md")),
    (
        "agents/openai.yaml",
        include_str!("../skills/canvatopsd/agents/openai.yaml"),
    ),
    (
        "references/operations.md",
        include_str!("../skills/canvatopsd/references/operations.md"),
    ),
];
fn destination(target: &SkillTarget) -> Result<PathBuf> {
    let parent = if let Some(dir) = &target.dir {
        std::path::absolute(dir)?
    } else {
        let relative =
            match target.agent.as_str() {
                "codex" => ".codex/skills",
                "claude-code" => ".claude/skills",
                "cursor" => ".cursor/skills",
                "copilot" => ".copilot/skills",
                "agents" => ".agents/skills",
                _ => return Err(CommandError::new(
                    "unknown_agent",
                    "Supported agents: codex, claude-code, cursor, copilot, agents; or use --dir",
                )
                .into()),
            };
        if target.agent == "codex" {
            match std::env::var_os("CODEX_HOME") {
                Some(home) => std::path::absolute(home)?.join("skills"),
                None => config::home()?.join(relative),
            }
        } else {
            config::home()?.join(relative)
        }
    };
    let path = parent.join("canvatopsd");
    // Embedded instructions cannot be deleted by replacing an installation.
    Ok(path)
}
pub fn run(command: &Skill) -> Result<Value> {
    let (target, replace) = match command {
        Skill::Show => return Ok(json!({"content":FILES[0].1})),
        Skill::Status(target) => (target, None),
        Skill::Install { target, replace } => (target, Some(*replace)),
    };
    let path = destination(target)?;
    let entry = match std::fs::symlink_metadata(&path) {
        Ok(entry) => Some(entry),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
        Err(e) => return Err(e.into()),
    };
    if entry.as_ref().is_some_and(|v| !v.is_dir()) {
        return Err(invalid(
            "Skill path must be a directory, not a symlink or file",
        ));
    }
    if let Some(replace) = replace {
        if entry.is_some() && !replace {
            return Err(CommandError::new(
                "skill_exists",
                "Skill already exists; inspect skill status and local edits, then use --replace",
            )
            .into());
        }
        if entry.is_some() {
            std::fs::remove_dir_all(&path)?;
        }
        std::fs::create_dir_all(&path)?;
        for (name, content) in FILES {
            let output = path.join(name);
            std::fs::create_dir_all(
                output
                    .parent()
                    .ok_or_else(|| invalid("Invalid Skill path"))?,
            )?;
            std::fs::write(output, content)?;
        }
        return Ok(
            json!({"agent":target.agent,"path":path,"replaced":entry.is_some(),"next":"Run skill status; start a new Agent session to refresh discovery"}),
        );
    }
    if entry.is_none() {
        return Ok(
            json!({"agent":target.agent,"path":path,"installed":false,"current":false,"next":"Run skill install with the same target"}),
        );
    }
    let mut changed = vec![];
    for (name, content) in FILES {
        match std::fs::read(path.join(name)) {
            Ok(bytes) => {
                if bytes != content.as_bytes() {
                    changed.push(*name);
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => changed.push(*name),
            Err(e) => return Err(e.into()),
        }
    }
    Ok(
        json!({"agent":target.agent,"path":path,"installed":true,"current":changed.is_empty(),"changed":changed,"next":"Review differences before replacing; start a new Agent session to refresh discovery"}),
    )
}
