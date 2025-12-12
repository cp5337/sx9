import { defineConfig } from 'vitepress'

export default defineConfig({
  title: "CTAS Documentation",
  description: "Documentation for the Convergent Threat Analysis System",
  base: '/',
  outDir: './.vitepress/dist',
  
  themeConfig: {
    logo: '/logo.svg',
    nav: [
      { text: 'Guide', link: '/guide/' },
      { text: 'System', link: '/system/' },
      { text: 'API', link: '/api/' },
      { text: 'HD4', link: '/hd4-framework/' }
    ],
    
    sidebar: {
      '/guide/': [
        {
          text: 'Getting Started',
          items: [
            { text: 'Introduction', link: '/guide/introduction' },
            { text: 'Quick Start', link: '/guide/quick-start' },
            { text: 'Installation', link: '/guide/installation' },
            { text: 'Interface', link: '/guide/interface' },
            { text: 'Navigation', link: '/guide/navigation' }
          ]
        }
      ],
      '/system/': [
        {
          text: 'Architecture',
          items: [
            { text: 'Overview', link: '/system/overview' },
            { text: 'Components', link: '/system/components' },
            { text: 'Data Architecture', link: '/system/data-architecture' },
            { text: 'Security', link: '/system/security' }
          ]
        }
      ]
    },

    socialLinks: [
      { icon: 'github', link: 'https://github.com/yourusername/ctas' }
    ],

    footer: {
      message: 'Released under the MIT License.',
      copyright: 'Copyright © 2024'
    }
  }
})