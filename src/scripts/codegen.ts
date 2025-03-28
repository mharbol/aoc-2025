
import { writeFileSync, readdirSync } from 'node:fs';

const theFileName = 'importDay.ts';
const theFunctionName = 'getDaySolution'
const thePreamble = `// This code was generated by \`src/scripts/codegen.ts\`. Do not modify.`;
const theDocumentation = "";

function getTheCases() {
    const theWantedFileRegex = /^day[0-9][0-9]\.ts$/;
    return readdirSync(import.meta.dirname)
        .filter(file => theWantedFileRegex.test(file))
        .map(file => file.substring(0, file.length - 3))
        .map(file => `        case ${parseInt(file.substring(3))}: return await import("@scripts/${file}");`)
        .join("\n")
}

function generateTheCode(): string {
    return thecode;
}

const thecode = `${thePreamble}
${theDocumentation}
export async function ${theFunctionName}(day: number) {
    switch (day) {
${getTheCases()}
    }
}
`

export default {
    name: 'codegen',
    hooks: {
        //@ts-ignore
        'astro:config:setup': ({ createCodegenDir }) => {
            const codegenDir = createCodegenDir();
            writeFileSync(new URL(theFileName, codegenDir), generateTheCode(), 'utf-8')
        }
    }
}
