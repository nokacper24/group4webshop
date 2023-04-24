import { defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";
import basicSSL from "@vitejs/plugin-basic-ssl";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react(), basicSSL()],
  server: {
    proxy: {
      "^/(api|resources)/": {
        target: process.env.BACKEND_URL || "http://localhost:8083",
        changeOrigin: true,
        secure: false,
      },
    },
  },
});
