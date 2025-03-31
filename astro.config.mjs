// @ts-check
import { defineConfig } from 'astro/config';
import srcgen from './src/scripts/srcgen';

// https://astro.build/config
export default defineConfig({
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
