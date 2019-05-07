{
  'targets': [
    {
      'target_name': 'zstd',
      'defines': [ 'NAPI_DISABLE_CPP_EXCEPTIONS' ],
      'msvs_settings': {
        'VCCLCompilerTool': {
          'ExceptionHandling': 0,
          'EnablePREfast': 'true',
        },
      },
      'xcode_settings': {
        'CLANG_CXX_LIBRARY': 'libc++',
        'MACOSX_DEPLOYMENT_TARGET': '10.7',
        'GCC_ENABLE_CPP_EXCEPTIONS': 'NO',
      },
      'include_dirs': [
        "<!@(node -p \"require('node-addon-api').include\")",
        "<(module_root_dir)/deps/zstd/lib",
      ],
      'dependencies': [
        "<!(node -p \"require('node-addon-api').gyp\")",
        "<(module_root_dir)/deps/zstd.gyp:decompressor",
      ],
      'sources': [
        'src/addon.cc',
      ],
      'conditions': [
        ['OS=="mac"', {
          'cflags+': ['-fvisibility=hidden'],
          'xcode_settings': {
            'OTHER_CFLAGS': ['-fvisibility=hidden']
          }
        }]
      ],
      'cflags': [ '-Werror', '-Wall', '-Wextra', '-Wpedantic', '-Wunused-parameter',  '-fno-exceptions' ],
      'cflags_cc': [ '-Werror', '-Wall', '-Wextra', '-Wpedantic', '-Wunused-parameter', '-fno-exceptions' ],
    },
  ],
}
