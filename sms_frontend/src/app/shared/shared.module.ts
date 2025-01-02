import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { InfoBackgroundComponent } from './components/info-background/info-background.component';
import { TypographyComponent } from './components/typography/typography.component';




@NgModule({
  declarations: [
    InfoBackgroundComponent,
    TypographyComponent
  ],
  imports: [
    CommonModule
  ]
})
export class SharedModule { }
