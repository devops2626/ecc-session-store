```markdown
# ecc-session-store Development Patterns

> Auto-generated skill from repository analysis

## Overview
This skill covers the development conventions and workflows for the `ecc-session-store` Rust codebase. It documents file naming, import/export styles, commit message patterns, and testing conventions, providing clear examples and suggested commands for common tasks. This guide is intended to help contributors maintain consistency and efficiency when working on the project.

## Coding Conventions

### File Naming
- Use **camelCase** for file names.
  - Example: `sessionStore.rs`, `userManager.rs`

### Import Style
- Use **relative imports** within the codebase.
  - Example:
    ```rust
    mod sessionStore;
    use crate::sessionStore::Session;
    ```

### Export Style
- Use **named exports** for modules and functions.
  - Example:
    ```rust
    pub struct SessionStore { /* ... */ }
    pub fn create_session() { /* ... */ }
    ```

### Commit Messages
- Follow **conventional commit** format.
- Use the `feat` prefix for new features.
- Average commit message length: ~47 characters.
  - Example:
    ```
    feat: add session expiration handling
    ```

## Workflows

### Adding a New Feature
**Trigger:** When implementing a new functionality.
**Command:** `/add-feature`

1. Create a new file using camelCase (e.g., `newFeature.rs`).
2. Implement the feature using relative imports as needed.
3. Export structs/functions using named exports.
4. Write or update tests in a corresponding `*.test.*` file.
5. Commit your changes using a conventional commit message:
    ```
    feat: short description of the new feature
    ```
6. Open a pull request for review.

### Running Tests
**Trigger:** When validating code changes.
**Command:** `/run-tests`

1. Identify test files matching the `*.test.*` pattern.
2. Run tests using the Rust testing framework (e.g., `cargo test`).
3. Review test output and address any failures.

## Testing Patterns

- Test files follow the `*.test.*` naming pattern (e.g., `sessionStore.test.rs`).
- The specific testing framework is not documented, but standard Rust practice is to use built-in test modules.
- Example test structure:
    ```rust
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_create_session() {
            // Arrange
            // Act
            // Assert
        }
    }
    ```

## Commands
| Command        | Purpose                                   |
|----------------|-------------------------------------------|
| /add-feature   | Start the workflow for adding a new feature|
| /run-tests     | Run all tests in the codebase             |
```
