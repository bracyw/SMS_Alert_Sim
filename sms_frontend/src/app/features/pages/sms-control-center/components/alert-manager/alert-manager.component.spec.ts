import { ComponentFixture, TestBed } from '@angular/core/testing';

import { AlertManagerComponent } from './alert-manager.component';

describe('BroadcastManagerComponent', () => {
  let component: AlertManagerComponent;
  let fixture: ComponentFixture<AlertManagerComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [AlertManagerComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(AlertManagerComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
