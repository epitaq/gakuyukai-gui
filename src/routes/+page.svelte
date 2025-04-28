<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { goto } from "$app/navigation";
  import {
    open,
    type DialogFilter,
    type OpenDialogOptions,
  } from "@tauri-apps/plugin-dialog";

  let coreMsg = $state("");
  let member = $state("");
  let open_error = $state("");
  const filters: DialogFilter[] = [
    { name: "excel file", extensions: ["xlsx", "xls"] },
  ];

  function openFileDialog() {
    const optionDialog: OpenDialogOptions = {
      title: "file dialog",
      filters,
      multiple: false,
      directory: false,
      defaultPath: coreMsg,
    };
    open(optionDialog)
      .then((v) => {
        if (v === null) return;
        coreMsg = v;
        invoke("wrap_load_gakuyukai_members", { path: v })
          .then(() => {
            console.info("successful loading gakuyukai file");
            // ファイル読み込み成功後にダッシュボードページに遷移
            goto("/dashboard");
          })
          .catch((e) => {
            console.error(e);
            open_error = e;
          });
      })
      .catch((_) => (coreMsg = "error"));
  }
</script>

<div class="container mx-auto px-4 py-8">
  <div class="bg-white rounded-lg shadow-lg p-6">
    <h1 class="text-2xl font-bold text-gray-800 mb-6">学友会メンバー管理</h1>

    <div class="space-y-4">
      <div class="flex items-center space-x-4">
        <button
          type="button"
          class="bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded-lg transition-colors"
          on:click={openFileDialog}
        >
          Excelファイルを選択
        </button>
      </div>

      {#if coreMsg}
        <div class="bg-gray-50 p-4 rounded-lg">
          <p class="text-sm text-gray-600">選択されたファイル:</p>
          <p class="text-gray-800 mt-1">{coreMsg}</p>
          <p class="text-sm text-gray-600">エラーログ:</p>
          <p class="text-gray-800 mt-1">{open_error}</p>
        </div>
      {/if}
    </div>
  </div>
</div>
