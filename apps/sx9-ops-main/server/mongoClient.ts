import { MongoClient } from 'mongodb';
import { logger } from './logger';

let client: MongoClient | null = null;

export const connectClient = async (): Promise<MongoClient> => {
  if (!client) {
    const uri = process.env.MONGODB_URI || 'mongodb://localhost:27017/';
    logger.info('Attempting to connect to MongoDB...');
    client = new MongoClient(uri);
    await client.connect();
    logger.info('Successfully connected to MongoDB');
  }
  return client;
};

export const getClient = (): MongoClient => {
  if (!client) {
    throw new Error('MongoDB client not initialized');
  }
  return client;
};

export const closeClient = async (): Promise<void> => {
  if (client) {
    await client.close();
    client = null;
    logger.info('MongoDB connection closed');
  }
};
