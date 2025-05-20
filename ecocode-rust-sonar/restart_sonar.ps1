Write-Host "Stopping all SonarQube containers..."
docker-compose down

Write-Host "Removing old volume data..."
if (Test-Path ./sonarqube_data) {
    Remove-Item -Path ./sonarqube_data -Recurse -Force -ErrorAction SilentlyContinue
}
if (Test-Path ./sonarqube_logs) {
    Remove-Item -Path ./sonarqube_logs -Recurse -Force -ErrorAction SilentlyContinue
}

Write-Host "Starting SonarQube services..."
docker-compose up -d sonarqube db

Write-Host "Waiting for SonarQube to start (this may take a few minutes)..."
$sonarqubeReady = $false
$attempts = 0
$maxAttempts = 30

while (-not $sonarqubeReady -and $attempts -lt $maxAttempts) {
    $attempts++
    
    try {
        $response = Invoke-WebRequest -Uri "http://localhost:9000/api/system/status" -Method Get -UseBasicParsing -ErrorAction SilentlyContinue
        
        if ($response.StatusCode -eq 200) {
            $content = $response.Content | ConvertFrom-Json
            if ($content.status -eq "UP") {
                $sonarqubeReady = $true
                Write-Host "SonarQube is up and running!"
            }
        }
    } catch {
        Write-Host "Waiting for SonarQube to initialize... (attempt $attempts of $maxAttempts)"
    }
    
    if (-not $sonarqubeReady) {
        Start-Sleep -Seconds 10
    }
}

if (-not $sonarqubeReady) {
    Write-Host "SonarQube failed to start within the expected time. Check the logs for details."
    exit 1
}

Write-Host "Starting sonar-scanner..."
docker-compose up sonar-scanner

Write-Host "Done. SonarQube should be available at http://localhost:9000"
Write-Host "Default login: admin/admin"
