export function shouldSkipLogging() {
  return process.env.NODE_ENV === "test";
}
