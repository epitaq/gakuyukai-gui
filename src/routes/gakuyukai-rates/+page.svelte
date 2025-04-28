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

<!-- HTML (unchanged) -->
<div class="container">
  <div class="tabs">
    <button
      class="file-select-btn"
      on:click={handleSingleFileSelect}
      disabled={loading}
    >
      単一ファイルを選択
    </button>
    <button
      class="file-select-btn"
      on:click={handleMultipleFolderSelect}
      disabled={loading}
    >
      フォルダを選択
    </button>
  </div>
  {#if loading}
    <p>処理中...</p>
  {/if}
  {#if result?.circles.length}
    <div class="tab-content" style="overflow: scroll; height: 400px;">
      <h3>結果</h3>
      <table>
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
  {/if}
  {#if result}
    <button class="file-select-btn" on:click={exportToExcel} disabled={loading}>
      エクスポート
    </button>
  {/if}
  {#if result?.error_file_path.length}
    <div class="error-section">
      <h3>エラーファイル</h3>
      <ul>
        {#each result.error_file_path as errorPath}
          <li>{errorPath}</li>
        {/each}
      </ul>
    </div>
  {/if}
</div>

<style>
  /* Styles (unchanged) */
  .container {
    padding: 20px;
  }
  .tabs {
    display: flex;
    gap: 10px;
    margin-bottom: 20px;
  }
  .file-select-btn {
    padding: 10px 20px;
    background-color: #28a745;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
  }
  .file-select-btn:disabled {
    background-color: #6c757d;
    cursor: not-allowed;
  }
  table {
    width: 100%;
    border-collapse: collapse;
    margin-bottom: 20px;
  }
  th,
  td {
    padding: 12px;
    text-align: left;
    border-bottom: 1px solid #ddd;
  }
  th {
    background-color: #f8f9fa;
  }
  .error-section {
    margin-top: 30px;
    padding: 20px;
    background-color: #fff3f3;
    border-radius: 4px;
  }
  .error-section h3 {
    color: #dc3545;
    margin-bottom: 10px;
  }
  .error-section ul {
    list-style-type: none;
    padding: 0;
  }
  .error-section li {
    color: #dc3545;
    margin-bottom: 5px;
  }
</style>
