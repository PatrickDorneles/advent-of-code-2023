export async function getFileText(path: string) {
  return await Bun.file(path).text();
}
