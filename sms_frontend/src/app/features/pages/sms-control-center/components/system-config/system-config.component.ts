import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup, Validators, ReactiveFormsModule} from '@angular/forms';
import { NewSenderPoolBody, postNewSenderPool } from 'src/api/system.api';
import { CustomValidators } from 'src/app/shared/validators/custom-validators';
import { InfoBackgroundComponent } from 'src/app/shared/components/info-background/info-background.component';




interface UpdateConfigForm {
  num_senders: FormControl<number>;
  failure_percent: FormControl<number>;
  avg_wait_time: FormControl<number>;
  std_dev_wait_time: FormControl<number>;
}

@Component({
  selector: 'system-config',
  imports: [ReactiveFormsModule, InfoBackgroundComponent],
  templateUrl: './system-config.component.html',
  styleUrl: './system-config.component.css',
  standalone: true,
})
export class SystemConfigComponent {
  errors: { message: string } = { message: "" };
  isSubmitting = false;
  updateConfigForm: FormGroup<UpdateConfigForm>;
  constructor() {
    this.updateConfigForm = new FormGroup<UpdateConfigForm>({
      num_senders: new FormControl(100, {
        validators: [Validators.required],
        nonNullable: true,
      }),
      failure_percent: new FormControl(50, {
        validators: [Validators.required, CustomValidators.validatePercentage],
        nonNullable: true,
      }),
      avg_wait_time: new FormControl(1, {
        validators: [Validators.required, CustomValidators.validateAboveZero],
        nonNullable: true,
      }),
      std_dev_wait_time: new FormControl(1, {
        validators: [Validators.required, CustomValidators.validateAboveZero],
        nonNullable: true,
      }),
    });
  }

  onSubmit() {
    this.isSubmitting = true;
    if(this.updateConfigForm.valid) {
      // Send request to update config
      const body: NewSenderPoolBody = {
        num_senders: this.updateConfigForm.controls.num_senders.value,
        failure_percent: this.updateConfigForm.controls.failure_percent.value,
        avg_send_time: this.updateConfigForm.controls.avg_wait_time.value,
        std_dev: this.updateConfigForm.controls.std_dev_wait_time.value,
      };
      // Send request to update config with newSenderPool
      postNewSenderPool(body).then((response) => {
        if(response.ok) {
          this.errors.message = "Successfully updated config";
        } else {
          this.errors.message = "Failed to update config";
        }
      }).finally(() => {
        this.isSubmitting = false;
      });
    } else {
      this.errors.message = "Form is invalid";
      this.isSubmitting = false
    }
  }
}
