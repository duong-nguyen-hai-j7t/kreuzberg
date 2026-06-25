import 'package:test/test.dart';
import 'dart:typed_data';
import 'package:xberg/xberg.dart';
import 'package:xberg/src/xberg_bridge_generated/frb_generated.dart' show RustLib;

void main() {
  setUpAll(() async {
    await RustLib.init();
  });

  test('text extraction works', () async {
    final content = Uint8List.fromList('Hello world'.codeUnits);
    final result = await XbergBridge.extractBytesSync(content, 'text/plain');
    print('Text: ${result.content.substring(0, 5)}');
  });
}
