# CTAS-7 Kali Tools Integration v7.3.1
# Exposes 165 CTAS tasks via REST API with escalation ladder

FROM kalilinux/kali-rolling:latest

# Install essential tools
RUN apt-get update && apt-get install -y \
    nmap \
    curl \
    wget \
    python3 \
    python3-pip \
    netcat-openbsd \
    && rm -rf /var/lib/apt/lists/*

# Install Python dependencies
RUN pip3 install --no-cache-dir \
    flask \
    requests

WORKDIR /app

# Copy task API server
COPY task-server.py .
COPY tasks.json .

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:15178/health || exit 1

EXPOSE 15178

CMD ["python3", "task-server.py"]

