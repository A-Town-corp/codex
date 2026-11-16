# Windows build orchestration; the CLI and launcher remain native Rust programs.
[CmdletBinding()]
param(
    [string]$TargetDirectory = '',
    [ValidateRange(1, 8)]
    [int]$BuildJobs = 1
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$sourceRoot = Split-Path -Parent $PSScriptRoot
$rustRoot = Join-Path $sourceRoot 'codex-rs'
if ([string]::IsNullOrWhiteSpace($TargetDirectory)) {
    $TargetDirectory = if ($env:CARGO_TARGET_DIR) { $env:CARGO_TARGET_DIR } else { Join-Path $rustRoot 'target' }
}
$TargetDirectory = [IO.Path]::GetFullPath($TargetDirectory)
$releaseDirectory = Join-Path $TargetDirectory 'release'

function Invoke-Checked {
    param([string]$Executable, [string[]]$Arguments)
    & $Executable @Arguments
    if ($LASTEXITCODE -ne 0) {
        throw "$Executable failed with exit code $LASTEXITCODE"
    }
}

# Keep release assertions/behavior while bounding compiler and linker RAM use.
# These override optimization settings only, never sandbox or permission settings.
$buildEnvironment = @{
    CARGO_TARGET_DIR = $TargetDirectory
    CARGO_BUILD_JOBS = [string]$BuildJobs
    CARGO_PROFILE_RELEASE_OPT_LEVEL = '1'
    CARGO_PROFILE_RELEASE_LTO = 'false'
    CARGO_PROFILE_RELEASE_CODEGEN_UNITS = '16'
    CARGO_PROFILE_RELEASE_DEBUG = '0'
    CODEX_LAUNCHER_TEST_ROOT = (Join-Path $TargetDirectory 'local-cost-launcher-tests')
    CODEX_LAUNCHER_TEST_FIXTURE = (Join-Path $releaseDirectory 'codex-launcher-fixture.exe')
    CODEX_LAUNCHER_UNDER_TEST = (Join-Path $releaseDirectory 'codex-launcher.exe')
}
$previousEnvironment = @{}
foreach ($name in $buildEnvironment.Keys) {
    $previousEnvironment[$name] = [Environment]::GetEnvironmentVariable($name, 'Process')
    [Environment]::SetEnvironmentVariable($name, $buildEnvironment[$name], 'Process')
}

Push-Location -LiteralPath $rustRoot
try {
    Write-Output 'Checking footer tests and Rust formatting'
    Invoke-Checked 'just' @('test', '-p', 'codex-tui', '--lib', '-E', 'test(local_token_cost) | test(status_line) | test(status_surface_preview)')
    Invoke-Checked 'cargo' @('fmt', '--all', '--check')
    $launcherSource = Join-Path $PSScriptRoot 'codex_no_daemon_launcher.rs'
    $launcherTests = Join-Path $PSScriptRoot 'codex_no_daemon_launcher_tests.rs'
    $launcherFixture = Join-Path $PSScriptRoot 'codex_launcher_fixture.rs'
    Invoke-Checked 'rustfmt' @('--check', '--edition', '2024', '--config', 'skip_children=true', $launcherSource, $launcherTests, $launcherFixture)

    Write-Output 'Checking all TUI targets and Clippy warnings'
    Invoke-Checked 'cargo' @('check', '--locked', '-p', 'codex-tui', '--all-targets')
    Invoke-Checked 'cargo' @('clippy', '--locked', '-p', 'codex-tui', '--all-targets', '--', '-D', 'warnings')

    Write-Output 'Building the release CLI and native launcher'
    Invoke-Checked 'cargo' @('build', '--locked', '--release', '-p', 'codex-cli', '--bin', 'codex')
    Invoke-Checked 'rustc' @('--edition', '2024', '-D', 'warnings', '-C', 'opt-level=2', $launcherSource, '-o', $env:CODEX_LAUNCHER_UNDER_TEST)
    Invoke-Checked 'rustc' @('--edition', '2024', '-D', 'warnings', $launcherFixture, '-o', $env:CODEX_LAUNCHER_TEST_FIXTURE)
    $testBinary = Join-Path $releaseDirectory 'codex-launcher-tests.exe'
    Invoke-Checked 'rustc' @('--edition', '2024', '-D', 'warnings', '--test', $launcherTests, '-o', $testBinary)
    Invoke-Checked $testBinary @()
    $cliBinary = Join-Path $releaseDirectory 'codex.exe'
    Invoke-Checked $cliBinary @('--version')
    Write-Output "Release CLI: $cliBinary"
    Write-Output "Native launcher: $env:CODEX_LAUNCHER_UNDER_TEST"
} finally {
    Pop-Location
    foreach ($name in $previousEnvironment.Keys) {
        [Environment]::SetEnvironmentVariable($name, $previousEnvironment[$name], 'Process')
    }
}
