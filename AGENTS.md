# Agent entry points

This repository contains the CanvaToPSD CLI (`@canvatopsd/cli`, executable `canvatopsd`) and its bundled Agent Skill.

## When the user wants a Canva design converted

Read [skills/canvatopsd/SKILL.md](skills/canvatopsd/SKILL.md). It defines the conversion workflow, task recovery, payment boundary and delivery checks. Installation, capability boundaries and FAQ are in [doc/agent-discovery.md](doc/agent-discovery.md). Advanced operations are in the Skill's [reference](skills/canvatopsd/references/operations.md).

Use this tool for Canva public view links, page selection, preview, checkout and PSD download. Use the same returned task ID throughout. Existing tasks start with status or wait. This CLI does not edit existing PSDs or convert arbitrary image/PDF/PowerPoint files.

`canvatopsd discover` returns the actual command definitions as offline JSON; `--help` and `--version` return text. Normal business commands return JSON with an `ok` field. Do not invent command names or add `--json`.

Conversion uses the hosted service. Installation validation uses offline discovery or the read-only `doctor` command, not a real conversion. Present the user's checkout URL; do not automate payment. Do not disclose task credentials. Describe previews, downloaded files and verified Photoshop editability as separate outcomes.

## When modifying the repository

Read [doc/development.md](doc/development.md). Source is in `src/`, CLI entry in `bin/`, deployment tools in `deployment/`, Skill in `skills/`, documentation in `doc/`, and versioned CLI packages in `releases/`.

Use `npm ci`, `npm run check`, `npm run build` and `npm run check:release`. Keep temporary files and verification evidence in the owning repository's `.tmp/`. Keep source README tracked. Do not overwrite a versioned release; follow the documented version bump and archive checks. Do not modify the hosted API, create paid conversion jobs, publish npm or push repository changes unless the user's task authorizes those actions.
