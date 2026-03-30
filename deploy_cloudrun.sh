#!/bin/bash
set -e

PROJECT_ID="project-2dce91ca-5ee8-488d-95c"
SERVICE_NAME="dino-blast"
IMAGE_NAME="gcr.io/$PROJECT_ID/$SERVICE_NAME"

echo "🦀 Building project for WebAssembly..."
cargo build --target wasm32-unknown-unknown --release

echo "📁 Copying WASM binary to project root..."
cp target/wasm32-unknown-unknown/release/dino-blast.wasm ./dino_blast.wasm

echo "📦 Building Docker image..."
docker build -t $IMAGE_NAME .

echo "📤 Pushing Docker image to GCR..."
docker push $IMAGE_NAME

echo "🚀 Deploying to Cloud Run ($SERVICE_NAME)..."
gcloud run deploy $SERVICE_NAME \
    --image $IMAGE_NAME \
    --project $PROJECT_ID \
    --platform managed \
    --region us-central1 \
    --allow-unauthenticated \
    --memory 256Mi

echo "✅ Deployment complete! URL: https://shaunkahler.com"
