$ErrorActionPreference = 'Stop'

# 1) Navigate to the root of the Rust folder (where this script is stored).
$rustRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location $rustRoot

# 2) Ensure the current location is the Rust root folder and a Git repo.
if (-not (Test-Path (Join-Path $rustRoot '.git'))) {
    Write-Error "This script must live in the root of your Git repo (Rust folder). '.git' was not found in: $rustRoot"
}

$currentPath = (Get-Location).Path
if ($currentPath -ne $rustRoot) {
    Write-Error "Failed to move to Rust root. Current: $currentPath | Expected: $rustRoot"
}

Write-Host "Repo root confirmed: $rustRoot" -ForegroundColor Green

# 3) Run git add .
git add .

# 4) Allow custom commit message input.
$commitMessage = Read-Host 'Enter commit message'
if ([string]::IsNullOrWhiteSpace($commitMessage)) {
    Write-Error 'Commit message cannot be empty.'
}

# 5) Run git commit -m "MY_INPUT_MESSAGE"
git commit -m "$commitMessage"

# 6) Run git push
git push

Write-Host 'Done: add, commit, and push completed.' -ForegroundColor Green
