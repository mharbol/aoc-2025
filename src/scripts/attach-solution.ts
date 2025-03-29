
import { getDaySolution } from '@codegen/importDay'; // generated source file, run `npx astro build` if there are import errors

const button = document.getElementById("btn-solve")!;
const dayNumber: number = parseInt(button.getAttribute('name')!);
var theMod;
getDaySolution(dayNumber).then(mod => {
    theMod = mod;
});
let part1 = document.getElementById('part-1');
let part2 = document.getElementById('part-2');
button.addEventListener("click", () => {
    //@ts-ignore
    const puzzleInput: string = document.getElementById("puzzle-input")?.value;
    const inputArray: string[] = puzzleInput.split('\n');
    part1!.innerHTML = theMod!.part1(inputArray);
    part2!.innerHTML = theMod!.part2(inputArray);
});
