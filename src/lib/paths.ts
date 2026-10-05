export function fileName(path: string): string {
  return path.split(/[\\/]/).pop() ?? path;
}

export function isIpk(path: string): boolean {
  return path.toLowerCase().endsWith(".ipk");
}
