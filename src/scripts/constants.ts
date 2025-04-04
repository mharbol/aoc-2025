
// Not all the text and constants are here, just the ones whose location isn't
// particularly obvious
export const YEAR: number = 2025;
export const DEFAULT_TITLE: string = `Advent of Code ${YEAR}`;
export const AUTHOR: string = "Marcus Harbol, Software Engineer";
export const URL_BASE: string = "aoc-2025"

export function githubUrl(dayNumber: number): string {
    return `https://github.com/mharbol/${URL_BASE}/blob/master/aoc_2025/src/solution/day${dayNumber.toString().padStart(2, '0')}.rs`
}
