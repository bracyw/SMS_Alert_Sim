import { ComponentFixture, TestBed } from '@angular/core/testing';

import { CaptionedValueComponentComponent } from './captioned-value-component.component';
import { Unit } from '../../utils/enums/unit.enum';
import { hasSingleOccurrence } from 'testing/test_helper_functions';
import { By } from '@angular/platform-browser';

describe('CaptionedValueComponentComponent', () => {
  let component: CaptionedValueComponentComponent;
  let fixture: ComponentFixture<CaptionedValueComponentComponent>;
  const getAllTypographyComponents = (fixture: ComponentFixture<CaptionedValueComponentComponent>) => fixture.debugElement.queryAll(By.css('typography'));

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [CaptionedValueComponentComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(CaptionedValueComponentComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });


  it('displays correct amount of typography components', () => {
    const compiled = fixture.nativeElement;
    expect(compiled.querySelectorAll('typography').length).toEqual(2);
  });

  it('displays correct value and unit in same typography component', () => {
    component.value = "100";
    component.unit = Unit.PERCENT;
    fixture.detectChanges();
    // check the fist typography component which should contain the value and unit (IF YOU CHANGE THE LAYOUT TEST WILL FAIL)
    expect(hasSingleOccurrence(getAllTypographyComponents(fixture)[0], '100%')).toBeTrue();
  });

  it('displays correct caption in second typography component', () => {
    component.caption = "Caption";
    fixture.detectChanges();
    // check the second typography component which should contain the caption (IF YOU CHANGE THE LAYOUT TEST WILL FAIL)
    expect(hasSingleOccurrence(getAllTypographyComponents(fixture)[1], 'Caption')).toBeTrue();
  });


});
