@echo off
cd ..

REM Prüfen, ob Docker oder Podman installiert ist
SET container_runtime=podman
where docker >nul 2>&1 && SET container_runtime=docker

%container_runtime% run --rm -v "%CD%":/usr/src/app -w /usr/src/app/ rust cargo build
%container_runtime% run --rm -v "%CD%":/usr/src/app -w /usr/src/app/frontend_vue node npm install
%container_runtime% run --rm -v "%CD%":/usr/src/app -w /usr/src/app/frontend_vue node npm run build

REM Setze Standard-Ordnernamen, falls kein Parameter übergeben wurde
IF "%1"=="" (
    SET final_folder=release
) ELSE (
    SET final_folder=%1
)

IF NOT EXIST %final_folder% mkdir %final_folder%
xcopy .\wwwroot\ .\%final_folder%\ /E /I /Y
copy .\target\debug\shelly_web.exe .\%final_folder%\shelly_web.exe
copy .\config.json .\%final_folder%\config.json
