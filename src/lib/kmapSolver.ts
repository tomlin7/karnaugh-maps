export function solve(grid: (0 | 1 | null)[][]): { expression: string; groups: number[][][] } {
  const groups: number[][][] = findGroups(grid);
  const terms = groups.map(group => groupToTerm(group, grid.length));
  const expression = terms.join(' + ');
  return { expression, groups };
}

function findGroups(grid: (0 | 1 | null)[][]): number[][][] {
  const groups: number[][][] = [];
  const rows = grid.length;
  const cols = grid[0].length;
  const visited = Array.from({ length: rows }, () => Array(cols).fill(false));

  for (let i = 0; i < rows; i++) {
    for (let j = 0; j < cols; j++) {
      if (grid[i][j] === 1 && !visited[i][j]) {
        const group = [];
        dfs(grid, i, j, visited, group);
        groups.push(group);
      }
    }
  }

  return groups;
}

function dfs(grid: (0 | 1 | null)[][], row: number, col: number, visited: boolean[][], group: number[][]) {
  const rows = grid.length;
  const cols = grid[0].length;
  const stack = [[row, col]];

  while (stack.length > 0) {
    const [r, c] = stack.pop()!;
    if (r < 0 || r >= rows || c < 0 || c >= cols || grid[r][c] !== 1 || visited[r][c]) continue;

    visited[r][c] = true;
    group.push([r, c]);

    // Check all 4 possible directions (up, down, left, right)
    stack.push([r - 1, c]);
    stack.push([r + 1, c]);
    stack.push([r, c - 1]);
    stack.push([r, c + 1]);
  }
}

function groupToTerm(group: number[][], gridSize: number): string {
  const variables = gridSize === 2 ? 2 : 4;
  const minValues = new Array(variables).fill(Infinity);
  const maxValues = new Array(variables).fill(-Infinity);

  for (const [row, col] of group) {
    const cellValues = gridSize === 2
      ? [row, col]
      : [Math.floor(row / 2), Math.floor(col / 2), row % 2, col % 2];

    for (let i = 0; i < variables; i++) {
      minValues[i] = Math.min(minValues[i], cellValues[i]);
      maxValues[i] = Math.max(maxValues[i], cellValues[i]);
    }
  }

  const term = minValues
    .map((min, index) => {
      const max = maxValues[index];
      if (min === max) {
        const variable = String.fromCharCode(65 + index);
        return min === 0 ? variable + "'" : variable;
      }
      return '';
    })
    .filter(Boolean)
    .join('');

  return term || '1';
}