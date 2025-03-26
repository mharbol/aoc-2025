// @ts-check
import { defineConfig } from 'astro/config';
import codegen from './src/scripts/codegen';

// https://astro.build/config
export default defineConfig({
  integrations: [
    codegen
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
