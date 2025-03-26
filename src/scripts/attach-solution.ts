
import { getDaySolution } from '@codegen/importDay'; // generated source file, run `npx astro build` if there are import errors

document.getElementsByName("btn-solve")?.forEach((button) => {
    console.log(button.id);
    //@ts-ignore
    var theMod;
    getDaySolution(parseInt(button.id)).then(mod => {
        theMod = mod;
    });
    button.addEventListener("click", () => {
        //@ts-ignore
        const puzzleInput: string = document.getElementById("puzzle-input")?.value;
        //@ts-ignore
        console.log(theMod?.part1(puzzleInput.split('\n')));
        //@ts-ignore
        console.log(theMod?.part2(puzzleInput.split('\n')));
    });
});
