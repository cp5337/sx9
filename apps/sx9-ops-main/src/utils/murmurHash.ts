import murmurHash from 'murmurhash';

/**
 * murmurHash.ts
 * Utility functions for generating MurmurHash
 * Author: Charlie Payne
 * Date: June 15, 2023
 */

/**
 * Generates a MurmurHash for the given input
 * @param input The input string to hash
 * @returns The generated hash as a number
 */
export function generateHash(input: string): number {
  return murmurHash.v3(input);
}

/**
 * Generates a MurmurHash and returns it as a hexadecimal string
 * @param input The input string to hash
 * @returns The generated hash as a hexadecimal string
 */
export function generateHashHex(input: string): string {
  const hash = generateHash(input);
  return hash.toString(16);
}

// Example usage:
// console.log(generateHash('example input'));
// console.log(generateHashHex('example input'));