# ABE Linear QA Integration - Linear Issues + PR Automation
FROM python:3.11-slim

# Install system dependencies
RUN apt-get update && apt-get install -y \
    git \
    curl \
    build-essential \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Create application directory
WORKDIR /opt/linear-qa

# Copy requirements and install Python dependencies
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

# Copy source code
COPY src/ ./src/
COPY scripts/ ./scripts/
COPY config/ ./config/

# Create necessary directories
RUN mkdir -p /opt/linear-qa/data \
             /opt/linear-qa/results \
             /opt/abe-qa/pr-tasks \
             /opt/linear-qa/logs

# Set permissions
RUN chmod +x scripts/*.sh 2>/dev/null || true

# Expose service port
EXPOSE 18111

# Environment variables
ENV PYTHONPATH=/opt/linear-qa/src
ENV LINEAR_QA_MODE=daemon
ENV LOG_LEVEL=info

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:18111/health || exit 1

# Start the Linear QA Integration Daemon
CMD ["python", "linear_qa_daemon.py"]