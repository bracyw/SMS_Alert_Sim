import { Component, OnInit } from '@angular/core';
import {
  Validators,
  FormGroup,
  FormControl,
  ReactiveFormsModule,
} from "@angular/forms";
import { InfoBackgroundComponent } from 'src/app/shared/components/info-background/info-background.component';
import { ApiService } from 'src/app/shared/services/api.service';

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
  errors: { message: string } = { message: "" };
  isSubmitting = false;
  sendStartReuestForm: FormGroup<SendRequestForm>;

  constructor() {
    this.sendStartReuestForm = new FormGroup<SendRequestForm>({
      opt_message: new FormControl(null, {
        validators: [],
      }),
      num_messages_to_send: new FormControl(1000, {
        validators: [Validators.required],
        nonNullable: true,
      })
    });
  }

  ngOnInit(): void {}

  /** Called for submitting send request form
   * 
   * @returns void
   */ 
  submitForm(): void {
    this.isSubmitting = true;

    if (this.sendStartReuestForm.valid) {
      ApiService.createAlert(this.submitForm.arguments.opt_message, this.submitForm.arguments.num_messages_to_send);
      this.isSubmitting = false;
    } else {
      this.isSubmitting = false;
    }
  }
}
