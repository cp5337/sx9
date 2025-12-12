#!/bin/bash
set -e

# Update and install essential tools
apt-get update
apt-get install -y \
    python3-pip \
    metasploit-framework \
    nmap \
    hydra \
    sqlmap \
    wireshark \
    aircrack-ng \
    john \
    nikto \
    burpsuite \
    exploitdb \
    netcat-traditional \
    proxychains \
    tor \
    python3-impacket

# Install Python API server dependencies
pip3 install flask flask-restx gunicorn

# Start API server
gunicorn --bind 0.0.0.0:8080 /opt/scripts/api.py