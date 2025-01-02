import { urls } from './urls';

/**
 * Fetches historical data from the server.
 * @returns A promise containing the response from the server
 */
export const getAllHistoryData = (): Promise<Response> => {
  return fetch(urls.getAllHistoryData());
}