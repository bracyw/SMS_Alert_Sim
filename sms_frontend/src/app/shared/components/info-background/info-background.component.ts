import { Component, Input } from '@angular/core';
import Theme from 'src/app/shared/utils/theme.utils';
import { TypographyComponent } from '../typography/typography.component';
import { MatIcon } from '@angular/material/icon';

/**
 * Component that is essentially the template/background for
 * info displayed on the dashboard
 */
interface ButtonInputs {
  onClick: () => void;
  icon: string;
}

@Component({
  selector: 'info-background',
  imports: [TypographyComponent, MatIcon],
  templateUrl: './info-background.component.html',
  styleUrls: ['./info-background.component.css'],
  standalone: true,
})
export class InfoBackgroundComponent {
  @Input() icon?: string;
  @Input() svgIcon?: string;
  @Input() backgroundColor?: string = Theme.infoBackground;
  @Input() title!: string;
  @Input() onClick!: (() => void) | undefined;
  @Input() button?: ButtonInputs;
}
