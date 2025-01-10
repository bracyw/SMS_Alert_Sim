import { ComponentFixture, TestBed } from '@angular/core/testing';

import { SmsControlCenterComponent } from './sms-control-center.component';

describe('SystemConfigComponent', () => {
  let component: SmsControlCenterComponent;
  let fixture: ComponentFixture<SmsControlCenterComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [SmsControlCenterComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(SmsControlCenterComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  // as long as this compiles not to worried about testing... also not looking to get into end-to-end testing

});
