import 'package:flutter/material.dart';
import 'package:cuevas_app/src/rust/api/cuevas.dart';
import 'package:cuevas_app/src/rust/frb_generated.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MaterialApp(home: PantallaCueva()));
}

class PantallaCueva extends StatefulWidget {
  const PantallaCueva({super.key});

  @override
  State<PantallaCueva> createState() => _PantallaCuevaState();
}

class _PantallaCuevaState extends State<PantallaCueva> {
  Mapa? _mapa;
  int _semilla = 1;
  String? _error;

  @override
  void initState() {
    super.initState();
    _generar();
  }

  Future<void> _generar() async {
    try {
      final mapa = await generarCueva(
        ancho: 96,
        alto: 64,
        semilla: BigInt.from(_semilla),
        rellenoPct: 45,
        pasos: 5,
      );
      setState(() {
        _mapa = mapa;
        _error = null;
      });
    } catch (e) {
      setState(() => _error = '$e');
    }
  }

  @override
  Widget build(BuildContext context) {
    final mapa = _mapa;
    return Scaffold(
      appBar: AppBar(title: Text('Cueva · semilla $_semilla')),
      body: Center(
        child: _error != null
            ? Text(_error!)
            : mapa == null
            ? const CircularProgressIndicator()
            : AspectRatio(
                aspectRatio: mapa.ancho / mapa.alto,
                child: CustomPaint(painter: PintorCueva(mapa)),
              ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: () {
          _semilla++;
          _generar();
        },
        child: const Icon(Icons.refresh),
      ),
    );
  }
}

/// Dibuja cada celda de pared como un cuadro. En un juego de Flame, las mismas celdas sirven para armar los tiles.
class PintorCueva extends CustomPainter {
  PintorCueva(this.mapa);
  final Mapa mapa;

  @override
  void paint(Canvas canvas, Size size) {
    final lado = size.width / mapa.ancho;
    canvas.drawRect(
      Offset.zero & size,
      Paint()..color = const Color(0xFFE8D9B5),
    );
    final pared = Paint()..color = const Color(0xFF3B2F2F);
    for (var y = 0; y < mapa.alto; y++) {
      for (var x = 0; x < mapa.ancho; x++) {
        if (mapa.celdas[y * mapa.ancho + x] == 1) {
          canvas.drawRect(Rect.fromLTWH(x * lado, y * lado, lado, lado), pared);
        }
      }
    }
  }

  @override
  bool shouldRepaint(PintorCueva oldDelegate) => oldDelegate.mapa != mapa;
}
