import { ComponentFixture, TestBed } from '@angular/core/testing';
import { ProgressMonitorComponent } from './progress-monitor.component';
import { ReactiveFormsModule } from '@angular/forms';

describe('ProgressMonitorComponent', () => {
  let component: ProgressMonitorComponent;
  let fixture: ComponentFixture<ProgressMonitorComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ProgressMonitorComponent, ReactiveFormsModule], // Import ReactiveFormsModule
    }).compileComponents();

    fixture = TestBed.createComponent(ProgressMonitorComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  /* AI Generated */
  it('should update the poll timing via the form', () => {
    // Access the form control
    const updatesEveryControl = component.progressForm.get('updatesEvery');
    expect(updatesEveryControl).toBeTruthy();

    // Set a new value to the form control
    updatesEveryControl?.setValue(10);
    component.submitForm(); // Submit the form
    fixture.detectChanges();

    // Verify that `updatedEvery` is updated
    expect(component.updatedEvery).toBe(10);
  });

  it('should not update the poll timing for invalid form values', () => {
    // Access the form control
    const updatesEveryControl = component.progressForm.get('updatesEvery');
    expect(updatesEveryControl).toBeTruthy();

    // Set an invalid value
    updatesEveryControl?.setValue(0); // Invalid because of `Validators.min(1)`
    component.submitForm(); // Submit the form
    fixture.detectChanges();

    // Verify that `updatedEvery` remains the default
    expect(component.updatedEvery).toBe(5); // Default value
  });
});
