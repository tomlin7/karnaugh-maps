<script lang="ts">
  import KMap from "./lib/kmap.svelte";
  import { solve } from "./lib/kmapSolver";

  let variables = 2;
  let grid: (0 | 1 | null)[][] = Array(2 ** variables)
    .fill(null)
    .map(() => Array(2 ** variables).fill(null));
  let solution = "";
  let groups: number[][][] = [];

  function updateGrid(row: number, col: number) {
    if (grid[row][col] === null) {
      grid[row][col] = 1;
    } else if (grid[row][col] === 1) {
      grid[row][col] = 0;
    } else {
      grid[row][col] = null;
    }
    grid = grid;
    groups = []; // Reset groups when grid is modified
    solution = ""; // Reset solution when grid is modified
  }

  function updateVariables(newVariables: number) {
    variables = newVariables;
    grid = Array(2 ** variables)
      .fill(null)
      .map(() => Array(2 ** variables).fill(null));
    groups = [];
    solution = "";
  }

  function solveProblem() {
    const result = solve(grid);
    solution = result.expression;
    groups = result.groups;
  }
</script>

<main>
  <div>
    <KMap
      {grid}
      {variables}
      {groups}
      on:cellClick={({ detail: { row, col } }) => updateGrid(row, col)}
    />
  </div>
  <button on:click={solveProblem}>Solve</button>
  <div>
    <h2>Solution:</h2>
    <p>{solution}</p>
  </div>
</main>

<style>
  main {
    text-align: center;
    padding: 1em;
    max-width: 800px;
    margin: 0 auto;
  }

  button {
    font-size: 1.2em;
    padding: 10px 20px;
    margin: 20px 0;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>
