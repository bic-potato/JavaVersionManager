
if ($env:JVS_LEVEL -eq "DEBUG")
{
    $executePath = $PSScriptRoot + "/target/debug/jvmain.exe"
} else {
    $executePath = $PSScriptRoot + "/jvmain.exe"
}


$formatted_args = New-Object System.Collections.ArrayList

for($i=0; $i -le $args.Count; $i++){
    if ($args[$i]){
    $__ = $formatted_args.Add("`"" + $args[$i] +"`"" );
    }
}


$env:JVS_EXEC_SHELL="pwsh"


$env:JVS_POSTSCRIPT = Join-Path $PSScriptRoot ("jvs_tmp_" + (Get-Random -SetSeed $PID) + ".ps1")

$startInfo = New-Object System.Diagnostics.ProcessStartInfo "`"$executePath`"" 
$startInfo.Arguments = ($formatted_args)
$startInfo.UseShellExecute = $false
$startInfo.RedirectStandardOutput = $true
$proc = [System.Diagnostics.Process]::Start($startInfo)
while (($b = $proc.StandardOutput.Read()) -ne -1) {
    Write-Host -NoNewline ([char]$b)
}
$proc.WaitForExit
$exitCode = $proc.ExitCode

if ($env:JVS_POSTSCRIPT -and (Test-Path $env:JVS_POSTSCRIPT)) {
	. $env:JVS_POSTSCRIPT
	Remove-Item -Force $env:JVS_POSTSCRIPT
}

$env:JVS_POSTSCRIPT = $null
exit $exitCode