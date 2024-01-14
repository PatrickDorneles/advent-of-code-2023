import { getFileText } from "./utils/get-file-text";

const fileText = await getFileText(import.meta.dir + "/day1.input");

console.time("result gen");

const replacementMap = new Map([
  ["one", "one1one"],
  ["two", "two2two"],
  ["three", "three3three"],
  ["four", "four4four"],
  ["five", "five5five"],
  ["six", "six6six"],
  ["seven", "seven7seven"],
  ["eight", "eight8eight"],
  ["nine", "nine9nine"],
]);

const result = fileText
  .trim()
  .split("\n")
  .map((value) => {
    replacementMap.forEach(
      (newVal, oldVal) => (value = value.replaceAll(oldVal, newVal)),
    );
    return value;
  })
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
