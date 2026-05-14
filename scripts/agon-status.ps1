# agon-status.ps1 — PowerShell wrapper.
$ErrorActionPreference = "Stop"
$root = Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Path)
bash "$root/scripts/agon-status.sh" @args
