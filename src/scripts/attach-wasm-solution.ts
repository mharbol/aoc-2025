
import { getDaySolution } from '@srcgen/importDay'; // generated source file, run `npx astro build` if there are import errors

const button = document.getElementById("btn-solve")!;
const dayNumber: number = parseInt(button.getAttribute('name')!);
let modSolution = getDaySolution(dayNumber);
var modInit = modSolution.init();
let part1 = document.getElementById('part-1');
let part2 = document.getElementById('part-2');
button.addEventListener("click", () => {
    //@ts-ignore
    const puzzleInput: string = document.getElementById("puzzle-input")?.value;
    const inputArray: string[] = puzzleInput.split('\n');
    modInit.then(() => {
        part1!.innerHTML = modSolution.part1!(inputArray);
        part2!.innerHTML = modSolution.part2!(inputArray);
    });
});
