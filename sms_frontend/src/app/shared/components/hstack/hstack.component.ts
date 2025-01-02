import { Component, Input, OnInit } from '@angular/core';

type JusitfyContent = 'space-between' | 'space-around' | 'space-evenly' | 'center' | 'start' | 'end';

type AlignItems = 'center' | 'start' | 'end' | 'stretch';

@Component({
  selector: 'hstack',
  templateUrl: './hstack.component.html',
  styleUrls: ['./hstack.component.css']
})
export class HStackComponent implements OnInit {
  @Input() spacing: string = '10px';
  @Input() justifyContent: JusitfyContent = 'center';
  @Input() alignItems: AlignItems = 'center';

  alignment!: string;

  ngOnInit() {
    this.alignment = `${this.justifyContent} ${this.alignItems}`;
  }
}
