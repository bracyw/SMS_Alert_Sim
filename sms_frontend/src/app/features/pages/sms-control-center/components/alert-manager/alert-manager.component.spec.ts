import { ComponentFixture, TestBed } from '@angular/core/testing';
import { ReactiveFormsModule } from '@angular/forms';
import { AlertManagerComponent } from './alert-manager.component';
import { ApiService } from 'src/app/shared/services/api.service';

describe('AlertManagerComponent', () => {
  let component: AlertManagerComponent;
  let fixture: ComponentFixture<AlertManagerComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [AlertManagerComponent, ReactiveFormsModule], // Import the standalone component
    }).compileComponents();

    fixture = TestBed.createComponent(AlertManagerComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create the component', () => {
    expect(component).toBeTruthy();
  });

  /* AI Generated */

  it('should submit valid form and call ApiService.createAlert', () => {
    // Spy on the static createAlert method
    spyOn(ApiService, 'createAlert').and.resolveTo(true);

    // Set valid form values
    component.sendStartReuestForm.setValue({
      opt_message: 'Test Message',
      num_messages_to_send: 100,
    });

    // Submit the form
    component.submitForm();

    // Assert the ApiService.createAlert was called with the correct arguments
    expect(ApiService.createAlert).toHaveBeenCalledWith('Test Message', 100);
  });

  it('should not submit invalid form', () => {
    // Spy on the static createAlert method
    spyOn(ApiService, 'createAlert').and.resolveTo(true);

    // Set invalid form values
    component.sendStartReuestForm.setValue({
      opt_message: 'Test Message',
      num_messages_to_send: 0, // Invalid value
    });

    // Submit the form
    component.submitForm();

    // Assert the ApiService.createAlert was not called
    expect(ApiService.createAlert).not.toHaveBeenCalled();
  });

  it('should handle optional message being null', () => {
    // Spy on the static createAlert method
    spyOn(ApiService, 'createAlert').and.resolveTo(true);

    // Set form values with `opt_message` as null
    component.sendStartReuestForm.setValue({
      opt_message: null,
      num_messages_to_send: 500,
    });

    // Submit the form
    component.submitForm();

    // Assert the ApiService.createAlert was called with `null` for `opt_message`
    expect(ApiService.createAlert).toHaveBeenCalledWith(null, 500);
  });
});
