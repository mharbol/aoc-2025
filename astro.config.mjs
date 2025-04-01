// @ts-check
import { defineConfig } from 'astro/config';
import srcgen from './src/scripts/srcgen';

// https://astro.build/config
export default defineConfig({
  site: 'https://mharbol.github.io',
  base: '/aoc-2025',
  integrations: [
    srcgen
  ],
  markdown: {
    remarkPlugins: [
      'remark-gfm', 'remark-smartypants', 'remark-math'
    ],
    rehypePlugins: [
      'rehype-mathjax'
    ]
  }
});
