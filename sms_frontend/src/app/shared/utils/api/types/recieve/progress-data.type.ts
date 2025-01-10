export interface ProgressData {
    msgs_sent: number; 
    msgs_failed: number; 
    avg_wait_time: number 
}

/** used for validating the progress data structure after it was created using json */
export function isProgressData(data: any): data is ProgressData {
    return (
      typeof data.msgs_sent === 'number' &&
      typeof data.msgs_failed === 'number' &&
      typeof data.avg_wait_time === 'number'
    );
  }
  