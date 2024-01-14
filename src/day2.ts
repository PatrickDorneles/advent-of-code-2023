import { getFileText } from "./utils/get-file-text";

const fileText = await getFileText(import.meta.dir + "/day2.input");

console.time("hey");

const rows = fileText.trim().split("\n");

let sum = 0;

for (const row of rows) {
  const [gameText, turnsText] = row.split(": ");

  const id = gameText.replace("Game ", "");
  const turns = turnsText.split("; ");

  let maxRedInTurn = 0;
  let maxBlueInTurn = 0;
  let maxGreenInTurn = 0;

  for (const turn of turns) {
    turn.split(", ").map((qtyPerColor) => {
      const [qty, color] = qtyPerColor.split(" ");
      if (color === "red" && Number(qty) > maxRedInTurn) {
        maxRedInTurn = Number(qty);
      }
      if (color === "blue" && Number(qty) > maxBlueInTurn) {
        maxBlueInTurn = Number(qty);
      }
      if (color === "green" && Number(qty) > maxGreenInTurn) {
        maxGreenInTurn = Number(qty);
      }
    });
  }

  if (maxRedInTurn <= 12 && maxBlueInTurn <= 14 && maxGreenInTurn <= 13) {
    sum += Number(id);
  }
}

console.timeEnd("hey");
console.log(sum);
