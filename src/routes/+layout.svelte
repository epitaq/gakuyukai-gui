<script lang="ts">
  let { children } = $props();
  import "../app.css";
  import { page } from "$app/stores";

  // ページタイトルのマッピング
  const pageTitles: Record<string, string> = {
    "": "ホーム",
    dashboard: "ダッシュボード",
    "gakuyukai-rates": "学友会料金",
  };

  // 現在のページ名とタイトルを取得
  const currentPage = $derived($page.url.pathname.split("/")[1] || "");
  const pageTitle = $derived(pageTitles[currentPage] || "ホーム");
</script>

<div class="window-frame min-h-screen">
  <!-- ツールバー -->
  <!-- <div class="bg-[--macos-bg-secondary] border-b border-[--macos-border] p-4">
    <div class="flex items-center justify-between">
      <h1 class="text-lg font-medium text-[--macos-text-primary]">
        {pageTitle}
      </h1>
      <div class="flex items-center space-x-2">
      </div>
    </div>
  </div> -->

  <!-- メインコンテンツ -->
  <main class="p-6">
    {@render children()}
  </main>
</div>

<style>
  /* スタイルは app.css で定義済み */
</style>
