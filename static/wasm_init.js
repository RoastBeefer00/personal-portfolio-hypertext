import init, { Universe, run_benchmark } from "/game-of-life/conway_game_of_life_rs.js?v=3";

export const wasm = await init();
export { Universe, run_benchmark };
