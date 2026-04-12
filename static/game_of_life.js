function getRandomCell() {
  const randomBool = Math.random() < 0.5;
  if (randomBool) {
    return Cell.Alive;
  } else {
    return Cell.Dead;
  }
}

const Cell = Object.freeze({
  Dead: 0,
  Alive: 1,
});

export class Universe {
  #width;
  #height;
  #cells;
  #pixel_buffer;

  constructor(width, height) {
    this.#width = width;
    this.#height = height;
    this.#cells = new Array(width * height);
    for (let i = 0; i < width * height; i++) {
      this.#cells[i] = getRandomCell();
    }
    this.#pixel_buffer = new Uint8ClampedArray(width * height * 4);
  }

  width() {
    return this.#width;
  }

  height() {
    return this.#height;
  }

  cells() {
    return this.#cells;
  }

  pixel_buffer() {
    return this.#pixel_buffer;
  }

  getIndex(row, column) {
    return row * this.#width + column;
  }

  // Gets neighbor count, wrapping around the edges of the universe.
  getLiveNeighborCount(row, column) {
    let count = 0;
    let north = row === 0 ? this.#height - 1 : row - 1;
    let south = row === this.#height - 1 ? 0 : row + 1;
    let west = column === 0 ? this.#width - 1 : column - 1;
    let east = column === this.#width - 1 ? 0 : column + 1;

    if (this.#cells[this.getIndex(north, west)] === Cell.Alive) {
      count++;
    }
    if (this.#cells[this.getIndex(north, column)] === Cell.Alive) {
      count++;
    }
    if (this.#cells[this.getIndex(north, east)] === Cell.Alive) {
      count++;
    }
    if (this.#cells[this.getIndex(row, west)] === Cell.Alive) {
      count++;
    }
    if (this.#cells[this.getIndex(row, east)] === Cell.Alive) {
      count++;
    }
    if (this.#cells[this.getIndex(south, west)] === Cell.Alive) {
      count++;
    }
    if (this.#cells[this.getIndex(south, column)] === Cell.Alive) {
      count++;
    }
    if (this.#cells[this.getIndex(south, east)] === Cell.Alive) {
      count++;
    }

    return count;
  }

  tick() {
    let new_cells = this.#cells.slice();
    for (let row = 0; row < this.#height; row++) {
      for (let col = 0; col < this.#width; col++) {
        let idx = this.getIndex(row, col);
        let cell = this.#cells[idx];
        let live_neighbors = this.getLiveNeighborCount(row, col);

        let next_cell;
        if (cell === Cell.Alive && live_neighbors < 2) {
          next_cell = Cell.Dead;
        } else if (cell === Cell.Alive && (live_neighbors === 2 || live_neighbors === 3)) {
          next_cell = Cell.Alive;
        } else if (cell === Cell.Alive && live_neighbors > 3) {
          next_cell = Cell.Dead;
        } else if (cell === Cell.Dead && live_neighbors === 3) {
          next_cell = Cell.Alive;
        } else {
          next_cell = cell;
        }

        new_cells[idx] = next_cell;
      }
    }

    this.#cells = new_cells;
  }

  render() {
    for (let i = 0; i < this.#cells.length; i++) {
      let pixel_index = i * 4;
      let [r, g, b] = this.#cells[i] === Cell.Alive ? [0, 0, 0] : [255, 255, 255];

      this.#pixel_buffer[pixel_index] = r;
      this.#pixel_buffer[pixel_index + 1] = g;
      this.#pixel_buffer[pixel_index + 2] = b;
      this.#pixel_buffer[pixel_index + 3] = 255; // opaque
    }
  }
}
