#!/bin/bash
set -e

echo "🦀 Building project for WebAssembly..."
cargo build --target wasm32-unknown-unknown --release

echo "📁 Copying WASM binary to project root..."
cp target/wasm32-unknown-unknown/release/dino-blast.wasm ./dino_blast.wasm

echo ""
echo "☁️  Deploying to Cloud Run..."
echo ""
echo "Make sure you have run 'gcloud auth login' first!"
echo ""

read -p "Enter your GCP project ID: " PROJECT_ID
read -p "Enter Cloud Run service name (default: dino-blast): " SERVICE_NAME

SERVICE_NAME=${SERVICE_NAME:-dino-blast}

echo ""
echo "Deploying..."
# Using Cloud Build to build and push the container natively
gcloud run deploy $SERVICE_NAME \
    --source . \
    --project $PROJECT_ID \
    --platform managed \
    --region us-central1 \
    --allow-unauthenticated \
    --memory 256Mi

echo ""
echo "✅ Deployment complete!"
echo ""
echo "Your site URL:"
gcloud run services describe $SERVICE_NAME --project $PROJECT_ID --platform managed --region us-central1 --format "value(status.url)"
