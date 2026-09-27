# McLoad — Master Plan

> **Product:** McLoad — Rust single binary `mcload`  
> **Tagline:** "There can be only one!"  
> **License:** MIT  
> **Process:** Agentic markdown plans; TDD (fail → pass); commit per step; **never push**

---

## Current epic

| ID       | Plan                                         | Status                                       | Summary                                                                                                                                                                                                              |
| -------- | -------------------------------------------- | -------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **0001** | [0001-scaffolding.md](./0001-scaffolding.md) | **CURRENT — DONE** (awaiting explicit close) | Initial scaffolding: Cargo binary, CLI modes, FrankenTUI hooks, tray stub, YAML config, concurrent metadata API, queue stubs, dev-container, README/LICENSE, minimal tests, cross-compile notes — verified GREEN 25/25 |

---

## Upcoming epics (stubs — not started)

| ID   | Working title                  | Intent                                                                       |
| ---- | ------------------------------ | ---------------------------------------------------------------------------- |
| 0002 | Profiling & metadata gathering | Walk FS; persist snapshot; Gathering queue workers                           |
| 0003 | Identity & fingerprint plugins | Binary fingerprint + name/size/mime identity; plugin API engagement          |
| 0004 | Similarity & SoT merge         | Largest duplicate/similar chunks; merge folders to exclusive source of truth |
| 0005 | Activity & Reckoning runtime   | Pause/Resume/Abort; fresh→ready reckoning; background/tray UX                |
| 0006 | Croft / Loft UI polish         | Real FrankenTUI Croft + Loft surfaces wired to core                          |

*(IDs and titles may change; detailed plans land when an epic is opened.)*

---

## Repo snapshot (planning baseline)

At creation of epic 0001 the tree was essentially empty: git repo with **no commits**, plus `.vscode/mcload.code-workspace` only.

---

## How to use these plans

1. Open the **current epic** markdown; execute TDD steps in order.
2. Check off items and keep architecture/decisions in sync with the code.
3. When an epic’s Definition of Done is met, mark it **DONE** here and promote the next epic to **CURRENT**.
4. Do not push remotes unless a human explicitly requests it outside these plans.
