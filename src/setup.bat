@echo off

echo 1. Generates or overwrites the current config paths
setlocal ENABLEDELAYEDEXPANSION
set word=/
set js=%appdata%\..\Local\Packages\Microsoft.WindowsTerminal_8wekyb3d8bbwe\LocalState\
set assets=%appdata%\..\Local\Packages\Microsoft.WindowsTerminal_8wekyb3d8bbwe\RoamingState
set js=%js:\=!word!%
set assets=%assets:\=!word!%
@echo { > "%~dp0config.json"
@echo     "settings_folder_path":"%js%", >> "%~dp0config.json"
@echo     "asset_path":"%assets%", >> "%~dp0config.json"
@echo     "questionColor":"lightgray", >> "%~dp0config.json"
@echo     "errorColor":"red", >> "%~dp0config.json"
@echo     "dialog_lines":-1 >> "%~dp0config.json"
@echo } >> "%~dp0config.json"

echo.
echo SUCCESS: Path was configured.
echo.

echo 2. Doing backup of current settings.json as settings.json.pre.wte.backup
copy /y "%js%settings.json" "%js%settings.json.pre.wte.backup"

echo.
echo 3. Set wte as an environment variable

echo $desired_entry = Get-Location;$old_path = [Environment]::GetEnvironmentVariable('path', 'machine');$old_path_entry_list = ($old_path).split(";");$new_path_entry_list = new-object system.collections.arraylist;foreach($old_path_entry in $old_path_entry_list){if($old_path_entry -eq $desired_entry){}else{[void]$new_path_entry_list.Add($old_path_entry)}}[void]$new_path_entry_list.Add($desired_entry);$new_path = $new_path_entry_list -Join ";";[Environment]::SetEnvironmentVariable('path', $new_path,'Machine');> "%~dp0envSetup01.ps1"
powershell -command "Start-Process PowerShell -Verb RunAs \""-Command `\""cd '%cd%'; & './envSetup01.ps1';`\""\"""
timeout /t 02
del "envSetup01.ps1"
echo.


::echo 4. Installing libraries 
::echo.
::cmd /C "npm install"

::echo SUCCESS: Libraries was installed.
::echo.

echo 4. Create batfile
echo.
echo ^@echo off ^& cmd /C "cd %cd%\..\ & cargo run" > "wte.bat"

echo Complete! Restart your terminal and then type "wte" to run the application
pause
