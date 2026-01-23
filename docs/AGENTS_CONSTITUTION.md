# Constellation Agents Constitution

This document establishes the collaboration protocols and coding standards for all AI agents (Antigravity, Claude, etc.) and human developers working on the Constellation project.

## 1. Workflow & Collaboration
*   **Stop & Ask**: If requirements are ambiguous, vague, or missing, **HALT** execution immediately. Ask the user for clarification. **DO NOT GUESS**.
*   **Atomic Phases**: Each development phase or bug fix is a unit of work.
    *   Must end with a **Commit**.
    *   Must be merged to `master` (or main branch) upon completion.
*   **Documentation-Code Parity**: When code behavior changes, update the corresponding documentation (`docs/`) and plans (`plans/`) immediately.
*   **English Everything**: All code comments, documentation, plans, and commit messages must be written in English. Check existing files and convert them if necessary.

## 2. Core Philosophy
*   **Domain-Driven Design (DDD)**: Architect software around the business domain. Use clear domain models and ubiquitous language in both code and communication.
    *   **Strict Alignment**: When introducing or modifying DDD core structures (Aggregates, Entities, Value Objects), **HALT** and align with the user.
    *   **No Assumptions**: Do NOT make decisions based on "common patterns" or "reasonable guesses".

## 3. Rust Coding Standards
*   **Quality**: Code must be clean, maintainable, memory-safe, and efficient.
*   **Error Handling (Strict)**:
    *   ❌ **NO** `unwrap()` or `expect()` in production code, unless preventing the panic is impossible or the error is truly unrecoverable/fatal. If used, it MUST be accompanied by a comment explaining why it is safe.
    *   ❌ **NO** `anyhow` (e.g., `anyhow::Result`) anywhere in the codebase. Do not add it as a dependency.
    *   ✅ **USE** `thiserror` for defining custom, strongly-typed errors.
    *   All public functions must return `Result<T, AppError>`.
*   **Concurrency**:
    *   ❌ **NO** Locks (`Mutex`, `RwLock`) generally. Only exception is for low-level resource synchronization where absolutely necessary.
    *   ✅ **USE** thread-safe concurrent containers (e.g., `DashMap`, `SegQueue`) for shared state.
*   **Anti-Patterns**:
    *   ❌ **NO** Singletons or Service Locators (Registry pattern). Avoid global mutable state.
*   **Formatting**: run `cargo fmt` before every commit.

## 4. Architecture Specifics (Tauri/Vue)
*   **Type Synchronization**: Ensure Rust models (`src-tauri/src/application/dto.rs`) and TypeScript interfaces (`src/types/`) stay synchronized. A change in one requires a change in the other.

## 5. Knowledge Base Maintenance (Agent Obligation)

All AI agents working on this project **MUST** maintain the knowledge base at `docs/knowledge/`.

### Structure
```
docs/knowledge/
├── architecture/     # DDD patterns, Tauri patterns, design decisions
├── integrations/     # External APIs (Windows, system tray, etc.)
├── performance/      # Optimization techniques (SQLite, virtual scrolling)
├── security/         # Validation, CSP, path traversal prevention
├── troubleshooting/  # Problem solutions and debugging guides
└── api/              # API design, query language specs
```

### Mandatory Requirements
*   **Document Non-Trivial Solutions**: When you solve a non-trivial problem, implement a new pattern, or discover important technical details, you **MUST** document it in the knowledge base **immediately** before completing the task.
*   **Categorize Correctly**: Place knowledge in the appropriate subfolder by topic. If uncertain, ask the user.
*   **Single Responsibility**: One topic per file. Keep files focused and scannable.
*   **Include Working Code**: Always include working code examples with full context.
*   **Reference Sources**: Add external links (official docs, crates.io, articles) at the end of each document.
*   **No Monolithic Files**: Split large topics into focused, individual files.
*   **Update Existing Docs**: If you discover better approaches or corrections to existing knowledge base entries, update them.

---
*Signed: User, Antigravity, Claude*
