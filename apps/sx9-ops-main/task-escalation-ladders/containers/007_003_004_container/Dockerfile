# CTAS Task Container: Comms Tech Sabotage
# Task ID: uuid-007-003-004
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
ENV TASK_ID="uuid-007-003-004"
ENV TASK_NAME="Comms Tech Sabotage"
ENV HD4_PHASE="Disable"

ENTRYPOINT ["/app/task.sh"]
