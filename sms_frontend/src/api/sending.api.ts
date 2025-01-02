import { urls } from './urls';

/**
 * Sends a request to create messages with all configurations.
 * @param body - The request payload to send to the server
 * @returns A promise containing the response from the server
 */
export const postSendCreate = (body: SendCreateBody): Promise<Response> => {
  console.log('send requested with body: ' + JSON.stringify(body));
  return fetch(urls.postSendCreate(), {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(body),
  });
};


/**
 * Type to represent the body of a send creation post request.
 */
export type SendCreateBody = {
    msg: string | null,
    num_msgs: number,
};