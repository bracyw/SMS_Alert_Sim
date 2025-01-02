import { ComponentFixture, TestBed } from '@angular/core/testing';

import { BroadcastManagerComponent } from './alert-manager.component';

describe('BroadcastManagerComponent', () => {
  let component: BroadcastManagerComponent;
  let fixture: ComponentFixture<BroadcastManagerComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [BroadcastManagerComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(BroadcastManagerComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
