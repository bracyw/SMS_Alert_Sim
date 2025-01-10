import { TestBed } from '@angular/core/testing';

import { ApiService } from './api.service';
import { getProgressData } from 'src/app/shared/utils/api/progress-api.utils';
import { ProgressData } from '../utils/api/types/recieve/progress-data.type';
import { JsonPipe } from '@angular/common';
import { SendCreateBody } from '../utils/api/types/send/send-create-body.type';
import { Alert } from '../utils/api/types/recieve/alert.type';

describe('ApiService', () => {
  let service: ApiService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(ApiService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });


  /* POLL PROGRESS TESTS */
  it('poll progress works with correct json', async () => {
    const fakeProgressData: ProgressData = {
      msgs_sent: 100,
      msgs_failed: 50,
      avg_wait_time: 3,
    };
  
    // Mock the global fetch API 
    // IMPORTANT: (should be alligned with our server)
    spyOn(globalThis, 'fetch').and.returnValue(
      Promise.resolve(
        new Response(JSON.stringify(fakeProgressData), {
          status: 200,
          headers: { 'Content-Type': 'application/json' },
        })
      )
    );
  
    // actually call pollProgress while fetch is mocked
    const response = await ApiService.pollProgress();
  
    // Ensure the response matches the mocked data
    expect(response).toEqual(fakeProgressData);
  });

  it('poll progress fails with incorrect json', async () => {
    const incorrectProgressData = {
      msgseeezy_sent: 100,
      msgs_failed: 50,
      avg_wait_time: 3,
    };
  
    // Mock the global fetch API 
    spyOn(globalThis, 'fetch').and.returnValue(
      Promise.resolve(
        new Response(JSON.stringify(incorrectProgressData), {
          status: 200,
          headers: { 'Content-Type': 'application/json' },
        })
      )
    );
  
    await expectAsync(ApiService.pollProgress()).toBeRejectedWithError("recieved invalid json");
  });

  // ensure poll progress response properly to a failure code response.
  // response json and status do not have to alligned with server
  it('poll progress fails when recieves response which is not "ok"', async () => {
    // Mock the global fetch API 
    spyOn(globalThis, 'fetch').and.returnValue(
      Promise.resolve(
        new Response("doesn't matter", { 
          status: 401,
          headers: { 'Content-Type': 'application/json' },
        })
      )
    );

    await expectAsync(ApiService.pollProgress()).toBeRejectedWithError("Error fetching progress data");
  });

  /* CREATE SEND TESTS */
  it('create send works with string message input', async () => {
    // Mock the global fetch API 
    spyOn(globalThis, 'fetch').and.returnValue(
      Promise.resolve(
        new Response("irrelevant", {
          status: 200,
          headers: { 'Content-Type': 'application/json' },
        })
      )
    );

    const response = await ApiService.createAlert("test", 60);
    expect(response).toEqual(true);
  });


  it('create send works with null message input', async () => {
    // Mock the global fetch API 
    spyOn(globalThis, 'fetch').and.returnValue(
      Promise.resolve(
        new Response("irrelevant", {
          status: 200,
          headers: { 'Content-Type': 'application/json' },
        })
      )
    );

    const response = await ApiService.createAlert(null, 60);
    expect(response).toEqual(true);
  });


  // this should not require a fetch mock
  it('create send fails with zero send amount input', async () => {
    // Mock the global fetch API 
    spyOn(globalThis, 'fetch').and.returnValue(
      Promise.resolve(
        new Response("irrelevant", {
          status: 200,
          headers: { 'Content-Type': 'application/json' },
        })
      )
    );

    await expectAsync(ApiService.createAlert("test", 0)).toBeRejectedWithError("Number of messages to send must be greater than 0");
  });

  // this should not require a fetch mock
  it('create send fails with negative send amount input', async () => {
    await expectAsync(ApiService.createAlert("test", 0)).toBeRejectedWithError("Number of messages to send must be greater than 0");
  });

  it('create send returns false when fetch fails', async () => {
    // Mock the global fetch API 
    spyOn(globalThis, 'fetch').and.returnValue(
      Promise.resolve(
        new Response("irrelevant", {
          status: 401,
          headers: { 'Content-Type': 'application/json' },
        })
      )
    );

    const response = await ApiService.createAlert("test", 60);
    expect(response).toEqual(false);
  });

  it('create send calls fetch with correct body', async () => {
    const fakeSendCreateBody: SendCreateBody = {
      msg: "test",
      num_msgs: 200,
    }

    // Mock the global fetch API
    let fetchSpy = spyOn(globalThis, 'fetch');

    // call the function
    try {
      await ApiService.createAlert("test", 200);
    } catch (error) {
      // error is irrelevant and should be tested elsewhere
    }

    // Ensure fetch was called with the correct body 
    // (actual url is up to api utils)
    expect(fetchSpy).toHaveBeenCalledWith(jasmine.any(String), {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(fakeSendCreateBody),
    });

  });


  /* GET ALERT HISTORY TESTS */
  it('get alert history works with correct json', async () => {
    /*
      message: string | null;
  send_amount_requested: number;
  messages_sent: number;
  messages_failed: number;
  total_time_to_send: number;
    */
    const fakeAlertHistory: Alert[] = [
      {
        message: "test",
        send_amount_requested: 100,
        messages_sent: 50,
        messages_failed: 50,
        total_time_to_send: 3,
      },
      {
        message: null,
        send_amount_requested: 200,
        messages_sent: 200,
        messages_failed: 0,
        total_time_to_send: 5,
      }
    ];

    // Mock the global fetch API 
    spyOn(globalThis, 'fetch').and.returnValue(
      Promise.resolve(
        new Response(JSON.stringify(fakeAlertHistory), {
          status: 200,
          headers: { 'Content-Type': 'application/json' },
        })
      )
    );

    const response = await ApiService.getAlertHistory();
    expect(response).toEqual(fakeAlertHistory);
  });


  it('get alert history fails with incorrect json', async () => {
    const incorrectAlertHistory = [
      {
        haha_WRONG_NAME: "test",
        send_amount_requested: 100,
        messages_sent: 50,
        messages_failed: 50,
        total_time_to_send: 3,
      },
      {
        message: null,
        send_amount_requested: 200,
        messages_sent: 200,
        messages_failed: 0,
        total_time_to_send: 5,
      }
    ];

    // Mock the global fetch API 
    spyOn(globalThis, 'fetch').and.returnValue(
      Promise.resolve(
        new Response(JSON.stringify(incorrectAlertHistory), {
          status: 200,
          headers: { 'Content-Type': 'application/json' },
        })
      )
    );

    await expectAsync(ApiService.getAlertHistory()).toBeRejectedWithError("recieved invalid json");
  });


  it('get alert history fails when recieves response which is not "ok"', async () => {
    // Mock the global fetch API 
    spyOn(globalThis, 'fetch').and.returnValue(
      Promise.resolve(
        new Response("doesn't matter", { 
          status: 401,
          headers: { 'Content-Type': 'application/json' },
        })
      )
    );

    await expectAsync(ApiService.getAlertHistory()).toBeRejectedWithError("Error fetching alert history data");
  });



  /* POST UPDATE SENDER POOL TESTS */
  it('post update sender pool works with correct input', async () => {
    // Mock the global fetch API 
    spyOn(globalThis, 'fetch').and.returnValue(
      Promise.resolve(
        new Response("irrelevant", {
          status: 200,
          headers: { 'Content-Type': 'application/json' },
        })
      )
    );

    const response = await ApiService.postUpdateSenderPool(10, 100, 60, 2);
    expect(response).toEqual(true);
  });

  it('post update sender pool fails with 0 senders input', async () => {
    await expectAsync(ApiService.postUpdateSenderPool(0, 50, 60, 2)).toBeRejectedWithError("Invalid input values");
  });

  it('post update sender pool fails with negative failure percent input', async () => {
    await expectAsync(ApiService.postUpdateSenderPool(10, -1, 60, 2)).toBeRejectedWithError("Invalid input values");
  });

  it('post update sender pool fails with failure percent above 100', async () => {
    await expectAsync(ApiService.postUpdateSenderPool(10, 101, 60, 2)).toBeRejectedWithError("Invalid input values");
  });

  it('post update sender pool fails with negative avg wait time input', async () => {
    await expectAsync(ApiService.postUpdateSenderPool(10, 50, -1, 2)).toBeRejectedWithError("Invalid input values");
  });

  it('post update sender pool fails with negative std dev input', async () => {
    await expectAsync(ApiService.postUpdateSenderPool(10, 50, 60, -1)).toBeRejectedWithError("Invalid input values");
  });

});

