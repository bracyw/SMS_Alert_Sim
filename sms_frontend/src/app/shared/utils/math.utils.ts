/** 
 * Calculate the percentage the part is of the total.
 * 
 * @param part the part of the total
 * @param total the total
 * 
 * @returns the percentage of the part in the total (will return 0 if total is 0)
*/
export const getPercent = (part: number, total: number): number => {
    return total === 0 ? 0 : (part / total) * 100;
}