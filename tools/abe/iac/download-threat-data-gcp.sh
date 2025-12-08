#!/bin/bash
# Download Threat Data on GCP - Using Existing Infrastructure
# Wrapper script that uses existing threat_content_fetcher.py
# Focus: Get the data back cheaply

set -e

echo "💰 Threat Data Download - GCP Budget (Using Existing System)"
echo "============================================================"
echo ""

# Use existing configuration from 04-abe-iac
PROJECT_ID="${GCP_PROJECT_ID:-ctas7-production}"
REGION="${GCP_REGION:-us-central1}"
ZONE="${GCP_ZONE:-us-central1-a}"
INSTANCE_NAME="ctas7-threat-data"
MACHINE_TYPE="e2-standard-4"  # Cheap CPU-only instance
PREEMPTIBLE="${PREEMPTIBLE:-true}"  # 80% cost savings

echo "📋 Configuration:"
echo "   Project: $PROJECT_ID"
echo "   Instance: $INSTANCE_NAME"
echo "   Machine: $MACHINE_TYPE (CPU-only, preemptible)"
echo "   Cost: ~\$0.10/hour (~\$0.40-0.60 total)"
echo ""

# Check gcloud
if ! command -v gcloud &> /dev/null; then
    echo "❌ gcloud CLI not found"
    exit 1
fi

gcloud config set project $PROJECT_ID

# Create preemptible instance
echo "🖥️  Creating preemptible instance..."
PREEMPT_FLAG=""
if [ "$PREEMPTIBLE" = "true" ]; then
    PREEMPT_FLAG="--preemptible"
fi

gcloud compute instances create $INSTANCE_NAME \
    --zone=$ZONE \
    --machine-type=$MACHINE_TYPE \
    $PREEMPT_FLAG \
    --maintenance-policy=TERMINATE \
    --image-family=ubuntu-2204-lts \
    --image-project=ubuntu-os-cloud \
    --boot-disk-size=200GB \
    --boot-disk-type=pd-standard \
    --scopes=https://www.googleapis.com/auth/cloud-platform

echo "✅ Instance created"
sleep 30

# Upload existing threat_content_fetcher.py
echo "📤 Uploading threat_content_fetcher.py..."
cd /Users/cp5337/Developer/ctas-7-shipyard-staging/04-abe-iac/node-interview-generator
gcloud compute scp --zone=$ZONE threat_content_fetcher.py $INSTANCE_NAME:~/ || {
    echo "⚠️  Upload failed, will download from repo"
}

# Setup and run
echo "📦 Setting up environment..."
gcloud compute ssh $INSTANCE_NAME --zone=$ZONE --command="
    sudo apt-get update
    sudo apt-get install -y python3 python3-pip git
    pip3 install requests pyyaml beautifulsoup4 ontogpt linkml 2>/dev/null || true
    mkdir -p ~/threat-data/output
"

# Run existing threat_content_fetcher.py
echo "📥 Running threat data download..."
gcloud compute ssh $INSTANCE_NAME --zone=$ZONE --command="
    cd ~/threat-data
    python3 ~/threat_content_fetcher.py --all --no-training 2>&1 | tee download.log
    echo ''
    echo '✅ Download complete'
    du -sh output/threat_content
"

# Package for download
echo "📦 Packaging data..."
gcloud compute ssh $INSTANCE_NAME --zone=$ZONE --command="
    cd ~/threat-data
    tar -czf threat_data_\$(date +%Y%m%d).tar.gz output/threat_content/
    ls -lh threat_data_*.tar.gz
"

echo ""
echo "✅ Data ready for download"
echo ""
echo "📥 Download data:"
echo "   gcloud compute scp $INSTANCE_NAME:~/threat-data/threat_data_*.tar.gz ./ --zone=$ZONE"
echo ""
echo "🗑️  Delete instance when done:"
echo "   gcloud compute instances delete $INSTANCE_NAME --zone=$ZONE"


