import { Component, OnInit } from '@angular/core';
import { getAllHistoryData } from 'src/api/history.api';
import { VStackComponent } from 'src/app/shared/components/vstack/vstack.component';
import { TypographyComponent } from 'src/app/shared/components/typography/typography.component';
import { InfoBackgroundComponent } from 'src/app/shared/components/info-background/info-background.component';
import { Alert } from 'src/app/shared/types/api/recieve/alert.type';
import { ApiService } from 'src/app/shared/services/api.service';

@Component({
  selector: 'app-alert-history',
  imports: [VStackComponent, TypographyComponent, InfoBackgroundComponent],
  templateUrl: './alert-history.component.html',
  styleUrl: './alert-history.component.css',
  standalone: true,
})

/**
 * Component for displaying the alert over all time for the current user 
 * (TODO: currently there is only one user in the system, there will be auth added).
 */
export class AlertHistoryComponent implements OnInit {
  alertHistory: Alert[] = [];

  constructor() {
    this.updateAlertHistory();
  }
  ngOnInit(): void {
    this.updateAlertHistory();
  }

  /**
   * Updates the alert history that will be displayed.
   * 
   * @returns void
   */
  async updateAlertHistory(): Promise<void> {
    const result = await ApiService.getAlertHistory();
    if (result instanceof Error) {
      console.error(result);
    } else {
      this.alertHistory = result;
    }
  }
  
}

