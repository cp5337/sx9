# MCP GLAF Visualizer Server
FROM node:20-slim

WORKDIR /app

# Copy package files
COPY package.json ./
COPY tsconfig.json ./

# Install dependencies
RUN npm install

# Copy source
COPY src ./src

# Build
RUN npm run build

EXPOSE 18051

CMD ["node", "dist/index.js"]


