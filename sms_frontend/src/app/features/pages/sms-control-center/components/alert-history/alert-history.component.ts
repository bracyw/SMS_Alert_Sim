import { Component, OnInit } from '@angular/core';
import { getAllHistoryData } from 'src/api/history.api';
import { VStackComponent } from 'src/app/shared/components/vstack/vstack.component';
import { TypographyComponent } from 'src/app/shared/components/typography/typography.component';
import { InfoBackgroundComponent } from 'src/app/shared/components/info-background/info-background.component';


export interface AlertHistory {
  message: string | null;
  send_amount_requested: number;
  messages_sent: number;
  messages_failed: number;
  total_time_to_send: number;
}

@Component({
  selector: 'app-alert-history',
  imports: [VStackComponent, TypographyComponent, InfoBackgroundComponent],
  templateUrl: './alert-history.component.html',
  styleUrl: './alert-history.component.css',
  standalone: true,
})
export class AlertHistoryComponent implements OnInit {
  alertHistory: AlertHistory[] = [];

  constructor() {
    this.getAlertHistory();
  }
  ngOnInit(): void {
    this.getAlertHistory();
  }



  async getAlertHistory(): Promise<void> {
      await getAllHistoryData().then(async (response) => {
        if (response.ok) {
          this.alertHistory = await response.json();
          console.log(this.alertHistory);
        }
      });

      console.log(this.alertHistory);
    }
  
}

