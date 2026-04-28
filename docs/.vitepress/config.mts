import { defineConfig, withBase } from 'vitepress'

const docsBase = '/Tokn/'

export default defineConfig({
  base: docsBase,
  title: 'Tokn',
  description: 'Tokn documentation for token management and pricing governance',
  lang: 'en-US',
  head: [['link', { rel: 'icon', href: withBase('/favicon.svg') }]],
})
