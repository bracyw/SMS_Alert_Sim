import { Component, OnInit } from '@angular/core';
import {
  Validators,
  FormGroup,
  FormControl,
  ReactiveFormsModule,
} from "@angular/forms";
import { from } from 'rxjs';
import { InfoBackgroundComponent } from 'src/app/shared/components/info-background/info-background.component';
import { ApiService } from 'src/app/shared/services/api.service';
import { CustomValidators } from 'src/app/shared/validators/custom-validators';

/**
 * Interface for the send request form.
 */
interface SendRequestForm {
  opt_message: FormControl<string | null>;
  num_messages_to_send: FormControl<number>;
}


@Component({
  selector: 'alert-manager',
  imports: [InfoBackgroundComponent, ReactiveFormsModule],
  templateUrl: './alert-manager.component.html',
  styleUrl: './alert-manager.component.css',
  standalone: true,
})


/**
 * Component for managing the broadcast of alerts.
 */ 
export class AlertManagerComponent implements OnInit {
  isSubmitting = false;
  sendStartReuestForm: FormGroup<SendRequestForm>;

  constructor() {
    this.sendStartReuestForm = new FormGroup<SendRequestForm>({
      opt_message: new FormControl(null, {
        validators: [],
      }),
      num_messages_to_send: new FormControl(1000, {
        validators: [Validators.required, Validators.min(1)],
        nonNullable: true,
      })
    });
  }

  ngOnInit(): void {}

  /** Called for submitting send request form
   * 
   * @returns void
   */ 
  async submitForm(): Promise<void> {
    if (this.sendStartReuestForm.invalid) {
      // this should really never happen because we don't allow the user to click submit unless the form is valid
      // hence the lack of error handling
      this.isSubmitting = false;
      return;
    }
  
    const formInput = this.sendStartReuestForm.value;
    const messageToSend = formInput.opt_message ?? null;
    const numMessagesToSend = formInput.num_messages_to_send!;
  
    await ApiService.createAlert(messageToSend, numMessagesToSend!);
    this.isSubmitting = false;
  }
  
}
