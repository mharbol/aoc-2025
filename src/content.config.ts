import { glob } from "astro/loaders";
import { z, defineCollection } from "astro:content";

const solutions = defineCollection({
    loader: glob({ pattern: '**/[^_]*.md', base: "./src/days" }),
    schema: z.object({
        dayNumber: z.number(),
        isReady: z.boolean(),
        inputRegex: z.string().optional(),
    })
});

export const collections = { solutions };
