# Contributing to uncver-artifacts

Thank you for your interest in contributing to **uncver-artifacts**! We welcome contributions from the community to help make this tool better.

## Development Setup

1. **Fork and Clone**:
   ```bash
   git clone https://github.com/your-username/uncver-artifacts.git
   cd uncver-artifacts
   ```

2. **Install Rust**:
   Ensure you have the latest stable Rust installed.
   ```bash
   rustup update stable
   ```

3. **Install Podman**:
   Since the CLI depends on Podman, make sure it's installed and initialized on your system.

## Workflow

### 1. Create a Branch
Always work on a new branch for your changes:
```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/your-bug-fix
```

### 2. Implementation
- Follow the existing code style.
- Use `tracing` for logging instead of `println!`.
- Ensure platform-specific code is wrapped in `#[cfg(...)]`.

### 3. Testing
Run the test suite to ensure no regressions:
```bash
cargo test
```

### 4. Committing
We use **Conventional Commits** to automate our release process:
- `feat: ...` for new features
- `fix: ...` for bug fixes
- `docs: ...` for documentation changes
- `chore: ...` for maintenance

### 5. Pull Request
Push your branch and open a PR against the `main` branch.

## Code of Conduct

Please be respectful and constructive in all interactions. We aim to foster a welcoming environment for everyone.

## Need Help?

Open an issue or reach out to the maintainers.
