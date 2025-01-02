import { AbstractControl, ValidationErrors, ValidatorFn } from '@angular/forms';

export class CustomValidators {
  static validatePercentage(): ValidatorFn {
    return (control: AbstractControl): ValidationErrors | null => {
      const value = control.value;
      if (value < 0) {
        return { negativeFailurePercentage: { value } };
      }
      if (value > 1) {
        return { excessiveFailurePercentage: { value } };
      }
      return null;
    };
  }

  static validateAboveZero(): ValidatorFn {
    return (control: AbstractControl): ValidationErrors | null => {
      const value = control.value;
      if (value < 0) {
        return { invalidSendTimeMean: { value } };
      }
      return null;
    };
  }
}