# DevNet - Local SynapseNet cluster for testing (Windows)
# Usage: .\scripts\devnet.ps1 [start|stop|status|clean]

param(
    [Parameter(Position=0)]
    [ValidateSet("start", "stop", "status", "clean")]
    [string]$Action = "start"
)

$ErrorActionPreference = "Stop"

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Split-Path -Parent $ScriptDir
$DevNetDir = Join-Path $ProjectRoot ".devnet"
$NumNodes = 3

function Write-Info {
    param([string]$Message)
    Write-Host "[INFO] $Message" -ForegroundColor Green
}

function Write-Warn {
    param([string]$Message)
    Write-Host "[WARN] $Message" -ForegroundColor Yellow
}

function Write-Error-Custom {
    param([string]$Message)
    Write-Host "[ERROR] $Message" -ForegroundColor Red
}

function Start-DevNet {
    Write-Info "Starting DevNet with $NumNodes nodes..."
    
    # Build project
    Write-Info "Building SynapseNet..."
    Push-Location $ProjectRoot
    cargo build --release
    Pop-Location
    
    # Create devnet directory
    New-Item -ItemType Directory -Force -Path $DevNetDir | Out-Null
    
    # Start nodes
    for ($i = 1; $i -le $NumNodes; $i++) {
        $NodeDir = Join-Path $DevNetDir "node$i"
        New-Item -ItemType Directory -Force -Path $NodeDir | Out-Null
        
        Write-Info "Starting node $i..."
        
        # Initialize node if not exists
        $KeyPath = Join-Path $NodeDir "node.key"
        if (-not (Test-Path $KeyPath)) {
            & "$ProjectRoot\target\release\syn.exe" --data-dir $NodeDir init
        }
        
        Write-Info "Node $i ready at $NodeDir"
    }
    
    Write-Info "DevNet started successfully!"
    Write-Info "Node directories:"
    for ($i = 1; $i -le $NumNodes; $i++) {
        Write-Host "  Node $i: $DevNetDir\node$i"
    }
    
    Write-Info ""
    Write-Info "Try these commands:"
    Write-Host "  syn --data-dir $DevNetDir\node1 add `"Hello from node 1`""
    Write-Host "  syn --data-dir $DevNetDir\node1 query `"Hello`""
}

function Stop-DevNet {
    Write-Info "Stopping DevNet..."
    
    # Kill any running node processes (when P2P is implemented)
    # Get-Process | Where-Object {$_.ProcessName -like "*syn*"} | Stop-Process -Force
    
    Write-Info "DevNet stopped"
}

function Get-DevNetStatus {
    Write-Info "DevNet status:"
    
    if (-not (Test-Path $DevNetDir)) {
        Write-Warn "DevNet not initialized"
        return
    }
    
    for ($i = 1; $i -le $NumNodes; $i++) {
        $NodeDir = Join-Path $DevNetDir "node$i"
        if (Test-Path $NodeDir) {
            Write-Host "  Node $i: initialized"
        } else {
            Write-Host "  Node $i: not initialized"
        }
    }
}

function Clear-DevNet {
    Write-Warn "Cleaning DevNet (all data will be lost)..."
    if (Test-Path $DevNetDir) {
        Remove-Item -Recurse -Force $DevNetDir
    }
    Write-Info "DevNet cleaned"
}

switch ($Action) {
    "start" { Start-DevNet }
    "stop" { Stop-DevNet }
    "status" { Get-DevNetStatus }
    "clean" { Clear-DevNet }
}
