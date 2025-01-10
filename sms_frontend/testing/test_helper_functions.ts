import { DebugElement } from '@angular/core';


/**
 * Checks if a given text appears exactly once in the content of a DebugElement.
 * 
 * @param element - The DebugElement whose content will be checked.
 * @param text - The text to search for.
 * @returns `true` if the text appears exactly once, otherwise `false`.
 */
export function hasSingleOccurrence(element: DebugElement, text: string): boolean {
    const textContent = element.nativeElement.textContent;
    const occurrences = textContent.match(new RegExp(text, 'g'))?.length || 0;
    return occurrences === 1;
}
