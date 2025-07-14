# Contributing to Ved DeFi

We welcome contributions to Ved DeFi! This document provides guidelines for contributing to our open source decentralized finance project.

## 🎯 Project Vision

Ved DeFi aims to create a truly decentralized alternative to centralized financial systems through:
- **VedCoin**: Cryptocurrency for individuals and businesses
- **VedGov**: Direct inter-government payment system

## 🛠️ Development Setup

### Prerequisites
- Rust 1.70+
- Git
- Linux/macOS (Windows support in development)

### Local Development
```bash
# Clone the main repository
git clone https://github.com/Ved-DeFi/Ved-DeFi.git
cd Ved-DeFi

# Initialize submodules
git submodule update --init --recursive

# Build VedCoin
cd VedCoin
cargo build --release

# Build VedGov  
cd ../VedGov
cargo build --release

# Run tests
cargo test
```

## 📝 Contributing Guidelines

### Code Standards
- **Rust**: Follow rustfmt guidelines (`cargo fmt`)
- **Documentation**: All public APIs must be documented
- **Testing**: New features require comprehensive tests
- **Security**: Security-sensitive code requires additional review

### Development Process
1. **Fork** the repository
2. **Create** a feature branch from `main`
3. **Implement** your changes with tests
4. **Run** `cargo fmt` and `cargo clippy`
5. **Ensure** all tests pass with `cargo test`
6. **Submit** a pull request with clear description

### Commit Message Format
```
type(scope): brief description

Detailed explanation of the changes (optional)

Fixes #issue_number (if applicable)
```

**Types**: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

**Examples**:
- `feat(vedcoin): add staking rewards distribution`
- `fix(vedgov): resolve multi-signature validation bug`
- `docs(readme): update installation instructions`

## 🔍 Areas for Contribution

### High Priority
- **Core blockchain implementation** (Rust)
- **Consensus mechanism development**
- **Smart contract security**
- **Cross-chain bridge protocols**
- **Wallet applications**

### Medium Priority
- **Developer tooling**
- **Documentation improvements**
- **Test coverage expansion**
- **Performance optimizations**
- **Mobile wallet development**

### Beginner Friendly
- **Documentation updates**
- **Example code and tutorials**
- **Bug reports and testing**
- **Translation work**
- **UI/UX improvements**

## 🔒 Security Contributions

Security is paramount in blockchain development. For security contributions:

### Security Review Process
1. **Security-sensitive PRs** require additional reviewers
2. **Formal verification** for critical components
3. **Comprehensive testing** including edge cases
4. **Code audit** before major releases

### Reporting Security Issues
- **DO NOT** create public issues for security vulnerabilities
- **Email**: security@ved-defi.org (create issue on GitHub for now)
- **Include**: Detailed description, steps to reproduce, impact assessment
- **Response**: We will respond within 48 hours

## 🧪 Testing Guidelines

### Test Requirements
- **Unit tests** for all new functionality
- **Integration tests** for cross-component interactions
- **Property-based tests** for complex algorithms
- **Benchmark tests** for performance-critical code

### Running Tests
```bash
# Run all tests
cargo test

# Run specific test suite
cargo test --package pallet-vedcoin

# Run with coverage
cargo tarpaulin --out Html
```

## 📚 Documentation Standards

### Code Documentation
```rust
/// Brief description of the function
/// 
/// # Arguments
/// * `param1` - Description of parameter 1
/// * `param2` - Description of parameter 2
/// 
/// # Returns
/// Description of return value
/// 
/// # Errors
/// Description of possible errors
/// 
/// # Examples
/// ```
/// let result = example_function(param1, param2);
/// ```
pub fn example_function(param1: Type1, param2: Type2) -> Result<ReturnType, Error> {
    // Implementation
}
```

### API Documentation
- Use clear, concise descriptions
- Provide practical examples
- Document error conditions
- Include performance characteristics

## 🌐 Community Guidelines

### Communication
- **Be respectful** and professional
- **Be constructive** in feedback and criticism
- **Be patient** with newcomers and questions
- **Be collaborative** in problem-solving

### Code Review
- **Focus on code**, not the person
- **Provide specific** and actionable feedback
- **Explain reasoning** behind suggestions
- **Acknowledge good work** when reviewing

## 🏷️ Issue and PR Labels

### Issue Labels
- `bug`: Something isn't working correctly
- `enhancement`: New feature or improvement
- `documentation`: Documentation needs
- `good first issue`: Good for newcomers
- `help wanted`: Extra attention is needed
- `security`: Security-related issue
- `performance`: Performance-related issue

### PR Labels
- `work in progress`: PR is not ready for review
- `needs review`: PR is ready for review
- `needs testing`: PR needs additional testing
- `breaking change`: PR introduces breaking changes

## 📋 Pull Request Checklist

Before submitting a pull request, ensure:

- [ ] Code follows project style guidelines
- [ ] All tests pass locally
- [ ] New tests cover the changes
- [ ] Documentation is updated if needed
- [ ] Commit messages follow the format
- [ ] No merge conflicts with main branch
- [ ] PR description clearly explains the changes
- [ ] Related issues are referenced

## 🚀 Release Process

Ved DeFi follows semantic versioning (SemVer):
- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

### Release Schedule
- **Major releases**: Quarterly
- **Minor releases**: Monthly
- **Patch releases**: As needed for critical fixes

## 📞 Getting Help

### Development Support
- **GitHub Discussions**: For general questions and ideas
- **GitHub Issues**: For bugs and feature requests
- **Code Review**: Request review from maintainers

### Mentorship
New contributors can request mentorship from experienced contributors for:
- Understanding the codebase
- Learning Rust and blockchain development
- Guidance on contribution areas

## 🎉 Recognition

We recognize contributions through:
- **Contributor list** in README
- **Release notes** mention significant contributions
- **Special badges** for major contributors
- **Conference opportunities** for active contributors

## 📄 Legal

By contributing to Ved DeFi, you agree that your contributions will be licensed under the MIT License.

---

**Thank you for contributing to Ved DeFi! Together, we're building the future of decentralized finance.**
