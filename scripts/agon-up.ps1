# agon-up.ps1 — PowerShell wrapper. Calls bash version via Git Bash.
$ErrorActionPreference = "Stop"
$root = Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Path)
bash "$root/scripts/agon-up.sh" @args
