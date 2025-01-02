import { Component } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { BroadcastManagerComponent } from './features/pages/sms-control-center/components/alert-manager/alert-manager.component';

@Component({
  selector: 'app-root',
  imports: [RouterOutlet],
  templateUrl: './app.component.html',
  styleUrl: './app.component.css'
})
export class AppComponent {
  title = 'sms_frontend';
}
