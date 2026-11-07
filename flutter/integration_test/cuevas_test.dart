import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:cuevas_app/src/rust/api/cuevas.dart';
import 'package:cuevas_app/src/rust/frb_generated.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  setUpAll(() async => await RustLib.init());

  Future<Mapa> cueva(int semilla) => generarCueva(
    ancho: 80,
    alto: 50,
    semilla: BigInt.from(semilla),
    rellenoPct: 45,
    pasos: 5,
  );

  test('la cueva de Rust llega completa y es repetible', () async {
    final a = await cueva(7);
    final b = await cueva(7);
    expect(a.celdas.length, 80 * 50);
    expect(a.celdas, b.celdas);
    expect(a.celdas.first, 1); // la esquina es pared
  });

  test('un parámetro fuera de rango llega como excepción', () async {
    await expectLater(
      generarCueva(
        ancho: 4,
        alto: 4,
        semilla: BigInt.one,
        rellenoPct: 45,
        pasos: 5,
      ),
      throwsA(isA<AnyhowException>()),
    );
  });
}
