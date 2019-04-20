#include <napi.h>
#include <zstd.h>

class DecompressStream : public Napi::ObjectWrap<DecompressStream> {
 public:
  static Napi::Object Init(Napi::Env env, Napi::Object exports) {
    Napi::HandleScope scope(env);

    Napi::Function func = DefineClass(env, "DecompressStream", {
      InstanceMethod("decompress", &DecompressStream::Decompress),
    });

    constructor = Napi::Persistent(func);
    constructor.SuppressDestruct();

    exports.Set("DecompressStream", func);
    return exports;
  }

  DecompressStream(const Napi::CallbackInfo& info) : Napi::ObjectWrap<DecompressStream>(info) {
     stream_ = ZSTD_createDStream();
     ZSTD_initDStream(stream_);
  }

 private:
  static Napi::FunctionReference constructor;

  Napi::Value Decompress(const Napi::CallbackInfo& info) {
    if (!info[0].IsArrayBuffer()) {
      NAPI_THROW(Napi::Error::New(Env(), "Argument must be an ArrayBuffer"), Napi::Value());
    }

    Napi::ArrayBuffer buffInJS = info[0].As<Napi::ArrayBuffer>();

    ZSTD_inBuffer input = { buffInJS.Data(), buffInJS.ByteLength(), 0 };

    size_t const buffOutSize = ZSTD_DStreamOutSize();
    void* const buffOut = malloc(buffOutSize);

    ZSTD_outBuffer output = { buffOut, buffOutSize, 0 };

    //while (input.pos < input.size) {
    //  output.pos = 0;
      size_t const r = ZSTD_decompressStream(stream_, &output, &input);
      if (ZSTD_isError(r)) {
        NAPI_THROW(Napi::Error::New(Env(), ZSTD_getErrorName(r)), Napi::Value());
      }
    //}

    return Napi::ArrayBuffer::New(Env(), output.dst, output.pos);
  }

  ZSTD_DStream* stream_;
};

Napi::FunctionReference DecompressStream::constructor;

Napi::Object Init(Napi::Env env, Napi::Object exports) {
  return DecompressStream::Init(env, exports);
}

NODE_API_MODULE(addon, Init)