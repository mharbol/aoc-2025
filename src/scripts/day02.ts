export function part1(lines: string[]): string {
    return lines.reduce(
        (acc, line) => acc += isSafe(line) ? 1 : 0,
        0
    ).toString();
}

export function part2(lines: string[]): string {
    return lines.reduce(
        (acc, line) => acc += isDampenSafe(line) ? 1 : 0,
        0
    ).toString();
}

function isSafe(report: string): boolean {
    let levels: number[] = [];
    report.split(' ').forEach(num => levels.push(parseInt(num)));
    let diffs = [];
    for (let idx = 1; idx < levels.length; ++idx) {
        diffs.push(levels[idx - 1] - levels[idx]);
    }
    const increasing: boolean = diffs[0] > 0;
    return diffs.every(diff => {
        return 0 != diff && Math.abs(diff) <= 3 &&
            ((increasing && 0 < diff) || (!increasing && 0 > diff));
    })
}

function isDampenSafe(report: string): boolean {
    let levels: number[] = [];
    report.split(' ').forEach(num => levels.push(parseInt(num)));
    for (let dropIdx = 0; dropIdx < levels.length; ++dropIdx) {
        let reportStr: string = "";
        for (let idx = 0; idx < levels.length; ++idx) {
            if (idx != dropIdx) {
                reportStr += levels[idx] + " ";
            }
        }
        if (isSafe(reportStr.trim())) {
            return true;
        }
    }
    return false;
}
