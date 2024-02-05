/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{html,js,ts,rs}", "../web-htmx/src/**/*.{html,js,ts,rs}"],
  theme: {
    extend: {},
  },
  plugins: [
    require("@tailwindcss/forms")({
      strategy: "base",
    }),
  ],
  safelist: [
    // used in web-client/server/forms.rs
    "sm:col-span-1", 
    "sm:col-span-2", 
    "sm:col-span-3",
    "sm:col-span-4", 
    "sm:col-span-5", 
    "sm:col-span-6",
    "sm:col-start-1", 
    "sm:col-start-2", 
    "sm:col-start-3",
    "sm:col-start-4", 
    "sm:col-start-5", 
    "sm:col-start-6",

    // used in web-client/server/modal.rs
    "sm:max-w-sm",
    "sm:max-w-md",
    "sm:max-w-lg",
    "sm:max-w-screen-sm",
    "sm:max-w-screen-md",
  ],  
}

