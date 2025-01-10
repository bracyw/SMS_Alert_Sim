import { Component, OnInit } from '@angular/core';
import { VStackComponent } from 'src/app/shared/components/vstack/vstack.component';
import { TypographyComponent } from 'src/app/shared/components/typography/typography.component';
import { InfoBackgroundComponent } from 'src/app/shared/components/info-background/info-background.component';
import { Alert } from 'src/app/shared/utils/api/types/recieve/alert.type';
import { ApiService } from 'src/app/shared/services/api.service';
import { interval } from 'rxjs';
import { HistoricalAlertCardComponent } from './components/historical-alert-card/historical-alert-card.component';


@Component({
  selector: 'alert-history',
  imports: [VStackComponent, InfoBackgroundComponent, HistoricalAlertCardComponent],
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
    interval(5000).subscribe(() => {
      this.updateAlertHistory();
    });
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

