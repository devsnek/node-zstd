'use strict';

const { DecompressStream: InternalDecompressStream } = require('../pkg');

let group;
if (globalThis.FinalizationGroup) {
  group = new FinalizationGroup((it) => {
    for (const ptr of it) {
      InternalDecompressStream.__wrap(ptr).free();
    }
  })
}

class DecompressStream extends InternalDecompressStream {
  constructor(...args) {
    super(...args);

    if (group !== undefined) {
      group.register(this, this.ptr, this);
    }
  }

  free(...args) {
    if (group !== undefined) {
      group.unregister(this);
    }
    return super.free(...args);
  }
}

module.exports = { DecompressStream };
