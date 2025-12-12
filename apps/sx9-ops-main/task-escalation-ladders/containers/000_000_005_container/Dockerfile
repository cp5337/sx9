# CTAS Task Container: Technical Training
# Task ID: uuid-000-000-005
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
ENV TASK_ID="uuid-000-000-005"
ENV TASK_NAME="Technical Training"
ENV HD4_PHASE="Hunt"

ENTRYPOINT ["/app/task.sh"]
