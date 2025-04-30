<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import {
    open,
    type DialogFilter,
    type OpenDialogOptions,
  } from "@tauri-apps/plugin-dialog";
  import { message } from "@tauri-apps/plugin-dialog";

  // Infoの型定義
  type Info = {
    name: string;
    file_path: string;
    member_count: number;
    gakuyukai_member_count: number;
    rate_string: string;
  };

  // 初期値を設定
  let info: Info = {
    name: "",
    file_path: "",
    member_count: 0,
    gakuyukai_member_count: 0,
    rate_string: "0%",
  };

  // 非同期で情報を取得しセットする
  invoke<Info>("wrap_get_info")
    .then((data) => {
      info = data;
    })
    .catch((e) => {
      message(e, { title: "Tauri", kind: "warning" });
      console.error(e);
    });

  const filters: DialogFilter[] = [
    { name: "excel file", extensions: ["xlsx", "xls"] },
  ];

  function openFileDialog() {
    const optionDialog: OpenDialogOptions = {
      title: "file dialog",
      filters,
      multiple: false,
      directory: false,
      defaultPath: info.file_path,
    };

    open(optionDialog)
      .then((v) => {
        if (v === null) return;

        info.file_path = v;
        goto(`/?path=${encodeURIComponent(v)}`);
      })
      .catch((e) => {
        console.error(e);
        message(e, { title: "Tauri", kind: "warning" });
      });
  }

  function updateInfo() {
    invoke<Info>("wrap_get_info")
      .then((data) => {
        info = data;
      })
      .catch((e) => {
        console.error(e);
        message(e, { title: "Tauri", kind: "warning" });
      });
  }

  function checkExcelFilePath(path: string) {
    return (
      (path.endsWith(".xlsx") || path.endsWith(".xls")) && !path.includes("~$")
    );
  }

  import { goto } from "$app/navigation";

  async function callWrapCalculateGakuyukaiRate() {
    const selected = await open({
      title: "file dialog",
      filters: [{ name: "excel file", extensions: ["xlsx", "xls"] }],
      multiple: false,
      directory: false,
    });

    if (selected) {
      goto(`/gakuyukai-rates?mode=single&path=${encodeURIComponent(selected)}`);
    }
  }

  async function callWrapCalculateGakuyukaiRates() {
    const selected = await open({
      title: "file dialog",
      multiple: false,
      directory: true,
    });

    if (selected) {
      goto(
        `/gakuyukai-rates?mode=multiple&path=${encodeURIComponent(selected)}`,
      );
    }
  }
</script>

<div class="max-w-6xl mx-auto">
  <!-- ヘッダー -->
  <div class="flex justify-between items-center mb-8">
    <div>
      <h2 class="text-xl font-medium text-[--macos-text-primary]">
        {info.name || "統計"}
      </h2>
      <p class="text-[--macos-text-secondary] mt-1">学友会の統計情報</p>
    </div>
    <button class="btn" on:click={openFileDialog}> 更新 </button>
  </div>

  <!-- 統計カード -->
  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
    <!-- 総数 -->
    <div class="card">
      <h3 class="text-[--macos-text-secondary] font-medium mb-2">総数</h3>
      <p class="text-4xl font-medium text-[--macos-accent]">
        {info.member_count}
      </p>
      <p class="text-[--macos-text-secondary] mt-2">全体の人数</p>
    </div>

    <!-- 学友会数 -->
    <div class="card">
      <h3 class="text-[--macos-text-secondary] font-medium mb-2">学友会数</h3>
      <p class="text-4xl font-medium text-[--macos-accent]">
        {info.gakuyukai_member_count}
      </p>
      <p class="text-[--macos-text-secondary] mt-2">学友会に所属する人数</p>
    </div>

    <!-- 比率 -->
    <div class="card">
      <h3 class="text-[--macos-text-secondary] font-medium mb-2">学友会比率</h3>
      <p class="text-4xl font-medium text-[--macos-accent]">
        {info.rate_string}
      </p>
      <p class="text-[--macos-text-secondary] mt-2">学友会に所属する人の割合</p>
    </div>
  </div>

  <!-- 団体の処理 -->
  <div class="card mt-8">
    <h3 class="text-[--macos-text-secondary] font-medium mb-4">
      団体の学友会率
    </h3>
    <div class="flex gap-4">
      <button
        type="button"
        class="btn btn-primary"
        on:click={callWrapCalculateGakuyukaiRate}
      >
        ファイルを選択
      </button>
      <button
        type="button"
        class="btn"
        on:click={callWrapCalculateGakuyukaiRates}
      >
        フォルダを選択
      </button>
    </div>
  </div>
</div>
