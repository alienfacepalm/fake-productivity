# Implementation Plan - Codebase Refactoring

## Goal Description
Refactor the `fake-productivity` application to adhere to Rust best practices. The primary goal is to **activate the currently dead/unlinked code** in `src/` and decompose the monolithic `main.rs` into smaller, maintainable modules. This will improve code organization and maintainability without changing the external behavior of the application.

## Proposed Changes

### Logic Distribution
I will move logic from `src/main.rs` into the following modules:

#### `main.rs`
- Add `mod` declarations for `matrix`, `productivity`, `terminal`, `utils`, `font`.
- Remove local `generate_*_log` functions.
- Remove local `generate_matrix_code` function.
- Remove local `setup_fullscreen_terminal`, `enter_fullscreen`, `exit_fullscreen` (move to `terminal.rs`).
- Update imports to use the new modules.

#### `productivity.rs`
- Move `generate_system_log`, `generate_database_log`, `generate_network_log`, etc. from `main.rs` to here.
- Make functions `pub`.

#### `matrix.rs`
- Update `generate_matrix_code` with the implementation from `main.rs`.

#### `terminal.rs`
- Move terminal setup/teardown logic here.

#### `utils.rs`
- Move helper functions like `get_timestamp` here.

## Verification Plan

### Automated Tests
- Run `cargo test` (I will add a basic test for the new modules if possible).
- Run `cargo check` to ensure no unused imports or dead code warnings remain.

### Manual Verification
- Run the application with `cargo run`.
- Verify "normal" log output appears.
- Verify "matrix mode" triggers with `Ctrl+M`.
- Verify "fullscreen" toggle with `Ctrl+F`.
- Verify "quick mode" with `cargo run -- --quick`.
