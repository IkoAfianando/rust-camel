#!/bin/bash

case $1 in
  "build")
    echo "Building development environment..."
    docker-compose build app-dev
    ;;

  "start")
    echo "Starting development environment..."
    docker-compose up app-dev
    ;;

  "stop")
    echo "Stopping development environment..."
    docker-compose down
    ;;

  "test")
    echo "Running tests..."
    docker-compose run --rm app-dev cargo test
    ;;

  "prod-build")
    echo "Building production image..."
    docker-compose build app-prod
    ;;

  "prod-start")
    echo "Starting production environment..."
    docker-compose up -d app-prod
    ;;

  "logs")
    echo "Showing logs..."
    docker-compose logs -f
    ;;

  "clean")
    echo "Cleaning up..."
    docker-compose down -v
    ;;

  *)
    echo "Usage: ./dev.sh [command]"
    echo "Commands:"
    echo "  build       - Build development environment"
    echo "  start       - Start development environment"
    echo "  stop        - Stop development environment"
    echo "  test        - Run tests"
    echo "  prod-build  - Build production image"
    echo "  prod-start  - Start production environment"
    echo "  logs        - Show logs"
    echo "  clean       - Clean up containers and volumes"
    ;;
esac