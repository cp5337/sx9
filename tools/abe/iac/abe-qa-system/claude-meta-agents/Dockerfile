# ABE Claude Meta-Agents - PR Automation System
FROM python:3.11-slim

# Install system dependencies
RUN apt-get update && apt-get install -y \
    git \
    curl \
    build-essential \
    libssl-dev \
    pkg-config \
    openssh-client \
    && rm -rf /var/lib/apt/lists/*

# Create application directory
WORKDIR /opt/claude-meta

# Copy requirements and install Python dependencies
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

# Copy source code
COPY src/ ./src/
COPY scripts/ ./scripts/
COPY config/ ./config/
COPY agents/ ./agents/

# Create necessary directories
RUN mkdir -p /opt/claude-meta/data \
             /opt/claude-meta/repos \
             /opt/abe-qa/pr-tasks \
             /opt/claude-meta/logs \
             /opt/claude-meta/temp

# Set permissions
RUN chmod +x scripts/*.sh 2>/dev/null || true

# Configure Git (for PR operations)
RUN git config --global user.name "ABE Claude Meta-Agent" && \
    git config --global user.email "noreply@abe-systems.com"

# Expose service port
EXPOSE 18112

# Environment variables
ENV PYTHONPATH=/opt/claude-meta/src
ENV CLAUDE_META_MODE=daemon
ENV LOG_LEVEL=info

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:18112/health || exit 1

# Start the Claude Meta-Agent Daemon
CMD ["python", "src/claude_meta_agent_daemon.py"]