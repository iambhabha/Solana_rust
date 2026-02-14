# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2024-02-14

### Added
- **Docker Support**: Added `Dockerfile` and `docker-compose.yml` for containerized deployment.
- **Automated Setup Scripts**: Added `setup.bat` (Windows) and `setup.sh` (Linux/Mac) for easy configuration.
- **Observability**: Added structured JSON logging using `tracing`.
- **Health Check**: Added `/health` endpoint for monitoring service status.
- **CORS Support**: Enabled Cross-Origin Resource Sharing for frontend integration.
- **CI/CD**: Added GitHub Actions for testing, releasing, and publishing Docker images.
- **Documentation**: Comprehensive `README.md` rewrite, `CONTRIBUTING.md`, and `LICENSE`.
- **Issue Templates**: Checklists for Bug Reports and Feature Requests.

### Changed
- **Codebase**: Refactored `main.rs` to use `tracing` instead of `println!`.
- **Configuration**: Standardized environment variable loading via `.env` file.

### Fixed
- **Build Size**: Optimized Docker image size using multi-stage builds.
