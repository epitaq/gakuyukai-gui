<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { goto } from "$app/navigation";
  import {
    open,
    type DialogFilter,
    type OpenDialogOptions,
  } from "@tauri-apps/plugin-dialog";
  import { writable } from "svelte/store";
  import { page } from "$app/stores";
  import { onMount } from "svelte";

  let coreMsg = writable("");
  let openError = writable("");
  let retrievedRows = writable<Array<Array<string>>>([]);
  let idColumnIndex = writable<number | null>(null);
  let isGakuyukaiColumnIndex = writable<number | null>(null);

  onMount(() => {
    const path = $page.url.searchParams.get("path");
    if (path && !$coreMsg) {
      coreMsg.set(path);
      loadInitialData(path);
    }
  });

  const filters: DialogFilter[] = [
    { name: "Excel Files", extensions: ["xlsx", "xls"] },
  ];

  function openFileDialog() {
    const optionDialog: OpenDialogOptions = {
      title: "file dialog",
      filters,
      multiple: false,
      directory: false,
      defaultPath: $coreMsg,
    };
    open(optionDialog)
      .then((v) => {
        if (v === null) return;
        coreMsg.set(v);
        loadInitialData(v);
      })
      .catch((_) => coreMsg.set("error"));
  }

  function loadInitialData(path: string) {
    invoke("read_excel_rows", { path, numRows: 3 })
      .then((rows: any[string]) => {
        console.log(rows);
        retrievedRows.set(rows);
      })
      .catch((e) => {
        console.error(e);
      });
  }

  function submitColumnSelection() {
    if ($idColumnIndex !== null && $isGakuyukaiColumnIndex !== null) {
      invoke("wrap_load_gakuyukai_members", {
        path: $coreMsg,
        idRow: $idColumnIndex,
        isRow: $isGakuyukaiColumnIndex,
      })
        .then(() => {
          console.info("Successfully loaded gakuyukai file");
          goto("/dashboard");
        })
        .catch((e) => {
          console.error(e);
          openError.set(e);
        });
    }
  }
</script>

<div class="card max-w-2xl mx-auto my-8">
  <div class="space-y-6">
    <!-- File Selection Card -->
    <div class="text-center mb-8">
      <h2 class="text-xl font-medium text-[--macos-text-primary]">
        学友会管理
      </h2>
      <p class="text-[--macos-text-secondary] mt-2">
        Excelファイルから学友会名簿を読み込みます
      </p>
    </div>
    <div class="flex justify-center">
      <button type="button" class="btn btn-primary" on:click={openFileDialog}>
        ファイルを選択
      </button>
    </div>
    {#if $coreMsg}
      <div class="bg-[--macos-bg-secondary] rounded-lg p-4 mt-6">
        <div class="space-y-3">
          <div>
            <h3 class="text-sm font-medium text-[--macos-text-secondary]">
              選択されたファイル
            </h3>
            <p class="text-[--macos-text-primary] mt-1">{$coreMsg}</p>
          </div>
        </div>
      </div>
    {/if}
  </div>
</div>
<!-- Column Selection Card -->
{#if $retrievedRows.length > 0}
  <div class="card max-w-2xl mx-auto my-8">
    <div class="space-y-3">
      <h3 class="text-xl font-medium text-[--macos-text-primary]">列を選択:</h3>
      <div>
        <table>
          <thead>
            <tr>
              {#each $retrievedRows[0] as header, idx}
                <th
                  class={idx === $idColumnIndex ||
                  idx === $isGakuyukaiColumnIndex
                    ? "selected-column"
                    : ""}
                >
                  {header || `列 ${idx + 1}`}
                </th>
              {/each}
            </tr>
          </thead>
          <tbody>
            {#each $retrievedRows.slice(1) as row}
              <tr>
                {#each row as cell, idx}
                  <td
                    class={idx === $idColumnIndex ||
                    idx === $isGakuyukaiColumnIndex
                      ? "selected-column"
                      : ""}
                  >
                    {cell}
                  </td>
                {/each}
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
      <div class="flex justify-around mt-4">
        <div>
          <label
            for="idColumnSelect"
            class="block text-[--macos-text-secondary]"
          >
            学籍番号の列:
          </label>
          <select
            bind:value={$idColumnIndex}
            id="idColumnSelect"
            class="form-select"
          >
            <option value={null}>選択してください</option>
            {#each $retrievedRows[0] as _, idx}
              <option value={idx}>列 {idx + 1}</option>
            {/each}
          </select>
        </div>
        <div>
          <label
            for="isGakuyukaiColumnSelect"
            class="block text-[--macos-text-secondary]"
          >
            学友会の列:
          </label>
          <select
            bind:value={$isGakuyukaiColumnIndex}
            id="isGakuyukaiColumnSelect"
            class="form-select"
          >
            <option value={null}>選択してください</option>
            {#each $retrievedRows[0] as _, idx}
              <option value={idx}>列 {idx + 1}</option>
            {/each}
          </select>
        </div>
      </div>
      <div class="flex justify-center mt-6">
        <button
          type="button"
          class="btn btn-primary"
          on:click={submitColumnSelection}
          disabled={$idColumnIndex === null || $isGakuyukaiColumnIndex === null}
        >
          学友会メンバーを読み込む
        </button>
      </div>
    </div>
  </div>
{/if}
{#if $openError}
  <div class="card max-w-2xl mx-auto my-8">
    <h3 class="text-sm font-medium text-[--macos-text-secondary]">
      エラーログ
    </h3>
    <p class="text-red-500 mt-1">{$openError}</p>
  </div>
{/if}

<style>
  .selected-column {
    background-color: rgba(59, 130, 246, 0.2); /* Light blue background */
  }
</style>
