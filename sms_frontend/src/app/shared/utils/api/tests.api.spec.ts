import { getProgressData } from 'src/app/shared/utils/api/progress-api.utils';
import { urls } from './urls.utils';
import { getAllHistoryData } from './history-api.utils';
import { SendCreateBody } from './types/send/send-create-body.type';
import { postSendCreate } from './sending-api.utils';
import { NewSenderPoolBody, postNewSenderPool } from './system-api.utils';

describe('Test API Utils', () => {

  it('getAllHistoryData correctly returns a response', async () => {
    let body = "Some random value which could be history data (doesn't matter)";
    // Mock the global fetch API
    spyOn(globalThis, 'fetch').and.returnValue(
      Promise.resolve(
        new Response(body, {
          status: 200,
          headers: { 'Content-Type': 'application/json' },
        })
      )
    );


    const response = await getAllHistoryData();
    expect(response.ok).toEqual(true);
    expect(response.status).toEqual(200);
    await expectAsync(response.text()).toBeResolvedTo(body);
  });


  it('getAllHistoryData calls fetch with the correct url', async () => {
    let spy = spyOn(globalThis, 'fetch').and.returnValue(
      Promise.resolve(
        new Response("doesn't matter", {
          status: 200,
          headers: { 'Content-Type': 'application/json' },
        })
      ));

    await getAllHistoryData();

    expect(spy).toHaveBeenCalledWith(urls.getAllHistoryDataURL());
  });

  it('getProgressData correctly returns a response', async () => {
    let body = "Some random value which could be progress data (doesn't matter)";

    // Mock the global fetch API
    spyOn(globalThis, 'fetch').and.returnValue(
      Promise.resolve(
        new Response(body, {
          status: 200,
          headers: { 'Content-Type': 'application/json' },
        })
      )
    );

    const response = await getProgressData();
    expect(response.ok).toEqual(true);
    expect(response.status).toEqual(200);
    await expectAsync(response.text()).toBeResolvedTo(body);
  });

  it('getProgressData calls fetch with the correct url', async () => {
    let spy = spyOn(globalThis, 'fetch').and.returnValue(
      Promise.resolve(
        new Response("doesn't matter", {
          status: 200,
          headers: { 'Content-Type': 'application/json' },
        })
      ));

    await getProgressData();

    expect(spy).toHaveBeenCalledWith(urls.getProgressDataURL());
  });


  it('postSendCreate correctly returns a response', async () => {
    let body = "Confirmation in some way that the message was sent";
    let sendCreateBody: SendCreateBody = {
      msg: "test",
      num_msgs: 60,
    };

    // Mock the global fetch API
    spyOn(globalThis, 'fetch').and.returnValue(
      Promise.resolve(
        new Response(body, {
          status: 200,
          headers: { 'Content-Type': 'application/json' },
        })
      )
    );

    const response = await postSendCreate(sendCreateBody);
    expect(response.ok).toEqual(true);
    expect(response.status).toEqual(200);
    await expectAsync(response.text()).toBeResolvedTo(body);
  });


  it('postSendCreate calls fetch with the correct url and body', async () => {
    let spy = spyOn(globalThis, 'fetch').and.returnValue(
      Promise.resolve(
        new Response("doesn't matter", {
          status: 200,
          headers: { 'Content-Type': 'application/json' },
        })
      ));

    let sendCreateBody: SendCreateBody = {
      msg: "test",
      num_msgs: 60,
    };

    await postSendCreate(sendCreateBody);

    expect(spy).toHaveBeenCalledWith(urls.postSendCreateURL(), {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(sendCreateBody),
    });
  });

  it('postNewSenderPool returns a response correctly', async () => {
    let body = "Some response confirming the sender pool was updated";
    let newSenderPoolBody: NewSenderPoolBody = {
      num_senders: 1,
      failure_percent: 60,
      avg_send_time: 3,
      std_dev: 2,
    };

    // Mock the global fetch API
    spyOn(globalThis, 'fetch').and.returnValue(
      Promise.resolve(
        new Response(body, {
          status: 200,
          headers: { 'Content-Type': 'application/json' },
        })
      )
    );

    const response = await postNewSenderPool(newSenderPoolBody);
    expect(response.ok).toEqual(true);
    expect(response.status).toEqual(200);
    await expectAsync(response.text()).toBeResolvedTo(body);
  });


  it('postNewSenderPool calls fetch with the correct url and body', async () => {
    let spy = spyOn(globalThis, 'fetch').and.returnValue(
      Promise.resolve(
        new Response("doesn't matter", {
          status: 200,
          headers: { 'Content-Type': 'application/json' },
        })
      ));

    let newSenderPoolBody: NewSenderPoolBody = {
      num_senders: 1000,
      failure_percent: 40,
      avg_send_time: 100,
      std_dev: 5,
    };

    await postNewSenderPool(newSenderPoolBody);

    expect(spy).toHaveBeenCalledWith(urls.postNewSenderPoolURL(), {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(newSenderPoolBody),
    });
  });

});

