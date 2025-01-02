import { urls } from './urls';

/**
 * Fetches progress data from the server.
 * @returns A promise containing the response from the server
 */
export const getProgressData = (): Promise<Response> => {
  return fetch(urls.getProgressData());
};
