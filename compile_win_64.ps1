function Write-BoldMessage {
    param (
        [string]$Message,
        [string]$Color
    )
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    Write-Host "[ $timestamp ] $Message" -ForegroundColor $Color
    Write-Host "`n"
}

if (Test-Path gmsv_ntr_win32.dll)
{
    Write-BoldMessage "Removing gmsv_ntr_win32.dll" "Red"
    rm gmsv_ntr_win32.dll
}

Write-BoldMessage "Building gmsv_ntr_win32.dll" "DarkCyan"
cargo build --release --target i686-pc-windows-msvc

if (Test-Path target/i686-pc-windows-msvc/release/gmsv_ntr.dll)
{
    Write-BoldMessage "Renaming gmsv_ntr.dll to gmsv_ntr_win32.dll" "Yellow"
    mv target/i686-pc-windows-msvc/release/gmsv_ntr.dll gmsv_ntr_win32.dll
    Write-BoldMessage "gmsv_ntr_win32.dll built successfully" "Green"
}