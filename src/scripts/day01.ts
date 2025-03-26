
export function part1(lines: string[]): string {
    let left: number[] = [];
    let right: number[] = [];
    lines.forEach(line => {
        let nums = line.split('   ');
        left.push(parseInt(nums[0]));
        right.push(parseInt(nums[1]));
    });
    left.sort();
    right.sort();
    let acc = 0;
    for (let idx = 0; idx < left.length; ++idx) {
        acc += Math.abs(left[idx] - right[idx]);
    }

    return acc.toString();
}

export function part2(lines: string[]): string {
    let left: number[] = [];
    let rightCount: Map<number, number> = new Map<number, number>();
    lines.forEach(line => {
        let nums = line.split('   ');
        left.push(parseInt(nums[0]));
        const right = parseInt(nums[1]);
        const rightVal = rightCount.get(right) ?? 0;
        rightCount.set(right, rightVal + 1);
    });
    let acc = 0;
    left.forEach(num => {
        acc += (rightCount.get(num) ?? 0) * num;
    })
    return acc.toString();
}
