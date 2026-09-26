import assert from 'node:assert/strict'
import { layout } from '../build/jco/plugin.layout.js'

const config = { maxAreas: 8, maxHandles: 8, minPanelSize: 20, handleHalfSize: 3 }
const initialized = layout.initScreen({ w: 640, h: 480, config, rootContentId: 'root' })
assert.equal(initialized.document.screenW, 640)
assert.equal(initialized.document.screenH, 480)
assert.equal(initialized.document.areas.length, 1)
assert.equal(initialized.document.areas[0].contentId, 'root')
assert.equal(initialized.info.action, 'init-screen')

const resized = layout.resizeScreen({ document: initialized.document, w: 800, h: 600, handleHalfSize: 4 })
assert.equal(resized.document.screenW, 800)
assert.equal(resized.document.screenH, 600)
assert.equal(resized.info.action, 'resize-screen')
assert.throws(() => layout.initScreen({ w: 0, h: 480, config, rootContentId: 'root' }))
console.log('plugin.layout standalone component test: ok')
