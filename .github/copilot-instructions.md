# Advent of Code Solutions Repository

**ALWAYS follow these instructions first and only fallback to additional search and context gathering if the information here is incomplete or found to be in error.**

This repository contains Rust solutions for [Advent of Code](https://adventofcode.com/) problems from 2015-2024. It uses a cargo workspace structure with each year as a separate crate and each day as a separate binary.

## Working Effectively

### Prerequisites
- Rust and Cargo must be installed
- Valid Advent of Code session token required for fetching puzzle input

### Essential Commands with Timing
Execute these commands from the repository root:

**Build the entire workspace:**
```bash
cargo build
```
- Takes ~47 seconds to complete. **NEVER CANCEL** - set timeout to 90+ minutes.
- Produces warnings about lifetime elision (this is expected).

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

**Lint code (fails currently):**
```bash
cargo clippy -- -D warnings
```
- Takes ~45 seconds but **FAILS** due to existing lifetime warnings in aoc/multiset.rs and other files.
- **NEVER CANCEL** - set timeout to 90+ minutes.
- Do not try to fix these warnings unless specifically asked.

**Run solution for specific day:**
```bash
cargo run --bin y24d12
```
- Requires SESSION environment variable (see setup below).
- Without SESSION, fails with "Environment variable error: environment variable not found".

### Session Token Setup
To run actual solutions (not just tests), set your Advent of Code session token:

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

5. **Solution execution test:**
   ```bash
   cargo run --bin y24d12
   ```
   - Should fail gracefully without SESSION token
   - With valid SESSION, should fetch input and display results

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
5. Test with real input using `cargo run --bin y<YY>d<DD>`

### Debugging Test Failures
- Individual day tests use hardcoded example inputs
- Tests call `solve_for_test()` method, not `solve_for_answer()`
- Multiple test functions allowed per day (see y24d12.rs with 5 examples)

## Important Notes

- **WARNING**: Some tests take very long (y15d04 is ignored for this reason)
- **WARNING**: Clippy currently fails - do not attempt to fix unless specifically requested
- The repository builds and tests successfully despite warnings
- Input fetching requires internet access to adventofcode.com
- Session tokens expire - refresh if getting authentication errors
- Each year is completely independent - changes in one year don't affect others

## CI/CD Information
GitHub Actions workflow (`.github/workflows/rust.yml`) runs:
1. `cargo clippy -- -D warnings` (currently fails)
2. `cargo build` 
3. `cargo test`

The clippy step will fail in CI due to existing warnings. This is a known issue.