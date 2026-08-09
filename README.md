# Nitesh Poudel — Portfolio

A static portfolio site built with [Astro](https://astro.build/), [Solid](https://www.solidjs.com/), and TypeScript. Astro renders the portfolio content; Solid powers the persisted theme control and copy-email interaction.

## Development

Use Node.js 22 or newer, then install dependencies:

```sh
npm install
```

Start the development server:

```sh
npm run dev
```

Validate the project and create a production build:

```sh
npm run check
npm run build
```

GitHub Actions installs dependencies with `npm ci`, builds `dist/`, and deploys it to GitHub Pages. Static assets, including the custom-domain `CNAME`, are served from `public/`.
