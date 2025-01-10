#!/bin/bash
# AI GENERATED


# Function to start the database with Docker
start_db() {
  echo "Starting or restarting Docker container for sms_db..."
  docker compose up -d --force-recreate sms_db
}

# Function to run migrations
run_migrations() {
  echo "Running migrations with sea-orm-cli..."
  sea-orm-cli migrate up
}

# Function to run all tests (default mode)
run_all_tests() {
  start_db
  run_migrations
  echo "Running all tests with cargo..."
  cargo test
}

# Function to run non-DB tests
run_non_db_tests() {
  echo "Running non-DB tests individually..."

  # List of test files excluding the DB-dependent test file
  test_files=(
    "tests_alert_services"
    "tests_error_utils"
    "tests_producer"
    "tests_progress_monitor"
    "tests_progress_services"
    "tests_rand_utils"
    "tests_sender_service"
    "tests_sender_utils"
    "tests_senders"
    "tests_sender_utils"
  )

  for test_file in "${test_files[@]}"; do
    echo "Running $test_file..."
    cargo test --test "$(basename "$test_file" .rs)"
  done
}

# Show usage information
usage() {
  echo "Usage: $0 [option]"
  echo "Options:"
  echo "  --all           Run all tests (default)"
  echo "  --non-db        Run only non-database-dependent tests"
  echo "  --help          Show this help message"
}

# Main script logic
case "$1" in
  --all | "")
    run_all_tests
    ;;
  --non-db)
    run_non_db_tests
    ;;
  --help)
    usage
    ;;
  *)
    echo "Unknown option: $1"
    usage
    exit 1
    ;;
esac
