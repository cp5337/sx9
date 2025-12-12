# CTAS Task Container: State-Sponsored Financing
# Task ID: uuid-004-001-006
# Full-featured execution environment

FROM kalilinux/kali-rolling:latest

# Install required tools
RUN apt-get update && apt-get install -y \
    nmap \
    python3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy task script
COPY task.sh /app/
RUN chmod +x /app/task.sh

# Task metadata
ENV TASK_ID="uuid-004-001-006"
ENV TASK_NAME="State-Sponsored Financing"
ENV HD4_PHASE="Detect"

ENTRYPOINT ["/app/task.sh"]
