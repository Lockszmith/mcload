# McLoad — Master Plan

> **Product:** McLoad — Rust single binary `mcload`  
> **Tagline:** "There can be only one!"  
> **License:** MIT  
> **Process:** Agentic markdown plans; TDD (fail → pass); commit per step; **never push**

---

## Current epic

| ID       | Plan                                         | Status                                                                     | Summary                                                                                                                                                                                                                              |
| -------- | -------------------------------------------- | -------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **0001** | [0001-scaffolding.md](./0001-scaffolding.md) | **CURRENT — REVISE green**, awaiting MC manual Croft+Loft YES before merge | Waves A–C impl green (`cargo test` **30/30**); Wave D docs alignment. Croft TTY + Loft Web start (ftui 0.7 default-on); tray CLI removed; LF normalized. Awaiting **MC manual Croft/Loft YES** before any merge ask. **Never push.** |

---

## Upcoming epics (stubs — not started)

| ID   | Working title                  | Intent                                                                                                          |
| ---- | ------------------------------ | --------------------------------------------------------------------------------------------------------------- |
| 0002 | Profiling & metadata gathering | Walk FS; persist snapshot; Gathering queue workers                                                              |
| 0003 | Identity & fingerprint plugins | Binary fingerprint + name/size/mime identity; plugin API engagement                                             |
| 0004 | Similarity & SoT merge         | Largest duplicate/similar chunks; merge folders to exclusive source of truth                                    |
| 0005 | Activity & Reckoning runtime   | Pause/Resume/Abort; fresh→ready reckoning; background/tray UX under Loft (no separate tray CLI)                 |
| 0006 | Croft / Loft UI polish         | Polish FrankenTUI Croft + Loft surfaces wired to core — **startup moved into 0001 REVISE**; polish remains here |

*(IDs and titles may change; detailed plans land when an epic is opened.)*

---

## Repo snapshot (planning baseline)

At creation of epic 0001 the tree was essentially empty: git repo with **no commits**, plus `.vscode/mcload.code-workspace` only.

---

## How to use these plans

1. Open the **current epic** markdown; execute TDD steps in order (for 0001 REVISE use [0001-revise-workload-split.md](./0001-revise-workload-split.md)).
2. Check off items and keep architecture/decisions in sync with the code.
3. When an epic’s Definition of Done is met **and** MC has given any required manual confirm, mark it **DONE** here and promote the next epic to **CURRENT**.
4. Do not push remotes unless a human explicitly requests it outside these plans.
5. Do **not** ask to merge 0001 until MC manually confirms Croft and Loft start.
