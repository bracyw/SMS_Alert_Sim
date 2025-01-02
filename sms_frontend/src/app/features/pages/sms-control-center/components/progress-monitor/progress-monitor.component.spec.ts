import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ProgressMonitorComponent } from './progress-monitor.component';

describe('ProgressMonitorComponent', () => {
  let component: ProgressMonitorComponent;
  let fixture: ComponentFixture<ProgressMonitorComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ProgressMonitorComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(ProgressMonitorComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
