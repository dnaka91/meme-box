/**
 * This script is used to rename the binary with the platform specific postfix.
 * When `tauri build` is ran, it looks for the binary name appended with the platform specific postfix.
 */
 
const execa = require('execa')
const fs = require('fs')

let suffix = ''
switch (process.platform) {
  case 'linux':
    suffix = '-linux';
  case 'darwin':
    suffix = '-macos';	
}

let extension = ''
if (process.platform === 'win32') {
  extension = '.exe'
}

async function main() {
  const rustInfo = (await execa('rustc', ['-vV'])).stdout
  const targetTriple = /host: (\S+)/g.exec(rustInfo)[1]
  if (!targetTriple) {
    console.error('Failed to determine platform target triple')
  }
  fs.renameSync(
    `release/out/memebox${suffix}${extension}`,
    `release/out/memebox-${targetTriple}${extension}`
  )
}

main().catch((e) => {
  throw e
})