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
export class ProgressMonitorComponent implements OnInit {
  updated_every = 5;

  ngOnInit(): void {
    // try to poll immediately
    this.pollProgress();
    this.updateLoop();
  }
  progressForm = new FormGroup({
    updates_every: new FormControl<number>(5, {
      validators: [Validators.required, Validators.min(1)],
      nonNullable: true,
    }),
  });

  private intervalId: any;
  progressData: { msgs_sent: number; msgs_failed: number; avg_wait_time: number } | null = null;
  isSubmitting = false;

  get failurePercent(): number {
    if (!this.progressData) return 0;
    const total = this.progressData.msgs_sent + this.progressData.msgs_failed;
    return total === 0 ? 0 : (this.progressData.msgs_failed / total) * 100;
  }

  async updateLoop(): Promise<void> {
    if (this.intervalId) clearInterval(this.intervalId);
      this.intervalId = setInterval(() => {
        this.pollProgress();
      }, this.updated_every * 1000);
  }

  async pollProgress(): Promise<void> {
    await getProgressData().then(async (response) => {
      if (response.ok) {
        this.progressData = await response.json();
      }
    });
  }

  submitForm(): void {
    if (this.progressForm.valid) {
      this.isSubmitting = true;
      const seconds = this.progressForm.value.updates_every;
      this.updated_every = seconds ? seconds : 5;
    }
  }
}
