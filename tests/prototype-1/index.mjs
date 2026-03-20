import { runCli } from "./test-entry.mjs"

const exitCode = await runCli()
process.exit(exitCode)
