# Constellation Agents Constitution

This document establishes the collaboration protocols and coding standards for all AI agents (Antigravity, Claude, etc.) and human developers working on the Constellation project.

## 1. Core Philosophy
*   **Domain-Driven Design (DDD)**: Architect software around the business domain. Use clear domain models and ubiquitous language in both code and communication.

## 2. Rust Coding Standards
*   **Error Handling (Strict)**:
    *   ❌ **NO** `unwrap()` or `expect()` in production code, unless preventing the panic is impossible or the error is truly unrecoverable/fatal. If used, it MUST be accompanied by a comment explaining why it is safe.
    *   ❌ **NO** `anyhow::Result` in library/backend code.
    *   ✅ **USE** `thiserror` for defining custom, strongly-typed errors.
    *   All public functions must return `Result<T, AppError>`.
*   **Quality**: Code must be clean, maintainable, memory-safe, and efficient.
*   **Formatting**: run `cargo fmt` before every commit.

## 3. Workflow & Collaboration
*   **Stop & Ask**: If requirements are ambiguous, vague, or missing, **HALT** execution immediately. Ask the user for clarification. **DO NOT GUESS**.
*   **Atomic Phases**: Each development phase or bug fix is a unit of work.
    *   Must end with a **Commit**.
    *   Must be merged to `master` (or main branch) upon completion.
*   **Documentation-Code Parity**: When code behavior changes, update the corresponding documentation (`docs/`) and plans (`plans/`) immediately.

## 4. Architecture Specifics (Tauri/Vue)
*   **Type Synchronization**: Ensure Rust models (`src/db/models.rs`, etc.) and TypeScript interfaces (`frontend/stores/*.ts`) stay synchronized. A change in one requires a change in the other.

## 5. Proposed Additions (Pending Approval)
*   **Test-Driven where possible**: Write unit tests for pure logic functions (especially in `src/utils` or `search` logic) to ensure correctness.
*   **Frontend Linting**: Ensure `pnpm lint` or typical Vue best practices are followed (Composition API, proper typing).

---
*Signed: User, Antigravity, Claude*
