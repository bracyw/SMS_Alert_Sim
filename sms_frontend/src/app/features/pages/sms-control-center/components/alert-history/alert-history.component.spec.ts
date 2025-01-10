import { ComponentFixture, TestBed } from '@angular/core/testing';
import { AlertHistoryComponent } from './alert-history.component';
import { Alert } from 'src/app/shared/utils/api/types/recieve/alert.type';
import { ApiService } from 'src/app/shared/services/api.service';
import { DebugElement } from '@angular/core';
import { By } from '@angular/platform-browser';
import { HistoricalAlertCardComponent } from './components/historical-alert-card/historical-alert-card.component';

describe('AlertHistoryComponent', () => {
  let component: AlertHistoryComponent;
  let fixture: ComponentFixture<AlertHistoryComponent>;
  let getAlertspy: jasmine.Spy;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [AlertHistoryComponent]
    })
    .compileComponents();

    getAlertspy = spyOn(ApiService, 'getAlertHistory');

    const fakeAlertHistory: Alert[] = [
    ];

    // Mock the ApiService.getAlertHistory() method (as it is it called when the component is created)
    // default value is an empty list... if you want a DIFFERENT RETURN value you can simply call the spy 
    // and return a different value just like the line below
    getAlertspy.and.returnValue(Promise.resolve(fakeAlertHistory));

    fixture = TestBed.createComponent(AlertHistoryComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('alert history should contain data from alert history api', async () => {
    const fakeAlertHistory: Alert[] = [
      {
        message: null,
        send_amount_requested: 100,
        messages_sent: 0,
        messages_failed: 0, 
        total_time_to_send: 0
      },
      {
        message: "hello world",
        send_amount_requested: 100,
        messages_sent: 100,
        messages_failed: 50,
        total_time_to_send: 0
      }
    ];

    // Change the mock to return actual data
    getAlertspy.and.returnValue(Promise.resolve(fakeAlertHistory))

    // Call the updateAlertHistory method
    await component.updateAlertHistory();

    // Ensure the alert history matches the mocked data
    expect(component.alertHistory).toEqual(fakeAlertHistory);
  });


  it('displays the correct amount of alert history elements', async () => {
    const fakeAlertHistory: Alert[] = [
      {
        message: null,
        send_amount_requested: 100,
        messages_sent: 0,
        messages_failed: 0, 
        total_time_to_send: 0
      },
      {
        message: "hello world",
        send_amount_requested: 100,
        messages_sent: 100,
        messages_failed: 50,
        total_time_to_send: 0
      }
    ];

    // Change the mock to return actual data
    getAlertspy.and.returnValue(Promise.resolve(fakeAlertHistory))

    // Call the updateAlertHistory method
    await component.updateAlertHistory();

    fixture.detectChanges();

    // Ensure the correct amount of data is displayed
    const alertHistoryElements: DebugElement[] = fixture.debugElement.queryAll(By.css('historical-alert-card'));
    expect(alertHistoryElements.length).toEqual(fakeAlertHistory.length);
  });


  /* partial AI generation ... little sketchy */
  it('passes the correct alert to the historical-alert-card component', async () => {
    const fakeAlertHistory: Alert[] = [
      {
        message: null,
        send_amount_requested: 100,
        messages_sent: 0,
        messages_failed: 0, 
        total_time_to_send: 0
      },
      {
        message: "hello world",
        send_amount_requested: 100,
        messages_sent: 100,
        messages_failed: 50,
        total_time_to_send: 0
      }
    ];

    // Change the mock to return actual data
    getAlertspy.and.returnValue(Promise.resolve(fakeAlertHistory))

    // Call the updateAlertHistory method
    await component.updateAlertHistory();

    fixture.detectChanges();

    // Ensure the correct amount of data is displayed
    const alertHistoryElements: DebugElement[] = fixture.debugElement.queryAll(By.css('historical-alert-card'));
    expect(alertHistoryElements.length).toEqual(fakeAlertHistory.length);

    // Ensure the correct alert is passed to the historical-alert-card component
    alertHistoryElements.forEach((alertHistoryElement, index) => {
      const historicalAlertCardComponent = alertHistoryElement.componentInstance as HistoricalAlertCardComponent;
      // surprisingly the below works for checking each value of the alert against each other.
      expect(historicalAlertCardComponent.alert).toEqual(fakeAlertHistory[index]);
    });
  });

});
