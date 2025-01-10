import { Component } from '@angular/core';
import { AlertManagerComponent } from './components/alert-manager/alert-manager.component';
import { ProgressMonitorComponent } from './components/progress-monitor/progress-monitor.component';
import { TypographyComponent } from 'src/app/shared/components/typography/typography.component';
import { VStackComponent } from 'src/app/shared/components/vstack/vstack.component';
import { HStackComponent } from 'src/app/shared/components/hstack/hstack.component';
import { SystemConfigComponent } from './components/system-config/system-config.component';
import { AlertHistoryComponent } from './components/alert-history/alert-history.component';

@Component({
    selector: 'app-sms-control-center', 
    standalone: true, // Declares this as a standalone component
    templateUrl: './sms-control-center.component.html',
    styleUrls: ['./sms-control-center.component.css'],
    imports: [AlertHistoryComponent, ProgressMonitorComponent, 
        TypographyComponent, VStackComponent, HStackComponent, 
        SystemConfigComponent, AlertHistoryComponent, AlertManagerComponent]
})

/** Used for displaying all control options for this sms alert simulator app  */
export class SmsControlCenterComponent {}
