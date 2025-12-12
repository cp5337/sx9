import { createLogger, format, transports } from 'winston';
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const logFilePath = path.join(__dirname, 'server.log');

export const logger = createLogger({
  level: 'info',
  format: format.combine(
    format.timestamp(),
    format.printf(({ timestamp, level, message }) => `[${timestamp}] [${level.toUpperCase()}] ${message}`)
  ),
  transports: [
    new transports.File({ filename: logFilePath }),
    new transports.Console()
  ]
});

export const getLogs = async (): Promise<string> => {
  try {
    return await fs.promises.readFile(logFilePath, 'utf8');
  } catch {
    return '';
  }
};
