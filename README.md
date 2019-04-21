# zucc

```js
const { DecompressStream } = require('zucc');

const stream = new DecompressStream();
const ab = stream.decompress(arrayBufferOfCompressedData);
```
