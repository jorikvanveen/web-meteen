import { nanoid } from "nanoid";
import "./app.css";
import App from "./App.svelte";
import VaultStorage from "./lib/storage";

const storage = new VaultStorage();

const app = new App({
  target: document.getElementById("app")!,
  props: {
    storage,
  },
});

export default app;
