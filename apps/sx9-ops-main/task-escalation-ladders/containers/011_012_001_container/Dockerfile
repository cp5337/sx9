# CTAS Task Container: Maritime Smuggling
# Task ID: uuid-011-012-001
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
ENV TASK_ID="uuid-011-012-001"
ENV TASK_NAME="Maritime Smuggling"
ENV HD4_PHASE="Dominate"

ENTRYPOINT ["/app/task.sh"]
