# Rust Message Processor (Rust Camel)

A Rust implementation of enterprise integration patterns inspired by Apache Camel. This project demonstrates how to build a robust message processing system using Rust's async capabilities and clean architecture principles.

## 🌟 Features

- Message processing pipeline similar to Apache Camel
- Clean Architecture implementation
- Async processing with Tokio
- REST API with Actix-web
- Docker support for development and production
- Comprehensive test suite
- Health check endpoints
- Logging and error handling
- Type-safe message processing

## 🏗️ Architecture

The project follows Clean Architecture principles with the following layers:

```
src/
├── domain/          # Core business logic and interfaces
│   ├── models/      # Domain entities
│   └── ports/       # Interface definitions
├── application/     # Use cases and business rules
│   ├── processors/  # Message processors
│   ├── pipeline/    # Processing pipeline
│   └── services/    # Application services
├── infrastructure/  # External implementations
│   ├── repositories/
│   └── adapters/
└── interfaces/      # Delivery mechanisms
    ├── api/         # REST API
    └── cli/         # Command line interface
```

## 🚀 Getting Started

### Prerequisites

- Rust 1.75 or higher
- Docker and Docker Compose (optional)
- curl (for testing)

### Local Development

1. Clone the repository:
```bash
git clone https://github.com/yourusername/rust-camel.git
cd rust-camel
```

2. Build the project:
```bash
cargo build
```

3. Run tests:
```bash
cargo test
```

4. Run the application:
```bash
cargo run
```

### Using Docker

1. Development environment:
```bash
# Build and start development environment
./dev.sh build
./dev.sh start

# Run tests in Docker
./dev.sh test
```

2. Production environment:
```bash
# Build and start production environment
./dev.sh prod-build
./dev.sh prod-start
```

## 📡 API Endpoints

### Create Message
```bash
curl -X POST http://localhost:8080/api/messages \
  -H "Content-Type: application/json" \
  -d '{"body":"This is a test message"}'
```

### Process Message
```bash
curl -X POST http://localhost:8080/api/messages/process \
  -H "Content-Type: application/json" \
  -d '{
    "message_id":"<ID_FROM_PREVIOUS_RESPONSE>",
    "additional_data":"optional extra information"
  }'
```

### Health Check
```bash
curl http://localhost:8080/health
```

## 🔧 Configuration

The application can be configured using environment variables:

```bash
RUST_LOG=debug              # Log level (debug, info, warn, error)
RUST_BACKTRACE=1           # Enable backtraces
```

## 🧪 Testing

The project includes several types of tests:

1. Unit tests:
```bash
cargo test
```

2. Specific test:
```bash
cargo test test_process_message
```

3. With logging:
```bash
RUST_LOG=debug cargo test -- --nocapture
```

## 📦 Available Processors

1. **LoggingProcessor**
    - Logs message processing events
    - Configurable prefix

2. **EnricherProcessor**
    - Adds metadata to messages
    - Configurable enrichment data

3. **TransformProcessor**
    - Transforms message content
    - Customizable transformation functions

4. **FilterProcessor**
    - Filters messages based on conditions
    - Configurable predicates

## 🛠️ Development Tools

The project includes several development tools:

```bash
# Development script
./dev.sh [command]

Available commands:
  build       - Build development environment
  start       - Start development environment
  stop        - Stop development environment
  test        - Run tests
  prod-build  - Build production image
  prod-start  - Start production environment
  logs        - Show logs
  clean       - Clean up containers and volumes
```

## 🔒 Security

- Non-root user in production Docker image
- Proper error handling
- Input validation
- No sensitive data logging

## 📊 Monitoring

- Health check endpoint
- Logging with different levels
- Docker healthcheck configuration

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgments

- Inspired by Apache Camel
- Built with Rust and its amazing ecosystem
- Thanks to all contributors

## 📚 Further Reading

- [Clean Architecture](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html)
- [Enterprise Integration Patterns](https://www.enterpriseintegrationpatterns.com/)
- [Rust Async Book](https://rust-lang.github.io/async-book/)
- [Actix Web Documentation](https://actix.rs/docs/)

## 🤔 FAQ

**Q: Why use Rust instead of Java/Apache Camel?**
A: Rust offers memory safety, zero-cost abstractions, and excellent performance. This implementation provides similar patterns with Rust's benefits.

**Q: How do I add a new processor?**
A: Implement the `Processor` trait for your new processor and add it to the pipeline. See existing processors for examples.

**Q: How do I customize the processing pipeline?**
A: Modify the pipeline configuration in `main.rs` to add, remove, or reorder processors.

## 📧 Contact

Your Name - [@Iko Afianando](https://twitter.com/venxfa)

Project Link: [https://github.com/IkoAfianando/rust-camel](https://github.com/yourusername/rust-camel)