import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup, Validators, ReactiveFormsModule } from '@angular/forms';
import { InfoBackgroundComponent } from 'src/app/shared/components/info-background/info-background.component';
import { HStackComponent } from 'src/app/shared/components/hstack/hstack.component';
import { DividerComponent } from 'src/app/shared/components/divider/divider';
import { ProgressData } from 'src/app/shared/utils/api/types/recieve/progress-data.type';
import { ApiService } from 'src/app/shared/services/api.service';
import { getPercent } from 'src/app/shared/utils/math.utils';
import { CaptionedValueComponentComponent } from 'src/app/shared/components/captioned-value-component/captioned-value-component.component';
import { Unit } from 'src/app/shared/utils/enums/unit.enum';


@Component({
  selector: 'progress-monitor',
  imports: [
    InfoBackgroundComponent,
    HStackComponent,
    DividerComponent,
    ReactiveFormsModule,
    CaptionedValueComponentComponent
  ],
  templateUrl: './progress-monitor.component.html',
  styleUrls: ['./progress-monitor.component.css'],
  standalone: true,
})

/**
 * Component for used for displaying info about system health and alert info during this uptime.
 */
export class ProgressMonitorComponent implements OnInit {
  percentUnit: Unit = Unit.PERCENT;
  updatedEvery = 5;  // The number of seconds to wait before updating the progress data
  progressData: ProgressData | null = null; // the progress data to display
  private intervalId: any;  // the id of the interval that updates the progress data

  // creates form group
  progressForm = new FormGroup({
    updatesEvery: new FormControl<number>(5, {
      validators: [Validators.required, Validators.min(0)],
      nonNullable: true,
    }),
  });
  isSubmitting = false;  // whether the form is currently submitting

  async ngOnInit(): Promise<void> {
    // try to poll immediately
    this.progressData = await ApiService.pollProgress();
    this.updateLoop();
  }

  /**
   * Calculates the failure percentage of the messages were requested to be sent.
   * 
   * @returns the percentage of messages that failed to send (as a whole number)
   */
  getfailurePercent(): number {
    if (!this.progressData) return 0;
    const total = this.progressData.msgs_sent + this.progressData.msgs_failed;
    return getPercent(this.progressData.msgs_failed, total);
  }

  /**
   * Updates the progress data every `updated_every` seconds.
   */
  async updateLoop(): Promise<void> {
    if (this.intervalId) {
      clearInterval(this.intervalId)
    }
    this.intervalId = setInterval(async () => {
      this.progressData = await ApiService.pollProgress();
      console.log(this.progressData);
    }, this.updatedEvery * 1000); 
  }

  /**
   * Called when user submits a the form to change the update interval.
   */
  submitForm(): void {
    if (this.progressForm.valid) {
      this.isSubmitting = true;
      const seconds = this.progressForm.value.updatesEvery;
      this.updatedEvery = seconds ? seconds : 5;
      this.updateLoop();
    }
  }
}
