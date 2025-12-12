# CTAS Task Container: Destroying Forensic Evidence
# Task ID: uuid-008-008-001
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
ENV TASK_ID="uuid-008-008-001"
ENV TASK_NAME="Destroying Forensic Evidence"
ENV HD4_PHASE="Dominate"

ENTRYPOINT ["/app/task.sh"]
