# SX9 Tool Exerciser - Kali-based container for cycling through all tools
# Captures outputs in structured format for hash pipeline processing

FROM kalilinux/kali-rolling:latest

LABEL maintainer="SX9 Core Engineering"
LABEL description="Tool exerciser for output harvesting"
LABEL version="1.0.0"

# Avoid prompts during package installation
ENV DEBIAN_FRONTEND=noninteractive
ENV TERM=xterm-256color

# Install base tools and common dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    # Core utilities
    curl wget git jq bc \
    # Network tools (safe tier)
    nmap masscan netdiscover arp-scan fping nbtscan \
    # Web tools
    nikto whatweb wpscan dirb gobuster ffuf \
    # OSINT
    theharvester amass subfinder \
    # Password tools (offline only)
    john hashcat hydra \
    # Vuln scanners
    nuclei \
    # Misc
    whois dnsutils netcat-openbsd \
    # Python for scripting
    python3 python3-pip \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Install additional Python tools
RUN pip3 install --no-cache-dir \
    requests beautifulsoup4 \
    shodan censys \
    && pip3 cache purge

# Create output directories
RUN mkdir -p /sx9/output/{raw,parsed,hashed} \
    && mkdir -p /sx9/config \
    && mkdir -p /sx9/logs

# Copy exerciser scripts
COPY exerciser.sh /sx9/exerciser.sh
COPY tool_profiles.toml /sx9/config/tool_profiles.toml
COPY output_parser.py /sx9/output_parser.py

RUN chmod +x /sx9/exerciser.sh

# Working directory
WORKDIR /sx9

# Default command
ENTRYPOINT ["/sx9/exerciser.sh"]
CMD ["--help"]

