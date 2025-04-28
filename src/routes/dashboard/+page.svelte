<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import {
    open,
    type DialogFilter,
    type OpenDialogOptions,
  } from "@tauri-apps/plugin-dialog";

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
    .catch((e) => console.error(e));

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
        invoke("wrap_load_gakuyukai_members", { path: v })
          .then(() => {
            console.info("successful loading gakuyukai file");
            updateInfo();
          })
          .catch((e) => {
            console.error(e);
          });
      })
      .catch((e) => console.error(e));
  }

  function updateInfo() {
    invoke<Info>("wrap_get_info")
      .then((data) => {
        info = data;
      })
      .catch((e) => console.error(e));
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

<div class="container mx-auto px-4 py-8">
  <div class="bg-white rounded-lg shadow-lg p-6">
    <!-- ヘッダー -->
    <div class="flex justify-between items-center mb-8">
      <h1 class="text-2xl font-bold text-gray-800">{info.name}</h1>
      <button
        class="bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded-lg transition-colors"
        on:click={openFileDialog}
      >
        ファイルを選択
      </button>
    </div>

    <!-- 統計カード -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      <!-- 総メンバー数 -->
      <div class="bg-blue-50 rounded-lg p-6">
        <h3 class="text-lg font-semibold text-gray-600 mb-2">総メンバー数</h3>
        <p class="text-4xl font-bold text-blue-600">{info.member_count}</p>
        <p class="text-sm text-gray-500 mt-2">全体のメンバー数</p>
      </div>

      <!-- 学友会メンバー数 -->
      <div class="bg-green-50 rounded-lg p-6">
        <h3 class="text-lg font-semibold text-gray-600 mb-2">
          学友会メンバー数
        </h3>
        <p class="text-4xl font-bold text-green-600">
          {info.gakuyukai_member_count}
        </p>
        <p class="text-sm text-gray-500 mt-2">学友会に所属するメンバー</p>
      </div>

      <!-- 比率 -->
      <div class="bg-purple-50 rounded-lg p-6">
        <h3 class="text-lg font-semibold text-gray-600 mb-2">学友会比率</h3>
        <p class="text-4xl font-bold text-purple-600">{info.rate_string}</p>
        <p class="text-sm text-gray-500 mt-2">
          全体に対する学友会メンバーの割合
        </p>
      </div>
      <!-- 団体の処理 -->
      <div class="bg-purple-50 rounded-lg p-6">
        <h3 class="text-lg font-semibold text-gray-600 mb-2">団体の学友会率</h3>
        <p class="text-4xl font-bold text-purple-600"></p>
        <button
          type="button"
          class="bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded-lg transition-colors"
          on:click={callWrapCalculateGakuyukaiRate}
        >
          Excelファイルを選択
        </button>
        <button
          type="button"
          class="bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded-lg transition-colors"
          on:click={callWrapCalculateGakuyukaiRates}
        >
          フォルダを選択
        </button>
      </div>
    </div>
  </div>
</div>
