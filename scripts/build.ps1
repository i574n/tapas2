param(
    $fast,
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. ../deps/polyglot/scripts/core.ps1


{ pwsh ../contract/build.ps1 } | Invoke-Block

{ pwsh ../scripts/outdated.ps1 } | Invoke-Block
