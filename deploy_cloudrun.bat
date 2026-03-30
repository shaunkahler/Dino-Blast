@echo off
setlocal
set PROJECT_ID=project-2dce91ca-5ee8-488d-95c
set SERVICE_NAME=dino-blast
set IMAGE_NAME=gcr.io/%PROJECT_ID%/%SERVICE_NAME%

echo 🦀 Building project for WebAssembly...
cargo build --target wasm32-unknown-unknown --release
if %errorlevel% neq 0 exit /b %errorlevel%

echo 📁 Copying WASM binary to project root...
copy /Y "target\wasm32-unknown-unknown\release\dino-blast.wasm" "dino_blast.wasm" >nul

echo 📦 Building Docker image...
docker build -t %IMAGE_NAME% .
if %errorlevel% neq 0 exit /b %errorlevel%

echo 📤 Pushing Docker image to GCR...
docker push %IMAGE_NAME%
if %errorlevel% neq 0 exit /b %errorlevel%

echo 🚀 Deploying to Cloud Run (%SERVICE_NAME%)...
gcloud run deploy %SERVICE_NAME% ^
    --image %IMAGE_NAME% ^
    --project %PROJECT_ID% ^
    --platform managed ^
    --region us-central1 ^
    --allow-unauthenticated ^
    --memory 256Mi

echo ✅ Deployment complete! URL: https://shaunkahler.com
pause
