<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { goto } from "$app/navigation";
  import {
    open,
    type DialogFilter,
    type OpenDialogOptions,
  } from "@tauri-apps/plugin-dialog";
  import { confirm } from "@tauri-apps/plugin-dialog";

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
        invoke("read_excel_rows", { path: v, numRows: 3 })
          .then((rows) => console.log(rows))
          .catch((e) => {
            console.error(e);
          });
        invoke("wrap_load_gakuyukai_members", { path: v, idRow: 0, isRow: 2 })
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

<div class="card max-w-2xl mx-auto">
  <div class="space-y-6">
    <div class="text-center mb-8">
      <h2 class="text-xl font-medium text-[--macos-text-primary]">
        学友会メンバー管理
      </h2>
      <p class="text-[--macos-text-secondary] mt-2">
        Excelファイルから学友会メンバーを読み込みます
      </p>
    </div>

    <div class="flex justify-center">
      <button type="button" class="btn btn-primary" onclick={openFileDialog}>
        ファイルを選択
      </button>
    </div>

    {#if coreMsg}
      <div class="bg-[--macos-bg-secondary] rounded-lg p-4 mt-6">
        <div class="space-y-3">
          <div>
            <h3 class="text-sm font-medium text-[--macos-text-secondary]">
              選択されたファイル
            </h3>
            <p class="text-[--macos-text-primary] mt-1">{coreMsg}</p>
          </div>
          {#if open_error}
            <div>
              <h3 class="text-sm font-medium text-[--macos-text-secondary]">
                エラーログ
              </h3>
              <p class="text-red-500 mt-1">{open_error}</p>
            </div>
          {/if}
        </div>
      </div>
    {/if}
  </div>
</div>
