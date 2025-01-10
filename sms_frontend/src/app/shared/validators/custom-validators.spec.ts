import { FormControl } from '@angular/forms';
import { CustomValidators } from './custom-validators';

/* AI GENERATED */
describe('CustomValidators', () => {
  describe('validatePercentage', () => {
    it('should return an error if the value is less than 0', () => {
      const control = new FormControl(-1);
      const result = CustomValidators.validatePercentage()(control);
      expect(result).toEqual({
        negativeFailurePercentage: { value: -1 },
      });
    });

    it('should return an error if the value is greater than 100', () => {
      const control = new FormControl(101);
      const result = CustomValidators.validatePercentage()(control);
      expect(result).toEqual({
        excessiveFailurePercentage: { value: 101 },
      });
    });

    it('should return null if the value is between 0 and 100 (inclusive)', () => {
      const control = new FormControl(50);
      const result = CustomValidators.validatePercentage()(control);
      expect(result).toBeNull();
    });

    it('should return null for boundary value 0', () => {
      const control = new FormControl(0);
      const result = CustomValidators.validatePercentage()(control);
      expect(result).toBeNull();
    });

    it('should return null for boundary value 100', () => {
      const control = new FormControl(100);
      const result = CustomValidators.validatePercentage()(control);
      expect(result).toBeNull();
    });

    it('should handle null or undefined values gracefully', () => {
      const control = new FormControl(null);
      const result = CustomValidators.validatePercentage()(control);
      expect(result).toBeNull();

      const controlUndefined = new FormControl(undefined);
      const resultUndefined = CustomValidators.validatePercentage()(controlUndefined);
      expect(resultUndefined).toBeNull();
    });

    it('should handle non-numeric values gracefully and return null', () => {
      const control = new FormControl('not-a-number');
      const result = CustomValidators.validatePercentage()(control);
      expect(result).toBeNull(); // Assuming non-numeric values are considered valid
    });
  });
});
