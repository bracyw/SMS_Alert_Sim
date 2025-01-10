import { SendCreateBody } from 'src/app/shared/utils/api/types/send/send-create-body.type';
import { urls } from './urls.utils';

/**
 * Sends a request to create messages with all configurations.
 * @param body - The request payload to send to the server
 * @returns A promise containing the response from the server
 */
export const postSendCreate = (body: SendCreateBody): Promise<Response> => {
  console.log('send requested with body: ' + JSON.stringify(body));
  return fetch(urls.postSendCreateURL(), {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(body),
  });
};