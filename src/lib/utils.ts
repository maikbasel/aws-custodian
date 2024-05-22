import { type ClassValue, clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';

/**
 * Combines multiple class names or class arrays into a single string of class names.
 *
 * @param {...ClassValue[]} inputs - The class names or class arrays to be combined.
 * @return {string} - A string containing the combined class names.
 */
export function cn(...inputs: ClassValue[]): string {
  return twMerge(clsx(inputs));
}
