#!/usr/bin/env pwsh

# Create target directory if it doesn't exist
if (-not (Test-Path -Path ".\target")) {
    New-Item -ItemType Directory -Path ".\target" | Out-Null
}

# Run clippy and generate the report
Write-Host "Running Clippy and generating report..." -ForegroundColor Green
cargo clippy --message-format=json > .\target\clippy-report.json 2>&1
if ($LASTEXITCODE -ne 0) {
    # If clippy fails, create an empty report
    Write-Host "Clippy encountered errors. Creating empty report file." -ForegroundColor Yellow
    Set-Content -Path ".\target\clippy-report.json" -Value "[]"
}

# Check if SonarQube is running in a Docker container
try {
    $response = Invoke-WebRequest -Uri "http://localhost:9000" -Method Head -TimeoutSec 5 -ErrorAction SilentlyContinue
    if ($response.StatusCode -eq 200) {
        Write-Host "SonarQube is running. Starting analysis..." -ForegroundColor Green
        
        # Run sonar-scanner
        sonar-scanner
    } else {
        Write-Host "SonarQube is not responding correctly. Make sure it's running." -ForegroundColor Yellow
    }
} catch {
    Write-Host "SonarQube is not running. Please start it with 'docker-compose up' in the ecocode-rust-sonar directory." -ForegroundColor Yellow
}
