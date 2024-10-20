<script lang="ts">
  import { createEventDispatcher } from "svelte";

  export let grid: (0 | 1 | null)[][];
  export let variables: number;
  export let groups: number[][][];

  const dispatch = createEventDispatcher();

  function handleCellClick(row: number, col: number) {
    dispatch("cellClick", { row, col });
  }

  function getCellContent(value: 0 | 1 | null): string {
    if (value === null) return "X";
    return value.toString();
  }

  function getGroupColor(groupIndex: number): string {
    const colors = [
      "#ff9999",
      "#99ff99",
      "#9999ff",
      "#ffff99",
      "#ff99ff",
      "#99ffff",
      "#ffd700",
      "#00ced1",
    ];
    return colors[groupIndex % colors.length];
  }

  function isInGroup(row: number, col: number): number | null {
    for (let i = 0; i < groups.length; i++) {
      if (groups[i].some((cell) => cell[0] === row && cell[1] === col)) {
        return i;
      }
    }
    return null;
  }
</script>

<div class="kmap">
  {#each grid as row, i}
    <div class="row">
      {#each row as cell, j}
        {@const groupIndex = isInGroup(i, j)}
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <!-- svelte-ignore a11y-no-static-element-interactions -->
        <div
          class="cell"
          on:click={() => handleCellClick(i, j)}
          style="background-color: {groupIndex !== null
            ? getGroupColor(groupIndex)
            : 'transparent'};"
        >
          <span
            class="value"
            style="color: {groupIndex !== null ? '#000' : '#fff'};"
          >
            {getCellContent(cell)}
          </span>
        </div>
      {/each}
    </div>
  {/each}
</div>

<style>
  .kmap {
    display: inline-block;
    border: 1px solid #ccc;
    margin: 1em 0;
  }

  .row {
    display: flex;
  }

  .cell {
    width: 60px;
    height: 60px;
    border: 1px solid #ccc;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    user-select: none;
    transition: background-color 0.3s ease;
  }

  .value {
    font-size: 1.5em;
    font-weight: bold;
    transition: color 0.3s ease;
  }
</style>
