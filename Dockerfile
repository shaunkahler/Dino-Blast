FROM python:3.11-slim

WORKDIR /app

COPY index.html .
COPY dino_blast.wasm .
COPY mq_js_bundle.js .
COPY serve.py .

# Optional assets
COPY dinoblast.png .
COPY stagemusic.ogg .
COPY high_scores.txt .

EXPOSE 8080

CMD ["python", "serve.py"]
