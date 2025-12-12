import { AES, enc } from 'crypto-js';

const ENCRYPTION_KEY = import.meta.env.VITE_ENCRYPTION_KEY || 'default-key-change-in-production';

export const encryptData = (data: unknown): string => {
  try {
    const stringData = JSON.stringify(data);
    return AES.encrypt(stringData, ENCRYPTION_KEY).toString();
  } catch (error) {
    console.error('Encryption error:', error);
    throw new Error('Failed to encrypt data');
  }
};

export const decryptData = (encryptedData: string): unknown => {
  try {
    const bytes = AES.decrypt(encryptedData, ENCRYPTION_KEY);
    const decryptedString = bytes.toString(enc.Utf8);
    return JSON.parse(decryptedString);
  } catch (error) {
    console.error('Decryption error:', error);
    throw new Error('Failed to decrypt data');
  }
};