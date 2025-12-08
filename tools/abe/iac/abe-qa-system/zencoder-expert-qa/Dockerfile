# ABE ZenCoder Expert QA - Layer 2 AI-Driven Analysis
# ZenCoder.ai integration with crawl4ai + Playwright + Multi-model AI
FROM mcr.microsoft.com/playwright/python:v1.40.0-jammy

# Install system dependencies
RUN apt-get update && apt-get install -y \
    python3 \
    python3-pip \
    git \
    curl \
    build-essential \
    libssl-dev \
    pkg-config \
    nodejs \
    npm \
    chromium-browser \
    firefox \
    && rm -rf /var/lib/apt/lists/*

# Install Playwright browsers
RUN playwright install --with-deps

# Create application directory
WORKDIR /opt/zencoder-qa

# Copy requirements and install Python dependencies
COPY requirements.txt .
RUN pip3 install --no-cache-dir -r requirements.txt

# Install Node.js dependencies for ZenCoder integration
COPY package.json package-lock.json ./
RUN npm ci --only=production

# Copy source code
COPY src/ ./src/
COPY scripts/ ./scripts/
COPY config/ ./config/
COPY agents/ ./agents/

# Create necessary directories
RUN mkdir -p /opt/zencoder-qa/data \
             /opt/zencoder-qa/results \
             /opt/zencoder-qa/models \
             /opt/zencoder-qa/logs \
             /opt/zencoder-qa/screenshots \
             /opt/zencoder-qa/crawl-cache

# Set permissions
RUN chmod +x scripts/*.sh

# Expose service port
EXPOSE 18110

# Environment variables
ENV PYTHONPATH=/opt/zencoder-qa/src
ENV NODE_PATH=/opt/zencoder-qa/node_modules
ENV ZENCODER_QA_MODE=daemon
ENV PLAYWRIGHT_BROWSERS_PATH=/ms-playwright
ENV LOG_LEVEL=info

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:18110/health || exit 1

# Start the ZenCoder Expert QA Engine
CMD ["python3", "src/zencoder_expert_daemon.py"]