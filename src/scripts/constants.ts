
// Not all the text and constants are here, just the ones whose location isn't
// particularly obvious
export const YEAR: number = 2025;
export const DEFAULT_TITLE: string = `Advent of Code ${YEAR} | Marcus Harbol`;
export const AUTHOR: string = "Marcus Harbol || Software Engineer";
export const URL_BASE: string = "aoc-2025"
export const PROJECT_URL: string = `https://github.com/mharbol/${URL_BASE}`;
export const UPDATE_12_DAYS_URL: string = "https://adventofcode.com/2025/about#faq_num_days";

export function githubUrl(dayId: string): string {
    return `${PROJECT_URL}/blob/master/aoc_2025/src/solution/${dayId}.rs`
}
