param(
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. ../deps/polyglot/scripts/core.ps1


function CheckToml {
    param (
        [Parameter(Position = 0, Mandatory)]
        [string] $toml,

        [Parameter(Position = 1, ValueFromRemainingArguments)]
        [Object[]]$_args
    )
    $toml = [IO.Path]::GetFullPath("$ScriptDir/$toml")
    Write-Output "`nCheckToml / toml: $toml / _args: $_args"
    { cargo outdated -m $toml @_args } | Invoke-Block -OnError Continue
}

function CheckJson {
    param (
        [string] $json
    )
    $json = [IO.Path]::GetFullPath("$ScriptDir/$json").Replace("\", "/")
    Write-Output "`nCheckJson / json: $json"
    { . $(Search-Command bun) --bun --cwd $json outdated-pre } | Invoke-Block -OnError Continue
}


CheckToml "../Cargo.toml" `-w

CheckToml "../contract/Cargo.toml"
