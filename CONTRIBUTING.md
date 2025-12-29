# Contributing to DevKit

First off, thank you for considering contributing to DevKit! It's people like you that make DevKit such a great tool.

## Code of Conduct

By participating in this project, you are expected to uphold our [Code of Conduct](CODE_OF_CONDUCT.md).

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check the existing issues as you might find out that you don't need to create one. When you are creating a bug report, please include as many details as possible:

* **Use a clear and descriptive title**
* **Describe the exact steps to reproduce the problem**
* **Provide specific examples**
* **Describe the behavior you observed and what you expected**
* **Include details about your configuration and environment**

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, please include:

* **Use a clear and descriptive title**
* **Provide a detailed description of the suggested enhancement**
* **Explain why this enhancement would be useful**
* **List some other tools where this enhancement exists, if applicable**

### Pull Requests

1. Fork the repo and create your branch from `main`
2. If you've added code that should be tested, add tests
3. Ensure the test suite passes (`cargo test`)
4. Make sure your code lints (`cargo clippy`)
5. Format your code (`cargo fmt`)
6. Write a clear commit message

## Development Process

1. **Fork and Clone**
   ```bash
   git clone https://github.com/YOUR_USERNAME/devkit.git
   cd devkit
   ```

2. **Create a Branch**
   ```bash
   git checkout -b feature/my-new-feature
   ```

3. **Make Changes**
   - Write clean, documented code
   - Follow existing code style
   - Add tests for new functionality

4. **Test**
   ```bash
   cargo test
   cargo clippy -- -D warnings
   cargo fmt --check
   ```

5. **Commit**
   ```bash
   git commit -m "Add feature: description"
   ```

6. **Push and PR**
   ```bash
   git push origin feature/my-new-feature
   ```

## Coding Standards

* Use `cargo fmt` for consistent formatting
* Run `cargo clippy` and fix all warnings
* Write clear, self-documenting code
* Add doc comments for public functions
* Keep functions small and focused
* Avoid unwrap() where possible, use proper error handling

## Testing

* Add unit tests for new functionality
* Ensure all tests pass before submitting PR
* Test on multiple platforms if possible

## Documentation

* Update README.md if you change functionality
* Add doc comments to public APIs
* Update CHANGELOG.md with your changes

## License

By contributing to DevKit, you agree that your contributions will be licensed under its MIT License.

## Questions?

Feel free to open an issue or reach out to the maintainers.

Thank you for contributing! 🎉
