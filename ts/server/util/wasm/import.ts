export async function importWasm(
  filePath: string,
): Promise<WebAssembly.Instance> {
  const wasmCode = await Deno.readFile(filePath);
  const wasmModule = new WebAssembly.Module(wasmCode);
  const wasmInstance = new WebAssembly.Instance(wasmModule);
  return wasmInstance;
}
