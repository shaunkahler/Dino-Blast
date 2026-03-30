@echo off
setlocal
echo 🦀 Building project for WebAssembly...
cargo build --target wasm32-unknown-unknown --release
if %errorlevel% neq 0 exit /b %errorlevel%

echo 📁 Copying WASM binary to project root...
copy /Y "target\wasm32-unknown-unknown\release\dino-blast.wasm" "dino_blast.wasm" >nul

echo.
echo Make sure you have run 'gcloud auth login' first!
echo.

set /p PROJECT_ID="Enter your GCP project ID: "
set /p SERVICE_NAME="Enter Cloud Run service name (default: dino-blast): "

if "%SERVICE_NAME%"=="" set SERVICE_NAME=dino-blast

set IMAGE_NAME=gcr.io/%PROJECT_ID%/%SERVICE_NAME%

echo.
echo 📦 Building Docker image locally...
docker build -t %IMAGE_NAME% .
if %errorlevel% neq 0 exit /b %errorlevel%

echo.
echo ☁️  Pushing image to Google Container Registry...
docker push %IMAGE_NAME%
if %errorlevel% neq 0 exit /b %errorlevel%

echo.
echo 🚀 Deploying to Cloud Run...
gcloud run deploy %SERVICE_NAME% ^
    --image %IMAGE_NAME% ^
    --project %PROJECT_ID% ^
    --platform managed ^
    --region us-central1 ^
    --allow-unauthenticated ^
    --memory 256Mi

echo.
echo ✅ Deployment complete!
echo.
echo Your site URL:
gcloud run services describe %SERVICE_NAME% --project %PROJECT_ID% --platform managed --region us-central1 --format "value(status.url)"

pause
