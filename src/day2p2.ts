import { getFileText } from "./utils/get-file-text";

const fileText = await getFileText(import.meta.dir + "/day2.input");

console.time("hey2");

const rows = fileText.trim().split("\n");

let sum = 0;

for (const row of rows) {
  const [, turnsText] = row.split(": ");

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

  sum += maxRedInTurn * maxGreenInTurn * maxBlueInTurn;
}

console.timeEnd("hey2");
console.log(sum);
