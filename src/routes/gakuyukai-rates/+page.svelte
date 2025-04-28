<script lang="ts">
  import { page } from "$app/stores";
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import {
    open,
    type DialogFilter,
    type OpenDialogOptions,
  } from "@tauri-apps/plugin-dialog";

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

  let activeTab: "single" | "multiple" = "single";
  let singleFileResult: CircleInfo | null = null;
  let multipleFileResults: CircleGakuyukaiRates | null = null;
  let loading = false;

  onMount(() => {
    const mode = $page.url.searchParams.get("mode");
    if (mode === "multiple") {
      activeTab = "multiple";
    }
  });

  function handleTabChange(tab: "single" | "multiple") {
    activeTab = tab;
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
          const result = await invoke<CircleInfo>(
            "wrap_calculate_gakuyukai_rate",
            { path: selected },
          );
          console.log(result);
          singleFileResult = result; // 必要に応じて状態に保存
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
          const result = await invoke<CircleGakuyukaiRates>(
            "wrap_calculate_gakuyukai_rates",
            { path: selected },
          );
          console.log(result);
          multipleFileResults = result; // 必要に応じて状態に保存
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
</script>

<div class="container">
  <div class="tabs">
    <button
      class:active={activeTab === "single"}
      on:click={() => handleTabChange("single")}
    >
      単一ファイル
    </button>
    <button
      class:active={activeTab === "multiple"}
      on:click={() => handleTabChange("multiple")}
    >
      複数ファイル
    </button>
  </div>

  <div class="content">
    {#if activeTab === "single"}
      <div class="tab-content">
        <button
          class="file-select-btn"
          on:click={handleSingleFileSelect}
          disabled={loading}
        >
          Excelファイルを選択
        </button>
        {#if loading}
          <p>処理中...</p>
        {/if}
        {#if singleFileResult}
          <table>
            <thead>
              <tr>
                <th>団体名</th>
                <!-- <th>ファイルパス</th> -->
                <th>総メンバー数</th>
                <th>学友会メンバー数</th>
                <th>学友会率</th>
              </tr>
            </thead>
            <tbody>
              <tr>
                <td>{singleFileResult.name}</td>
                <!-- <td>{singleFileResult.file_path}</td> -->
                <td>{singleFileResult.member_count}</td>
                <td>{singleFileResult.gakuyukai_member_count}</td>
                <td>{singleFileResult.rate_string}</td>
              </tr>
            </tbody>
          </table>
        {/if}
      </div>
    {:else}
      <div class="tab-content">
        <button
          class="file-select-btn"
          on:click={handleMultipleFolderSelect}
          disabled={loading}
        >
          フォルダを選択
        </button>
        {#if loading}
          <p>処理中...</p>
        {/if}
        {#if multipleFileResults?.circles.length}
          <table>
            <thead>
              <tr>
                <th>団体名</th>
                <!-- <th>ファイルパス</th> -->
                <th>総メンバー数</th>
                <th>学友会メンバー数</th>
                <th>学友会率</th>
              </tr>
            </thead>
            <tbody>
              {#each multipleFileResults.circles as circle}
                <tr>
                  <td>{circle.name}</td>
                  <!-- <td>{circle.file_path}</td> -->
                  <td>{circle.member_count}</td>
                  <td>{circle.gakuyukai_member_count}</td>
                  <td>{circle.rate_string}</td>
                </tr>
              {/each}
            </tbody>
          </table>
        {/if}

        {#if multipleFileResults?.error_file_path.length}
          <div class="error-section">
            <h3>エラーファイル</h3>
            <ul>
              {#each multipleFileResults.error_file_path as errorPath}
                <li>{errorPath}</li>
              {/each}
            </ul>
          </div>
        {/if}
      </div>
    {/if}
  </div>
</div>

<style>
  .container {
    padding: 20px;
  }

  .tabs {
    display: flex;
    gap: 10px;
    margin-bottom: 20px;
  }

  .tabs button {
    padding: 10px 20px;
    border: none;
    border-radius: 4px;
    background-color: #f0f0f0;
    cursor: pointer;
  }

  .tabs button.active {
    background-color: #007bff;
    color: white;
  }

  .file-select-btn {
    margin-bottom: 20px;
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
