# Run clippy and generate report for SonarQube
cargo clippy --message-format=json > ./target/clippy-report.json

# Run dylint with our custom linter
cargo dylint ec2 --all
