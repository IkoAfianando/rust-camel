FROM rust:1.75-slim-bookworm

# Install required packages
RUN apt-get update && \
    apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

# Copy the entire project
COPY . .

# Build the application
RUN cargo build

# Expose port
EXPOSE 8080

# Run the application
CMD ["cargo", "run"]