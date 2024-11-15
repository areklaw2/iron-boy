import { GameBoy } from "wasm";

const canvas = document.getElementById("screen");
const ctx = canvas.getContext("2d");
let gameBoy = null;
let gameName = null;
let intervalId = 0;

const loadFile = (file) => {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = () => {
      const data = reader.result;
      const buffer = new Uint8Array(data);
      gameBoy = new GameBoy(file.name, buffer, false);
      gameName = gameBoy.game_title();
      resolve();
    };
    reader.onerror = (error) => {
      reject(error);
    };
    reader.readAsArrayBuffer(file);
  });
};

const startGameBoy = () => {
  // if (intervalId != 0) {
  //   console.log("killing emulator");
  //   clearInterval(intervalId);
  //   intervalId = 0;
  //   gameBoy = null;
  // }

  intervalId = setInterval(() => {
    gameBoy.get_frame(ctx);
    console.log("running");
  }, 16);
};

document.getElementById("file-input").addEventListener(
  "change",
  async (event) => {
    await loadFile(event.target.files[0]);
    startGameBoy();
  },
  false
);

document.getElementById("start").addEventListener(
  "click",
  (e) => {
    if (null == gameBoy) {
      startGameBoy();
    }
  },
  false
);

document.addEventListener(
  "keydown",
  (event) => {
    if (null !== gameBoy) {
      console.log(event.key, "down");
      gameBoy.button_down(event.key);
    }
  },
  false
);

document.addEventListener(
  "keyup",
  (event) => {
    if (null !== gameBoy) {
      console.log(event.key, "up");
      gameBoy.button_up(event.key);
    }
  },
  false
);
