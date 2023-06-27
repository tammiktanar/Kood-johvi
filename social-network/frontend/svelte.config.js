import adapter from '@sveltejs/adapter-node'
import sveltePreprocess from 'svelte-preprocess'

/** @type {import('@sveltejs/kit').Config} */
export default {
	kit: {
		adapter: adapter()
	},
	preprocess: sveltePreprocess({
		scss: {
			prependData: `@import './static/themes/kood.scss';`
		}
	}),
	onwarn: (warning, handler) => {
		const { code, frame } = warning;
		if (code === "css-unused-selector")
			return;

		handler(warning);
	},
}
