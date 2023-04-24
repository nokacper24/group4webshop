import { defineConfig, loadEnv } from "vite";
import react from "@vitejs/plugin-react-swc";
import basicSSL from "@vitejs/plugin-basic-ssl";

// https://vitejs.dev/config/
export default ({ mode }) => {
  process.env = { ...process.env, ...loadEnv(mode, process.cwd()) };
  const url = `${process.env.VITE_URL}:${process.env.VITE_PORT}`;

  return defineConfig({
    plugins: [react(), basicSSL()],
    server: {
      proxy: {
        "^/(api|resources)/": {
          target: url,
          changeOrigin: true,
          secure: false,
        },
      },
    },
  });
}