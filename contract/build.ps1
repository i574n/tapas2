param(
    $fast,
    $SkipNotebook,
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. ../deps/polyglot/scripts/core.ps1
. ../deps/polyglot/deps/spiral/lib/spiral/lib.ps1


$projectName = "tapas2_contract"

if (!$fast -and !$SkipNotebook) {
    { . deps/spiral/workspace/target/release/spiral$(_exe) dib --path "$ScriptDir/$projectName.dib" --retries $($fast -or !$env:CI ? 1 : 3) } | Invoke-Block -Location ../deps/polyglot
}

{ . ../deps/polyglot/apps/parser/dist/DibParser$(_exe) "$projectName.dib" spi } | Invoke-Block

{ . ../deps/polyglot/apps/spiral/dist/Supervisor$(_exe) --build-file "$projectName.spi" "$projectName.fsx" --timeout 180000 } | Invoke-Block

$runtime = $fast -or $env:CI ? @("--runtime", ($IsWindows ? "win-x64" : "linux-x64")) : @()
$builderArgs = @("$projectName.fsx", $runtime, "--packages", "Fable.Core", "--modules", @(GetFsxModules), "lib/fsharp/Common.fs")
{ . ../deps/polyglot/apps/builder/dist/Builder$(_exe) @builderArgs } | Invoke-Block

$targetDir = GetTargetDir $projectName

{ BuildFable $targetDir $projectName "rs" "CONTRACT" } | Invoke-Block

$path = "$targetDir/$projectName.rs"
if (!($path | Test-Path)) {
    $path = "$targetDir/target/rs/$projectName.rs"
}
if (!($path | Test-Path)) {
    $path = "$targetDir/target/rs/polyglot/target/Builder/$projectName/$projectName.rs"
}
Write-Output "tapas2/contract/build.ps1 / path: $path"
(Get-Content $path) `
    -replace "`"../../../lib", "`"../deps/polyglot/deps/spiral/lib" `
    -replace "`"./lib", "`"../deps/polyglot/lib" `
    -replace "`"../../../../../lib", "`"../deps/polyglot/deps/spiral/lib" `
    -replace "`"../../../../../deps/spiral", "`"../deps/polyglot/deps/spiral" `
    -replace "`"../../../deps", "`"../deps" `
    -replace "`"../../../../../../../../../../../../polyglot", "`"../deps/polyglot" `
    -replace ".fsx`"]", ".rs`"]" `
    -replace ".rs`"]", "_contract.rs`"]" `
    -replace "use fable_library_rust::DateTime_::DateTime;", "type DateTime = ();" `
    | FixRust `
    | Set-Content "$projectName.rs"

cargo fmt --

{ cargo +nightly-2024-07-14 build --release --target wasm32-unknown-unknown } | Invoke-Block -EnvironmentVariables @{ "AUTOMATION" = "False" }
New-Item dist -ItemType Directory -Force | Out-Null
Copy-Item ../target/wasm32-unknown-unknown/release/tapas2_contract.wasm dist/tapas2.wasm -Force

Write-Output "tapas2/contract/build.ps1 / `$targetDir = $targetDir / `$projectName: $projectName / `$env:CI:'$env:CI'"

if ($env:CI) {
    Remove-Item ../../target/wasm32-unknown-unknown/release/deps -Recurse -Force -ErrorAction Ignore
}
