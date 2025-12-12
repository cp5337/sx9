import express from 'express';
import cors from 'cors';
import { logger, getLogs } from './logger';
import { connectClient, getClient, closeClient } from './mongoClient';

const app = express();
const port = 3001;

app.use(cors());
app.use(express.json());

app.get('/api/mongodb/logs', async (_req, res) => {
  const logs = await getLogs();
  res.json({ logs });
});

app.post('/api/mongodb/connect', async (_req, res) => {
  try {
    const client = await connectClient();
    const adminDb = client.db('admin');
    const result = await adminDb.command({ ping: 1 });
    logger.info(`MongoDB ping result: ${JSON.stringify(result)}`);
    res.json({ success: true, message: 'Connected to MongoDB' });
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : 'Failed to connect to MongoDB';
    logger.error(errorMessage);
    res.status(500).json({ success: false, error: errorMessage });
  }
});

app.get('/api/mongodb/databases', async (_req, res) => {
  try {
    const client = getClient();
    logger.info('Fetching databases...');
    const adminDb = client.db('admin');
    const databases = await adminDb.admin().listDatabases();
    const dbNames = databases.databases.map(db => db.name);
    logger.info(`Found databases: ${dbNames.join(', ')}`);
    res.json(dbNames);
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : 'Failed to fetch databases';
    logger.error(errorMessage);
    res.status(500).json({ error: errorMessage });
  }
});

app.get('/api/mongodb/collections/:database', async (req, res) => {
  try {
    const client = getClient();
    logger.info(`Fetching collections for database: ${req.params.database}`);
    const db = client.db(req.params.database);
    const collections = await db.listCollections().toArray();
    const collectionNames = collections.map(col => col.name);
    logger.info(`Found collections for ${req.params.database}: ${collectionNames.join(', ')}`);
    res.json(collectionNames);
  } catch (error) {
    const errorMessage =
      error instanceof Error
        ? error.message
        : `Failed to fetch collections for database ${req.params.database}`;
    logger.error(errorMessage);
    res.status(500).json({ error: errorMessage });
  }
});

const gracefulShutdown = async () => {
  logger.info('Received shutdown signal. Shutting down...');
  await closeClient();
  logger.info('Shutdown complete', () => {
    process.exit(0);
  });
};

process.on('SIGINT', gracefulShutdown);
process.on('SIGTERM', gracefulShutdown);

app.listen(port, () => {
  logger.info(`Server running on port ${port}`);
});
