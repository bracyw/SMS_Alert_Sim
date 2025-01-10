import { Component, Input, OnInit } from '@angular/core';
import { Alert } from 'src/app/shared/utils/api/types/recieve/alert.type';
import { getPercent } from 'src/app/shared/utils/math.utils';
import { VStackComponent } from 'src/app/shared/components/vstack/vstack.component';
import { TypographyComponent } from 'src/app/shared/components/typography/typography.component';

@Component({
  selector: 'historical-alert-card',
  imports: [VStackComponent, TypographyComponent],
  templateUrl: './historical-alert-card.component.html',
  styleUrl: './historical-alert-card.component.css',
  standalone: true
})
export class HistoricalAlertCardComponent implements OnInit {
  // the alert to display (given the whole alert for future changes)
  @Input() alert: Alert = {
    message: "This is a placeholder",
    send_amount_requested: 0,
    messages_sent: 0,
    messages_failed: 0,
    total_time_to_send: 0
  };


  ngOnInit(): void {
  }



  /**
   * Returns failure percent for alert.
   * There is an assumption made that the full send amount was proccessed
   * 
   * @returns the failure percentage of the alert held by this component.
   */
  getFailureRate(): number {
    return getPercent(this.alert.messages_failed, this.alert.send_amount_requested);
  }

}
