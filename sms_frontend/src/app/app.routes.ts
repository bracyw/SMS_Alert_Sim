import { Routes } from '@angular/router';

export const routes: Routes = [
    { path: "", loadComponent: () => import('src/app/features/pages/sms-control-center/sms-control-center.component').then(m => m.SmsControlCenterComponent) },
];
