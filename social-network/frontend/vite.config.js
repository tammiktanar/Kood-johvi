import {sveltekit} from '@sveltejs/kit/vite';
import {webSocketServer} from './webSocketPluginVite.js';

/** @type {import('vite').UserConfig} */
const config = {
    plugins: [sveltekit(), webSocketServer,
        (function LoadSecrets() {
            return {
                name: 'load-secrets',
                configureServer: async () => {
                    ;(await import('dotenv')).config()
                }
            }
        })()
    ]
};

export default config;
