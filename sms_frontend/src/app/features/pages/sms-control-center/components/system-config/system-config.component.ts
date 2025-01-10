import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup, Validators, ReactiveFormsModule} from '@angular/forms';
import { NewSenderPoolBody, postNewSenderPool } from 'src/app/shared/utils/api/system-api.utils';
import { CustomValidators } from 'src/app/shared/validators/custom-validators';
import { InfoBackgroundComponent } from 'src/app/shared/components/info-background/info-background.component';
import { ApiService } from 'src/app/shared/services/api.service';




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
        validators: [Validators.required, Validators.min(1)],
        nonNullable: true,
      }),
      failure_percent: new FormControl(50, {
        validators: [Validators.required, CustomValidators.validatePercentage()],
        nonNullable: true,
      }),
      avg_wait_time: new FormControl(1, {
        validators: [Validators.required, Validators.min(0)],
        nonNullable: true,
      }),
      std_dev_wait_time: new FormControl(1, {
        validators: [Validators.required, Validators.min(0)],
        nonNullable: true,
      }),
    });
  }

  async onSubmit() {
    this.isSubmitting = true;
    if(this.updateConfigForm.valid) {
      const numSenders = this.updateConfigForm.controls.num_senders.value
      const failurePercent = this.updateConfigForm.controls.failure_percent.value
      const avgWaitTime = this.updateConfigForm.controls.avg_wait_time.value
      const stdDev = this.updateConfigForm.controls.std_dev_wait_time.value

      // currently we are not worried about alerting the user if the request fails
      // the function will log an error in the console though
      await ApiService.postUpdateSenderPool(numSenders, failurePercent, avgWaitTime, stdDev);
    } 
    this.isSubmitting = false
  }
}
