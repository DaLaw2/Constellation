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
*   **Error Handling (Strict)**:
    *   ❌ **NO** `unwrap()` or `expect()` in production code, unless preventing the panic is impossible or the error is truly unrecoverable/fatal. If used, it MUST be accompanied by a comment explaining why it is safe.
    *   ❌ **NO** `anyhow` (e.g., `anyhow::Result`) anywhere in the codebase. Do not add it as a dependency.
    *   ✅ **USE** `thiserror` for defining custom, strongly-typed errors.
    *   All public functions must return `Result<T, AppError>`.
*   **Quality**: Code must be clean, maintainable, memory-safe, and efficient.
*   **Formatting**: run `cargo fmt` before every commit.
*   **Anti-Patterns**:
    *   ❌ **NO** Singletons or Service Locators (Registry pattern). Avoid global mutable state.
*   **Concurrency**:
    *   ❌ **NO** Locks (`Mutex`, `RwLock`) generally. Only exception is for low-level resource synchronization where absolutely necessary.
    *   ✅ **USE** thread-safe concurrent containers (e.g., `DashMap`, `SegQueue`) for shared state.

## 4. Architecture Specifics (Tauri/Vue)
*   **Type Synchronization**: Ensure Rust models (`src-tauri/src/application/dto.rs`) and TypeScript interfaces (`src/types/`) stay synchronized. A change in one requires a change in the other.

## 5. Knowledge Base Management

The project maintains a categorized knowledge base at `docs/knowledge/`.

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

### Rules
*   **Categorize**: Place knowledge in the appropriate subfolder by topic.
*   **Single Responsibility**: One topic per file. Keep files focused and scannable.
*   **Include Code**: Always include working code examples with context.
*   **Reference Sources**: Add external links (docs, crates, articles) at the end.
*   **Update on Discovery**: When solving a non-trivial problem or learning a new pattern, add it to the knowledge base immediately.
*   **No Monolithic Files**: Avoid large "everything" documentation files. Split by topic.

---
*Signed: User, Antigravity, Claude*
