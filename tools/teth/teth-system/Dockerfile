# TETH - Tool Entropy Testing Harness
# Dockerfile for OrbStack deployment

FROM python:3.11-slim

LABEL maintainer="Charles E. Payne"
LABEL description="TETH - Tool Entropy Testing Harness"

WORKDIR /app

# Install dependencies
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

# Copy source
COPY teth/ ./teth/
COPY main.py .

# Set Python path
ENV PYTHONPATH=/app
ENV PYTHONUNBUFFERED=1

# Default command runs examples
CMD ["python", "main.py"]
