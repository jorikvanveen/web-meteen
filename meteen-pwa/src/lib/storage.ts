import { writable, type Writable } from "svelte/store";
import * as wasm_model from "../meteen-storage-wasm/meteen_storage_wasm";
import { nanoid } from "nanoid";
import * as keyval from "idb-keyval";

const DO_JSON = true;

export default class VaultStorage {
  public projects: Writable<wasm_model.Project[]>;
  private didLoad = false;

  constructor() {
    this.projects = writable([]);

    this.projects.subscribe(async (projects) => {
      // Save new vault on every change
      if (this.didLoad) {
        if (DO_JSON) {
          keyval
            .set("vault", wasm_model.serialize_json())
            .then(() => {
              console.log("Saved successfully");
            })
            .catch((e) => {
              console.error(e);
            });
        } else {
          keyval
            .set("vault", wasm_model.serialize())
            .then(() => {
              console.log("Saved successfully");
            })
            .catch((e) => {
              console.error(e);
            });
        }
      }
    });
  }

  public async load() {
    const serialized: Uint8Array | string | undefined =
      await keyval.get("vault");

    if (serialized) {
      if (DO_JSON) {
        console.dir(JSON.parse(serialized as string));
        wasm_model.deserialize_json(serialized as string);
      } else {
        wasm_model.deserialize(serialized as Uint8Array);
      }
      this.projects.set(wasm_model.get_all_projects());
    } else {
      console.log("Creating new vault");
      wasm_model.create_new_db();
      this.projects.set(wasm_model.get_all_projects());
    }

    this.didLoad = true;
  }

  public create_task(
    name: string,
    deadline: Date | null,
    deadlineHasTime: boolean,
    scheduled: Date | null,
    scheduledHasTime: boolean,
    priority: number,
  ) {
    const task = new wasm_model.Task(nanoid());
    task.summary = name;
    task.priority = new wasm_model.Priority(priority);

    console.log("Creating task with deadline", deadline?.valueOf());

    if (deadline) {
      task.deadline = new wasm_model.DateOrDateTime(
        deadlineHasTime,
        BigInt(deadline.valueOf()),
      );
    }

    if (scheduled) {
      task.scheduled = new wasm_model.DateOrDateTime(
        scheduledHasTime,
        BigInt(scheduled.valueOf()),
      );
    }

    wasm_model.create_task(task, "inbox");
    this.projects.set(wasm_model.get_all_projects());
  }

  public updateTaskDone(task_id: string, done: boolean) {
    wasm_model.update_task_done(task_id, done);
    this.projects.set(wasm_model.get_all_projects());
  }
}
