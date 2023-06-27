import { defineConfig } from 'vite'
import EnvironmentPlugin from 'vite-plugin-environment'

export default defineConfig({
	root: "./client",
	server: {
		port: 8080,
	},
	plugins: [
		EnvironmentPlugin({"PORT": null, "PRODUCTION": null}, { defineOn: "import.meta.env" }),
	],
})
