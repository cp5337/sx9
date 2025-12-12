#!/usr/bin/env node

import https from 'https';
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// Read token from .env file
const envPath = path.join(__dirname, '..', '.env');
const envContent = fs.readFileSync(envPath, 'utf8');
const tokenMatch = envContent.match(/VITE_MAPBOX_ACCESS_TOKEN=(.+)/);

if (!tokenMatch) {
  console.error('❌ No Mapbox token found in .env file');
  process.exit(1);
}

const token = tokenMatch[1].trim();
console.log('🔍 Testing Mapbox token...');
console.log(`Token: ${token.substring(0, 20)}...`);

// Test the token with a simple API call
const testUrl = `https://api.mapbox.com/geocoding/v5/mapbox.places/test.json?access_token=${token}`;

https.get(testUrl, (res) => {
  let data = '';
  
  res.on('data', (chunk) => {
    data += chunk;
  });
  
  res.on('end', () => {
    try {
      const response = JSON.parse(data);
      
      if (res.statusCode === 200) {
        console.log('✅ Token is valid!');
        console.log('Response:', response);
      } else if (res.statusCode === 401) {
        console.log('❌ Token is invalid (401 Unauthorized)');
        console.log('Response:', response);
      } else if (res.statusCode === 403) {
        console.log('❌ Token is forbidden (403 Forbidden)');
        console.log('Response:', response);
      } else {
        console.log(`⚠️  Unexpected status: ${res.statusCode}`);
        console.log('Response:', response);
      }
    } catch (error) {
      console.log('❌ Failed to parse response:', error.message);
      console.log('Raw response:', data);
    }
  });
}).on('error', (error) => {
  console.error('❌ Network error:', error.message);
});
