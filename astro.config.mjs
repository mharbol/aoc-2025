// @ts-check
import { defineConfig } from 'astro/config';

// https://astro.build/config
export default defineConfig({
  markdown: {
    remarkPlugins: [
      'remark-gfm', 'remark-smartypants', 'remark-math'
    ],
    rehypePlugins: [
      'rehype-mathjax'
    ]
  }
});
