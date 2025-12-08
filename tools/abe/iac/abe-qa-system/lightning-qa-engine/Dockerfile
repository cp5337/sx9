# ABE Lightning QA Engine - Layer 1 Script-Only Analysis
# GPU-accelerated, non-invasive analysis with PR automation
FROM nvidia/cuda:12.2-runtime-ubuntu22.04

# Install system dependencies
RUN apt-get update && apt-get install -y \
    python3 \
    python3-pip \
    git \
    curl \
    build-essential \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Install Rust for performance-critical components
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Create application directory
WORKDIR /opt/lightning-qa

# Copy requirements and install Python dependencies
COPY requirements.txt .
RUN pip3 install --no-cache-dir -r requirements.txt

# Copy source code
COPY src/ ./src/
COPY scripts/ ./scripts/
COPY config/ ./config/

# Create necessary directories
RUN mkdir -p /opt/lightning-qa/data \
             /opt/lightning-qa/results \
             /opt/lightning-qa/logs

# Set permissions
RUN chmod +x scripts/*.sh

# Expose service port
EXPOSE 18109

# Environment variables
ENV CUDA_VISIBLE_DEVICES=0
ENV PYTHONPATH=/opt/lightning-qa/src
ENV LIGHTNING_QA_MODE=daemon
ENV LOG_LEVEL=info

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:18109/health || exit 1

# Start the Lightning QA Engine
CMD ["python3", "src/lightning_qa_daemon.py"]