# ABE External API Service Dockerfile
# MARC 21 Compliant Document Intelligence Platform
# Service-Disabled Veteran-Owned Business

FROM node:18-alpine

# Security: Create non-root user
RUN addgroup -g 1001 -S abe && \
    adduser -S abe -u 1001

# Set working directory
WORKDIR /app

# Copy package files
COPY package*.json ./

# Install dependencies
RUN npm ci --only=production && \
    npm cache clean --force

# Copy application code
COPY server.js ./

# Change ownership to non-root user
RUN chown -R abe:abe /app

# Switch to non-root user
USER abe

# Expose port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD node -e "const http = require('http'); const req = http.request({host: 'localhost', port: 8080, path: '/health', timeout: 2000}, (res) => {process.exit(res.statusCode === 200 ? 0 : 1);}); req.on('error', () => process.exit(1)); req.end();"

# Start application
CMD ["node", "server.js"]