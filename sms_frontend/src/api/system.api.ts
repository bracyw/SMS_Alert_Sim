import { urls } from './urls';

/**
 * Sends a request to update the sender pool used to send messages
 */
export const postNewSenderPool = (body: NewSenderPoolBody): Promise<Response> => {
  return fetch(urls.postNewSenderPool(), {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(body),
  })};


/**
 * Type to represent the body of a sender pool update post request.
 */
export type NewSenderPoolBody = {
  num_senders: number,
  failure_percent: number,
  avg_send_time: number,
  std_dev: number,
};