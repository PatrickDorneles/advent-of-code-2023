import { getFileText } from "./utils/get-file-text";

const fileText = await getFileText(import.meta.dir + "/day1.input");

console.time("result gen");

const result = fileText
  .trim()
  .split("\n")
  .reduce((acc, value) => {
    let firstNumChar: string | undefined = undefined;
    let lastNumChar: string | undefined = undefined;

    for (let index = 0; index < value.length; index++) {
      if (!firstNumChar && !isNaN(parseInt(value[index]))) {
        firstNumChar = value[index];
      }

      if (!lastNumChar && !isNaN(parseInt(value[value.length - 1 - index]))) {
        lastNumChar = value[value.length - 1 - index];
      }

      if (firstNumChar && lastNumChar) {
        break;
      }
    }

    return String(Number(acc) + Number(`${firstNumChar}${lastNumChar}`));
  }, "0");

console.timeEnd("result gen");
console.log(result);

// Correct, yayy!
