import { defineConfig } from 'vitepress'

// Inlined from former cross-repo docs-hub/.vitepress/base.config import.
// Resolves the VitePress `base` path for GitHub Pages project sites.
function resolveDocsBase(): string {
  const explicit = process.env.DOCS_BASE
  if (explicit) return explicit.endsWith('/') ? explicit : `${explicit}/`
  const repo = process.env.GITHUB_REPOSITORY // "owner/repo"
  if (repo && repo.includes('/')) {
    const name = repo.split('/')[1]
    return `/${name}/`
  }
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
