import { writable, type Writable } from "svelte/store";
import * as wasm_model from "../meteen-storage-wasm/meteen_storage_wasm";
import { nanoid } from "nanoid";
import * as keyval from "idb-keyval";

export default class VaultStorage {
  public projects: Writable<wasm_model.Project[]>;
  private didLoad = false;

  constructor() {
    this.projects = writable([]);

    this.projects.subscribe(async (projects) => {
      // Save new vault on every change
      if (this.didLoad) {
        keyval
          .set("vault", wasm_model.serialize())
          .then(() => {
            console.log("Saved successfully");
          })
          .catch((e) => {
            console.error(e);
          });
      }
    });
  }

  public async load() {
    const serialized: Uint8Array | undefined = await keyval.get("vault");
    if (serialized) {
      wasm_model.deserialize(serialized);
      this.projects.set(wasm_model.get_all_projects());
    } else {
      console.log("Creating new vault");
      wasm_model.create_new_db();
      this.projects.set(wasm_model.get_all_projects());
    }

    this.didLoad = true;
  }

  public create_task(name: string) {
    const task = new wasm_model.Task(nanoid());
    task.summary = name;

    wasm_model.create_task(task, "inbox");
    this.projects.set(wasm_model.get_all_projects());
  }

  public updateTaskDone(task_id: string, done: boolean) {
    wasm_model.update_task_done(task_id, done);
    this.projects.set(wasm_model.get_all_projects());
  }
}
