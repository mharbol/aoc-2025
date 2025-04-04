// @ts-check
import { defineConfig } from 'astro/config';
import srcgen from './src/scripts/srcgen';
import { URL_BASE } from './src/scripts/constants';

// https://astro.build/config
export default defineConfig({
  site: 'https://mharbol.github.io',
    base: `/${URL_BASE}`,
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
