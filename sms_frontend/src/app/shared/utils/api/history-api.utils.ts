import { urls } from './urls.utils';

/**
 * Fetches historical data from the server.
 * @returns A promise containing the response from the server
 */
export const getAllHistoryData = (): Promise<Response> => {
  return fetch(urls.getAllHistoryDataURL());
}