import { Component, Input } from '@angular/core';
import { Unit } from '../../utils/enums/unit.enum';
import { TypographyComponent } from '../typography/typography.component';
import { VStackComponent } from '../vstack/vstack.component';

@Component({
  selector: 'captioned-value-component',
  imports: [TypographyComponent, VStackComponent],
  templateUrl: './captioned-value-component.component.html',
  styleUrl: './captioned-value-component.component.css',
  standalone: true
})
export class CaptionedValueComponentComponent {
  @Input() unit: Unit = Unit.NONE;
  @Input() value: String = "";
  @Input() caption: string = "";

  constructor() { }

  ngOnInit(): void {
  }

}
