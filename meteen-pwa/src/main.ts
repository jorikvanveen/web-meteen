import { nanoid } from "nanoid";
import "./app.css";
import App from "./App.svelte";
import VaultStorage from "./lib/storage";
import { mount } from "svelte";

const storage = new VaultStorage();

const app = mount(App, {
  target: document.getElementById("app")!,
  props: {
    storage,
  },
});

export default app;
