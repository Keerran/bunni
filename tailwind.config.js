/** @type {import('tailwindcss').Config} */
export default {
    content: ['./src/**/*.{html,js,svelte,ts}'],
    theme: {
        extend: {
            colors: {
                main: {
                    darker: "#090910",
                    dark: "#0F0F1A",
                    DEFAULT: "#1A1A2E",
                    light: "#20233c",
                }
            }
        },
    },
    plugins: [],
}

