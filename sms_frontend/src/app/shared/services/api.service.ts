import { Injectable } from '@angular/core';
import { getAllHistoryData } from 'src/app/shared/utils/api/history-api.utils';
import { Alert, isAlert } from '../utils/api/types/recieve/alert.type';
import { SendCreateBody } from '../utils/api/types/send/send-create-body.type';
import { postSendCreate } from 'src/app/shared/utils/api/sending-api.utils';
import { isProgressData, ProgressData } from '../utils/api/types/recieve/progress-data.type';
import { getProgressData } from 'src/app/shared/utils/api/progress-api.utils';
import { NewSenderPoolBody, postNewSenderPool } from '../utils/api/system-api.utils';

@Injectable({
  providedIn: 'root'
})
export class ApiService {

  constructor() { }


  /**
   * Polls the progress data of all Alerts current and past alerts run while the backend has been
   * See the {@link ProgressData} type for more information on the structure of a ProgressData object.
   * 
   * @throws An error if the progress data could not be fetched
   * @returns the {@link ProgressData} object containing the progress data
   */
  static async pollProgress(): Promise<ProgressData> {
      const response = await getProgressData();
      if (response.ok) {
        const progressData: ProgressData = await response.json();
        if(isProgressData(progressData)) {
          return progressData;
        } else {
          throw new Error("recieved invalid json");
        }
      }
      throw new Error("Error fetching progress data");
  }
  


  /**
   * Sends a request to create an alert with the given message and number of messages to send.
   * 
   * @param message - The message to send in the alert (can be null if you want to send random messages)
   * @param numMessagesToSend - The number of messages to send in the alert
   * 
   * @throws An error if the there was an unexpected issue sending the request
   * @returns A promise containing a boolen indicating if the request was successful
   */
  static async createAlert(message: string | null, numMessagesToSend: number): Promise<boolean> {
    if (numMessagesToSend <= 0) {
      throw new Error("Number of messages to send must be greater than 0");
    }

    const sendCreateBody: SendCreateBody = {
      msg: message,
      num_msgs: numMessagesToSend,
    };
  
    try {
      const response = await postSendCreate(sendCreateBody);
      return response.ok;
    } catch (error) {
      console.error("Unexpected error creating alert:", error);
      throw new Error("Unexpected error creating alert");
    }
  }
  
  
  
  /**
   * Fetches the alert history data from the backend, and returns it as an array of Alert objects.
   * See the {@link Alert} type for more information on the structure of an Alert object.
   * 
   * @throws An error if the alert history data could not be fetched
   * @returns Alert[] | Error the alert history data or an error if the data could not be fetched
   */
  static async getAlertHistory(): Promise<Alert[]> {
    const response = await getAllHistoryData();
    if (response.ok) {
      let alertHistory: Alert[] = await response.json();
      console.log(alertHistory);
      if(alertHistory.every(isAlert)) {
        return alertHistory;
      } else {
        throw new Error("recieved invalid json");
      }
    } else {
        throw new Error("Error fetching alert history data");
    }
  }

  /**
   * Creates a new sender pool with given post body {@link NewSenderPoolBody}.
   * 
   * @param body - The body of the request to create a new sender pool
   * 
   * @throws An error if the there was an unexpected issue sending the request
   * @returns A promise containing a boolen indicating if the request was successful
   */
  static async postUpdateSenderPool(numSenders: number, failurePercent: number, avgSendTime: number, stdDev: number): Promise<boolean> {
    if (numSenders <= 0 || failurePercent < 0 || failurePercent > 100 || avgSendTime <= 0 || stdDev <= 0 ) {
      throw new Error("Invalid input values");
    }
    const body: NewSenderPoolBody = {
      num_senders: numSenders,
      failure_percent: failurePercent,
      avg_send_time: avgSendTime,
      std_dev: stdDev
    }
    try {
      const response = await postNewSenderPool(body);
      return response.ok;
    } catch (error) {
      console.error("Unexpected error creating sender pool:", error);
      throw new Error("Unexpected error creating sender pool");
    }
  }
}