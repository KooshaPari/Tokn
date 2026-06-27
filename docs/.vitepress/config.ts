import { defineConfig } from 'vitepress'

// Resolves the VitePress `base` path for GitHub Pages project sites vs custom domains.
function resolveDocsBase(): string {
  const explicit = process.env.DOCS_BASE ?? process.env.VITEPRESS_BASE
  if (explicit) return explicit.endsWith('/') ? explicit : `${explicit}/`
  if (process.env.PHENOTYPE_CUSTOM_DOMAIN === 'true') return '/'
  const repo = process.env.GITHUB_REPOSITORY?.split('/')[1]
  if (process.env.GITHUB_PAGES === 'true' && repo) return `/${repo}/`
  return '/'
}

const docsBase = resolveDocsBase()

export default defineConfig({
  title: 'TokenLedger',
  description: 'Token and cost tracking for LLM operations',
  base: docsBase,
  ignoreDeadLinks: true,
  srcExclude: ['**/templates/**', '**/research/**', '**/.generated/**'],
  themeConfig: {
    nav: [
      { text: 'Wiki', link: '/wiki/' },
      { text: 'Development Guide', link: '/development-guide/' },
      { text: 'Document Index', link: '/document-index/' },
      { text: 'API', link: '/api/' },
      { text: 'Roadmap', link: '/roadmap/' }
    ],
    sidebar: [{ text: 'Categories', items: [
      { text: 'Wiki', link: '/wiki/' },
      { text: 'Development Guide', link: '/development-guide/' },
      { text: 'Document Index', link: '/document-index/' },
      { text: 'API', link: '/api/' },
      { text: 'Roadmap', link: '/roadmap/' }
    ] }]
  }
})
