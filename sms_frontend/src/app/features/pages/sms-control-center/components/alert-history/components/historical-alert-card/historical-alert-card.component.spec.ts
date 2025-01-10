import { ComponentFixture, TestBed } from '@angular/core/testing';

import { HistoricalAlertCardComponent } from './historical-alert-card.component';
import { By } from '@angular/platform-browser';
import { DebugElement } from '@angular/core';
import { hasSingleOccurrence } from 'testing/test_helper_functions';


describe('HistoricalAlertCardComponent', () => {
  let component: HistoricalAlertCardComponent;
  let fixture: ComponentFixture<HistoricalAlertCardComponent>;
  const findTypographyComponents = (fixture: ComponentFixture<HistoricalAlertCardComponent>): DebugElement[] => {
    return fixture.debugElement.queryAll(By.css('typography'));
  }

  const findFailureRateTypography = (fixture: ComponentFixture<HistoricalAlertCardComponent>): DebugElement => {
    return findTypographyComponents(fixture).find(el => el.nativeElement.textContent.includes('failure rate:'))!;
  }

  const findMessageSentTypography = (fixture: ComponentFixture<HistoricalAlertCardComponent>): DebugElement => {
    return findTypographyComponents(fixture).find(el => el.nativeElement.textContent.includes('message sent:'))!;
  }

  const findSendAmountTypography = (fixture: ComponentFixture<HistoricalAlertCardComponent>): DebugElement => {
    return findTypographyComponents(fixture).find(el => el.nativeElement.textContent.includes('send amount request:'))!;
  }

  const findMessagesSentTypography = (fixture: ComponentFixture<HistoricalAlertCardComponent>): DebugElement => {
    return findTypographyComponents(fixture).find(el => el.nativeElement.textContent.includes('total time to send:'))!;
  }


  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [HistoricalAlertCardComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(HistoricalAlertCardComponent);
    component = fixture.componentInstance;

    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('failure rate is correctly displayed', () => {
    component.alert = {
      message: "Testing",
      send_amount_requested: 100,
      messages_sent: 50,
      messages_failed: 50,
      total_time_to_send: 23
    };
    fixture.detectChanges();
    expect(hasSingleOccurrence(findFailureRateTypography(fixture), '50')).toBeTrue();
  });

  it('message sent is correctly displayed', () => {
    component.alert = {
      message: "Testing",
      send_amount_requested: 100,
      messages_sent: 50,
      messages_failed: 50,
      total_time_to_send: 23
    };
    fixture.detectChanges();
    expect(hasSingleOccurrence(findMessageSentTypography(fixture), 'Testing')).toBeTrue();
  });


  it('send amount requested is correctly displayed', () => {
    component.alert = {
      message: "Testing",
      send_amount_requested: 100,
      messages_sent: 50,
      messages_failed: 50,
      total_time_to_send: 23
    };
    fixture.detectChanges();
    expect(hasSingleOccurrence(findSendAmountTypography(fixture), '100')).toBeTrue();
  });

  it('total time to send is correctly displayed', () => {
    component.alert = {
      message: "Testing",
      send_amount_requested: 100,
      messages_sent: 50,
      messages_failed: 50,
      total_time_to_send: 23
    };
    fixture.detectChanges();
    expect(hasSingleOccurrence(findMessagesSentTypography(fixture), '23')).toBeTrue();
  });
});
