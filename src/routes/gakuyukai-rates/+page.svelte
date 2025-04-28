<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open, save } from "@tauri-apps/plugin-dialog";
  import { page } from "$app/stores";
  import { onMount } from "svelte";

  interface CircleInfo {
    name: string;
    file_path: string;
    member_count: number;
    gakuyukai_member_count: number;
    rate_string: string;
  }

  interface CircleGakuyukaiRates {
    circles: CircleInfo[];
    error_file_path: string[];
  }

  let result: CircleGakuyukaiRates | null = null;
  let loading = false;

  function convertSingleToMultiple(single: CircleInfo): CircleGakuyukaiRates {
    return {
      circles: [single],
      error_file_path: [],
    };
  }

  onMount(() => {
    const path = $page.url.searchParams.get("path");
    const mode = $page.url.searchParams.get("mode");
    if (path && mode) {
      handlePathCalculation(path, mode);
    }
  });

  async function handlePathCalculation(path: string, mode: string) {
    loading = true;
    try {
      if (mode === "single") {
        const singleResult = await invoke<CircleInfo>(
          "wrap_calculate_gakuyukai_rate",
          { path: decodeURIComponent(path) },
        );
        result = convertSingleToMultiple(singleResult);
      } else if (mode === "multiple") {
        result = await invoke<CircleGakuyukaiRates>(
          "wrap_calculate_gakuyukai_rates",
          { path: decodeURIComponent(path) },
        );
      }
    } catch (error) {
      console.error("Error calculating rates via path parameter:", error);
    } finally {
      loading = false;
    }
  }

  async function handleSingleFileSelect() {
    try {
      const selected = await open({
        title: "file dialog",
        filters: [{ name: "Excel", extensions: ["xlsx", "xls"] }],
        multiple: false,
        directory: false,
      });
      if (selected && typeof selected === "string") {
        loading = true;
        try {
          const singleResult = await invoke<CircleInfo>(
            "wrap_calculate_gakuyukai_rate",
            { path: selected },
          );
          result = convertSingleToMultiple(singleResult);
        } catch (error) {
          console.error("Error calculating rate:", error);
        } finally {
          loading = false;
        }
      }
    } catch (error) {
      console.error("Error opening file dialog:", error);
    }
  }

  async function handleMultipleFolderSelect() {
    try {
      const selected = await open({
        title: "file dialog",
        multiple: false,
        directory: true,
      });
      if (selected && typeof selected === "string") {
        loading = true;
        try {
          result = await invoke<CircleGakuyukaiRates>(
            "wrap_calculate_gakuyukai_rates",
            { path: selected },
          );
        } catch (error) {
          console.error("Error calculating rates:", error);
        } finally {
          loading = false;
        }
      }
    } catch (error) {
      console.error("Error opening folder dialog:", error);
    }
  }

  async function exportToExcel() {
    if (result) {
      try {
        const selected = await save({
          title: "Save Excel File",
          defaultPath: "export.xlsx",
        });
        if (selected && typeof selected === "string") {
          await invoke("wrap_export_to_excel", {
            rates: result,
            path: selected,
          });
          console.log("Exported successfully!");
        }
      } catch (error) {
        console.error("Error exporting to Excel:", error);
      }
    }
  }

  function updateCircleName(index: number, newName: string) {
    if (result && result.circles[index]) {
      result.circles[index].name = newName;
    }
  }
</script>

<!-- HTML -->
<div class="max-w-6xl mx-auto p-4">
  <!-- ヘッダー -->
  <div class="flex justify-between items-center mb-4">
    <div>
      <h2 class="text-xl font-medium text-[--macos-text-primary]">学友会率</h2>
      <p class="text-[--macos-text-secondary] mt-1">団体ごとの学友会率を計算</p>
    </div>
    <div class="flex gap-4">
      <button class="btn" on:click={handleSingleFileSelect} disabled={loading}>
        ファイルを選択
      </button>
      <button
        class="btn"
        on:click={handleMultipleFolderSelect}
        disabled={loading}
      >
        フォルダを選択
      </button>
    </div>
  </div>
  <div>
    {#if loading}
      <div class="flex justify-center items-center py-8">
        <p class="text-[--macos-text-secondary]">処理中...</p>
      </div>
    {/if}
    {#if result?.circles.length}
      <div class="card mb-4">
        <div class="flex justify-between items-center mb-2">
          <h3 class="text-lg font-medium text-[--macos-text-primary]">結果</h3>
          {#if result}
            <button
              class="btn btn-primary"
              on:click={exportToExcel}
              disabled={loading}
            >
              エクスポート
            </button>
          {/if}
        </div>
        <div class="scrollable">
          <table class="min-w-full">
            <thead>
              <tr>
                <th>団体名</th>
                <th>総メンバー数</th>
                <th>学友会メンバー数</th>
                <th>学友会率</th>
              </tr>
            </thead>
            <tbody>
              {#each result.circles as circle, index}
                <tr>
                  <td>
                    <input
                      type="text"
                      class="w-full"
                      value={circle.name}
                      on:input={(e) =>
                        updateCircleName(
                          index,
                          (e.target as HTMLInputElement).value,
                        )}
                    />
                  </td>
                  <td>{circle.member_count}</td>
                  <td>{circle.gakuyukai_member_count}</td>
                  <td>{circle.rate_string}</td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      </div>
    {/if}
    {#if result?.error_file_path.length}
      <div class="card border-red-200">
        <h3 class="text-red-600">エラーファイル</h3>
        <ul class="space-y-2 error-scrollable">
          {#each result.error_file_path as errorPath}
            <li class="text-red-500">{errorPath}</li>
          {/each}
        </ul>
      </div>
    {/if}
  </div>
</div>

<style>
  .scrollable {
    overflow-y: scroll;
    max-height: 50vh;
    overscroll-behavior: none;
  }
  .error-scrollable {
    overflow-y: scroll;
    max-height: 10vh;
    overscroll-behavior: none;
  }
</style>
