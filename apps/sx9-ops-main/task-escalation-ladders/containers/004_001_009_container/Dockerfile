# CTAS Task Container: Bulk Cash Smuggling
# Task ID: uuid-004-001-009
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
ENV TASK_ID="uuid-004-001-009"
ENV TASK_NAME="Bulk Cash Smuggling"
ENV HD4_PHASE="Detect"

ENTRYPOINT ["/app/task.sh"]
