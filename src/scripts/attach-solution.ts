
import { getDaySolution } from '@codegen/importDay'; // generated source file, run `npx astro build` if there are import errors

document.getElementsByName("btn-solve")?.forEach((button) => {
    let theDay: number = parseInt(button.id);
    //@ts-ignore
    var theMod;
    getDaySolution(theDay).then(mod => {
        theMod = mod;
    });
    let part1 = document.getElementById('part-1');
    let part2 = document.getElementById('part-2');
    button.addEventListener("click", () => {
        //@ts-ignore
        const puzzleInput: string = document.getElementById("puzzle-input")?.value;
        const inputArray: string[] = puzzleInput.split('\n');
        //@ts-ignore
        part1.innerHTML = theMod.part1(inputArray);
        //@ts-ignore
        part2.innerHTML = theMod.part2(inputArray);
    });
});
