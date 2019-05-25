'use strict';

const { create_stream: createStream, decompress } = require('../pkg');

class DecompressStream {
  constructor() {
    this.stream = createStream();
  }

  decompress(data) {
    const d = decompress(this.stream, new Uint8Array(data));
    return d.buffer;
  }
}

module.exports = { DecompressStream };
