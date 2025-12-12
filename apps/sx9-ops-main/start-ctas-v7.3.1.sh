#!/bin/bash
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# CTAS Main Ops v7.3.1 - Startup Script
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# Complete production vertical in containers
# Dev agents (PM2) call in via API Gateway (port 18450)
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

set -e

echo "╔════════════════════════════════════════════════════════════════╗"
echo "║                                                                ║"
echo "║   🚀 CTAS Main Ops v7.3.1 - Container Deployment 🚀          ║"
echo "║                                                                ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""

# Check if .env exists
if [ ! -f .env ]; then
    echo "⚠️  .env file not found. Creating from .env.example..."
    cp .env.example .env
    echo "✅ Created .env file. Please update VITE_MAPBOX_TOKEN"
    echo ""
fi

# Check Docker/OrbStack
if ! docker info > /dev/null 2>&1; then
    echo "❌ Docker/OrbStack is not running!"
    echo "   Please start OrbStack and try again."
    exit 1
fi

echo "📦 Building CTAS v7.3.1 Container Stack..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Stop existing containers (if any)
echo "🛑 Stopping existing CTAS containers..."
docker-compose -f docker-compose.ctas-v7.3.1.yml down 2>/dev/null || true
echo ""

# Build images
echo "🏗️  Building custom images..."
docker-compose -f docker-compose.ctas-v7.3.1.yml build
echo ""

# Start the stack
echo "🚀 Starting CTAS v7.3.1 stack..."
docker-compose -f docker-compose.ctas-v7.3.1.yml up -d
echo ""

# Wait for services to initialize
echo "⏳ Waiting for services to initialize (30 seconds)..."
sleep 30
echo ""

# Health check
echo "🔍 Checking service health..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

services=(
    "15174:Frontend"
    "18450:API Gateway"
    "8000:SurrealDB"
    "19014:Sledis"
    "55000:Wazuh"
    "18102:AXON"
    "18106:Legion ECS"
    "11434:Phi-3"
    "18200:HFT Ground Stations"
    "15601:Plasma Dashboard"
    "18300:Kali Tools"
    "18350:Kali Microkernel"
)

for service in "${services[@]}"; do
    port=$(echo $service | cut -d: -f1)
    name=$(echo $service | cut -d: -f2)
    
    if curl -f -s "http://localhost:$port/health" > /dev/null 2>&1 || \
       curl -f -s "http://localhost:$port" > /dev/null 2>&1; then
        echo "✅ $name (Port $port) - HEALTHY"
    else
        echo "⚠️  $name (Port $port) - STARTING..."
    fi
done

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🎯 CTAS Main Ops v7.3.1 - DEPLOYED!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "📊 Access Points:"
echo "   • Frontend:         http://localhost:15174"
echo "   • Frontend (Mirror): http://localhost:25174"
echo "   • API Gateway:      http://localhost:18450"
echo "   • Plasma Dashboard: http://localhost:15601"
echo "   • SurrealDB:        http://localhost:8000"
echo ""
echo "🔌 API Endpoints for Dev Center (PM2 agents):"
echo "   • GET  /health                    - Health check"
echo "   • GET  /api/usims                 - List USIMs (2,309 records)"
echo "   • POST /api/plasma/alert          - Send alert to Plasma"
echo "   • POST /api/kali/execute          - Execute Kali tool"
echo "   • GET  /api/tasks                 - List CTAS tasks (165)"
echo ""
echo "📂 Container Logs:"
echo "   docker-compose -f docker-compose.ctas-v7.3.1.yml logs -f [service]"
echo ""
echo "🛑 Stop Stack:"
echo "   docker-compose -f docker-compose.ctas-v7.3.1.yml down"
echo ""
echo "🚀 Ready for production!"

