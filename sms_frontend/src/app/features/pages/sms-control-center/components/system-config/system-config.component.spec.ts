import { ComponentFixture, TestBed } from '@angular/core/testing';

import { SystemConfigComponent } from './system-config.component';
import { ApiService } from 'src/app/shared/services/api.service';

describe('SystemConfigComponent', () => {
  let component: SystemConfigComponent;
  let fixture: ComponentFixture<SystemConfigComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [SystemConfigComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(SystemConfigComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('form with valid values calls API with correct values', async () => {
    // spy on the postUpdateSenderPool
    spyOn(ApiService, 'postUpdateSenderPool').and.returnValue(Promise.resolve(true));

    // Set valid form values
    component.updateConfigForm.setValue({
      num_senders: 10,
      failure_percent: 50,
      avg_wait_time: 60,
      std_dev_wait_time: 2
    });

    await component.onSubmit();

    expect(ApiService.postUpdateSenderPool).toHaveBeenCalledWith(10, 50, 60, 2);
  });

  it('invalid senders value in form does not call API', async () => {
    // spy on the postUpdateSenderPool (doesn't matter what we return)
    spyOn(ApiService, 'postUpdateSenderPool').and.returnValue(Promise.resolve(true));

    // Set invalid form values
    component.updateConfigForm.setValue({
      num_senders: -1,
      failure_percent: 70,
      avg_wait_time: 60,
      std_dev_wait_time: 2
    });

    await component.onSubmit();

    expect(ApiService.postUpdateSenderPool).not.toHaveBeenCalled();
  });

  it('invalid failure percent value in form does not call API', async () => {
    // spy on the postUpdateSenderPool (doesn't matter what we return)
    spyOn(ApiService, 'postUpdateSenderPool').and.returnValue(Promise.resolve(true));

    // set failure below 0
    component.updateConfigForm.setValue({
      num_senders: 10,
      failure_percent: -1,
      avg_wait_time: 60,
      std_dev_wait_time: 2
    });

    await component.onSubmit();

    expect(ApiService.postUpdateSenderPool).not.toHaveBeenCalled();

    // set failure above 100
    component.updateConfigForm.setValue({
      num_senders: 10,
      failure_percent: 101,
      avg_wait_time: 60,
      std_dev_wait_time: 2
    });

    await component.onSubmit();

    expect(ApiService.postUpdateSenderPool).not.toHaveBeenCalled();
  });


  it('invalid avg wait time value in form does not call API', async () => {
    // spy on the postUpdateSenderPool (doesn't matter what we return)
    spyOn(ApiService, 'postUpdateSenderPool').and.returnValue(Promise.resolve(true));

    // set avg wait time below 0
    component.updateConfigForm.setValue({
      num_senders: 10,
      failure_percent: 70,
      avg_wait_time: -1,
      std_dev_wait_time: 2
    });

    await component.onSubmit();

    expect(ApiService.postUpdateSenderPool).not.toHaveBeenCalled();
  });


  it('invalid std dev wait time value in form does not call API', async () => {
    // spy on the postUpdateSenderPool (doesn't matter what we return)
    spyOn(ApiService, 'postUpdateSenderPool').and.returnValue(Promise.resolve(true));

    // set std dev wait time below 0
    component.updateConfigForm.setValue({
      num_senders: 10,
      failure_percent: 70,
      avg_wait_time: 60,
      std_dev_wait_time: -1
    });

    await component.onSubmit();

    expect(ApiService.postUpdateSenderPool).not.toHaveBeenCalled();
  });

  
});
