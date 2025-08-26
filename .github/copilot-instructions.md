# Advent of Code Solutions Repository

**ALWAYS follow these instructions first and only fallback to additional search and context gathering if the information here is incomplete or found to be in error.**

This repository contains Rust solutions for [Advent of Code](https://adventofcode.com/) problems from 2015-2024. It uses a cargo workspace structure with each year as a separate crate and each day as a separate binary.

## Working Effectively

### Prerequisites
- Rust and Cargo must be installed
- Session token NOT required for Copilot tasks (tests run without it)

### Essential Commands with Timing
Execute these commands from the repository root:

**Build the entire workspace:**
```bash
cargo build
```
- Takes ~47 seconds to complete. **NEVER CANCEL** - set timeout to 90+ minutes.
- Completes successfully with warnings (lifetime elision warnings are expected).

**Run all tests:**
```bash
cargo test
```
- Takes ~23 seconds to complete. **NEVER CANCEL** - set timeout to 45+ minutes.
- One test (y15d04::examples) is ignored due to long runtime.

**Run tests for specific year:**
```bash
cargo test -p aoc2024
```
- Takes ~0.2 seconds. Much faster than full test suite.

**Run tests for specific day:**
```bash
cargo test --bin y24d12
```
- Runs only tests for that specific day's solution.
- Very fast (under 0.1 seconds).

**Lint code:**
```bash
cargo clippy -- -D warnings
```
- Takes ~45 seconds to complete. **NEVER CANCEL** - set timeout to 90+ minutes.
- Currently fails due to lifetime warnings in aoc/multiset.rs, but improvements are expected.


### Session Token Setup
**NOTE: Session tokens are NOT needed for Copilot tasks. Only use if you need to run actual solutions manually.**

To run actual solutions (not needed for Copilot), set your Advent of Code session token:

**Option 1 - Environment variable:**
```bash
export SESSION=your_session_token_here
```

**Option 2 - .env file:**
Create `.env` file in repository root:
```
SESSION=your_session_token_here
```

Get session token by inspecting cookies in browser developer tools when logged into adventofcode.com.

### Creating New Day Solutions
Use the xtask helper to create new day templates:

```bash
cargo xtask new 2024 25
```
- Creates new day file from template
- Automatically sets YEAR and DAY constants
- Years must be 2015-2024, days must be 1-25

## Repository Structure

### Workspace Layout
```
├── aoc/                    # Common utilities library
├── 2015/ to 2024/         # Year-specific solution crates
├── xtask/                 # Task automation tool
├── day_template.rs        # Template for new solutions
├── template/yXXdYY.rs     # Older template format
└── .github/workflows/     # CI configuration
```

### Solution File Pattern
Each day follows this pattern:
- Location: `YYYY/src/bin/yYYdDD.rs` (e.g., `2024/src/bin/y24d12.rs`)
- Contains `parse`, `task1`, `task2` functions
- Uses `Solution` struct from aoc library
- Includes test module with example inputs

### Common Utilities (aoc crate)
Key modules in the `aoc` crate:
- `input.rs` - Fetches puzzle input via HTTP
- `solution.rs` - Solution framework and runner
- `vec2d.rs` - 2D grid utilities
- `multiset.rs` - Multiset operations
- `pairs.rs` - Pair iteration utilities
- `permute.rs` - Permutation utilities

## Validation Scenarios

**ALWAYS perform these validation steps after making changes:**

1. **Build validation:**
   ```bash
   cargo build
   ```
   - Must complete successfully (warnings OK)
   - Wait full ~47 seconds, **NEVER CANCEL**

2. **Test validation:**
   ```bash
   cargo test
   ```
   - Must pass (1 ignored test OK)
   - Wait full ~23 seconds, **NEVER CANCEL**

3. **Specific year tests:**
   ```bash
   cargo test -p aoc2024
   ```
   - Tests only the specified year
   - Faster feedback loop for year-specific changes

4. **Specific day tests:**
   ```bash
   cargo test --bin y24d12
   ```
   - Tests only the specified day
   - Fastest feedback for single day changes

5. **Solution execution validation (NOT for Copilot):**
   ```bash
   cargo test --bin y24d12
   ```
   - Use tests instead of running actual solutions
   - Copilot should NEVER run solutions to find answers

## Common Tasks

### Viewing Example Solutions
Recent complete solutions for reference:
- `2024/src/bin/y24d12.rs` - Grid-based problem with multiple examples
- `2024/src/bin/y24d15.rs` - State manipulation with 3 test cases
- `2023/src/bin/y23d10.rs` - Path finding example

### Adding New Solution
1. Use `cargo xtask new <year> <day>` to create template
2. Implement `parse`, `task1`, `task2` functions
3. Add test cases with known example inputs
4. Run `cargo test -p aoc<year>` to validate
5. Run `cargo test -p aoc<year>` to validate (DO NOT run actual solutions)

### Debugging Test Failures
- Individual day tests use hardcoded example inputs
- Tests call `solve_for_test()` method, not `solve_for_answer()`
- Multiple test functions allowed per day (see y24d12.rs with 5 examples)

## Important Notes

- **🚨 CRITICAL: Copilot must NEVER solve Advent of Code problems. The user wants to solve them personally without AI assistance.**
- **Copilot can help with:** linting errors, refactoring, build issues, test fixes, code quality improvements
- **Copilot must NOT:** implement solutions, provide algorithm hints, or run binaries to find answers
- **WARNING**: Some tests take very long (y15d04 is ignored for this reason)
- The repository builds and tests successfully
- Input fetching requires internet access to adventofcode.com
- Session tokens expire - refresh if getting authentication errors
- Each year is completely independent - changes in one year don't affect others

## CI/CD Information
GitHub Actions workflow (`.github/workflows/rust.yml`) runs:
1. `cargo clippy -- -D warnings` (currently fails due to lifetime warnings)
2. `cargo build` 
3. `cargo test`

Build and test steps should pass successfully.