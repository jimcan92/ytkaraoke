import daisyui from 'daisyui';
import type { Config } from 'tailwindcss';

export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],

	theme: {
		extend: {
			animation: {
				scroll: 'scroll 5s linear infinite'
			},
			keyframes: {
				scroll: {
					from: { transform: 'translateX(100%)' },
					to: { transform: 'translateX(-100%)' }
				}
			}
		}
	},

	plugins: [daisyui]
} as Config;
