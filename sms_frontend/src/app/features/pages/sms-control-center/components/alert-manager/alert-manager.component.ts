import { Component, OnInit } from '@angular/core';
import {
  Validators,
  FormGroup,
  FormControl,
  ReactiveFormsModule,
} from "@angular/forms";
import { postSendCreate, SendCreateBody } from 'src/api/sending.api';
import { InfoBackgroundComponent } from 'src/app/shared/components/info-background/info-background.component';



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


export class BroadcastManagerComponent implements OnInit {
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

  submitForm(): void {
    this.isSubmitting = true;

    if (this.sendStartReuestForm.valid) {
      // Handle form submission
      const sendCreateBody: SendCreateBody = {
        msg: this.sendStartReuestForm.value.opt_message as null,
        num_msgs: this.sendStartReuestForm.value.num_messages_to_send as number,
      };

      postSendCreate(sendCreateBody).then((response) => {
        if (response.ok) {
          this.isSubmitting = false;
          // Handle success
          
        } else {
          this.isSubmitting = false;
          // Handle server errors
        }
      });
    } else {
      this.isSubmitting = false;
      // Handle form errors
    }
  }
}
