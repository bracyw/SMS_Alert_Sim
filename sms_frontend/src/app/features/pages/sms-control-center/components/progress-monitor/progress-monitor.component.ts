import { Component, OnInit } from '@angular/core';
import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule, FormControl, FormGroup, Validators, ReactiveFormsModule } from '@angular/forms';
import { InfoBackgroundComponent } from 'src/app/shared/components/info-background/info-background.component';
import { HStackComponent } from 'src/app/shared/components/hstack/hstack.component';
import { TypographyComponent } from 'src/app/shared/components/typography/typography.component';
import { VStackComponent } from 'src/app/shared/components/vstack/vstack.component';
import { DividerComponent } from 'src/app/shared/components/divider/divider';
import { getProgressData } from 'src/api/progress.api';
import { ProgressData } from 'src/app/shared/types/api/recieve/progress-data.type';
import { ApiService } from 'src/app/shared/services/api.service';
import { getPercent } from 'src/app/shared/utils/math.utils';


@Component({
  selector: 'app-progress-monitor',
  imports: [
    InfoBackgroundComponent,
    HStackComponent,
    TypographyComponent,
    VStackComponent,
    DividerComponent,
    ReactiveFormsModule
  ],
  templateUrl: './progress-monitor.component.html',
  styleUrls: ['./progress-monitor.component.css'],
  standalone: true,
})

/**
 * Component for used for displaying info about system health and alert info during this uptime.
 */
export class ProgressMonitorComponent implements OnInit {

  updated_every = 5;  // The number of seconds to wait before updating the progress data
  progressData: ProgressData | null = null; // the progress data to display
  isSubmitting = false;  // whether the form is currently submitting
  private intervalId: any;  // the id of the interval that updates the progress data
  progressForm = new FormGroup({
    updates_every: new FormControl<number>(5, {
      validators: [Validators.required, Validators.min(1)],
      nonNullable: true,
    }),
  });

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
    }, this.updated_every * 1000);
  }

  /**
   * Called when user submits a the form to change the update interval.
   */
  submitForm(): void {
    if (this.progressForm.valid) {
      this.isSubmitting = true;
      const seconds = this.progressForm.value.updates_every;
      this.updated_every = seconds ? seconds : 5;
    }
  }
}
