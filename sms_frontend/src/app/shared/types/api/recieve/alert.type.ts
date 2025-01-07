export interface Alert {
  message: string | null;
  send_amount_requested: number;
  messages_sent: number;
  messages_failed: number;
  total_time_to_send: number;
}

/** used for validating the alert structure after it was created using json */
export function isAlert(data: any): data is Alert {
  return (
    typeof data.message === 'string' || data.message === null &&
    typeof data.send_amount_requested === 'number' &&
    typeof data.messages_sent === 'number' &&
    typeof data.messages_failed === 'number' &&
    typeof data.total_time_to_send === 'number'
  );
}