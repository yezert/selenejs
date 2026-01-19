import fs from 'node:fs'
import path from 'node:path'

const pkgRoot = path.resolve(new URL('.', import.meta.url).pathname, '..')
const srcDir = path.join(pkgRoot, 'src', 'rust')
const outDir = path.join(pkgRoot, 'dist', 'rust')

function copyDirRecursive(from, to) {
  fs.mkdirSync(to, { recursive: true })
  for (const entry of fs.readdirSync(from, { withFileTypes: true })) {
    const src = path.join(from, entry.name)
    const dst = path.join(to, entry.name)
    if (entry.isDirectory()) {
      copyDirRecursive(src, dst)
    } else if (entry.isFile()) {
      fs.copyFileSync(src, dst)
    }
  }
}

if (!fs.existsSync(srcDir)) {
  console.warn('[copy-rust] src rust dir not found:', srcDir)
  process.exit(0)
}

copyDirRecursive(srcDir, outDir)
console.log('[copy-rust] copied', srcDir, '->', outDir)

