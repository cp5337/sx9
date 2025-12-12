# CTAS Task Container: Physical Surveillance
# Task ID: uuid-001-005-001
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
ENV TASK_ID="uuid-001-005-001"
ENV TASK_NAME="Physical Surveillance"
ENV HD4_PHASE="Hunt"

ENTRYPOINT ["/app/task.sh"]
