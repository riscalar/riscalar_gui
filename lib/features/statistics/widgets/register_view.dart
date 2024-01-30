import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

enum NumberView { hex, dec }

class RegisterView extends ConsumerStatefulWidget {
  const RegisterView(this.registers, {super.key});
  final Map<String, dynamic> registers;

  @override
  ConsumerState<RegisterView> createState() => _RegisterViewState();
}

class _RegisterViewState extends ConsumerState<RegisterView> {
  NumberView _numberView = NumberView.hex;

  @override
  Widget build(BuildContext context) {
    return Column(
      mainAxisAlignment: MainAxisAlignment.center,
      mainAxisSize: MainAxisSize.min,
      children: [
        Text(
          'Register State',
          style: Theme.of(context).textTheme.titleLarge,
        ),
        TextButton(
          onPressed: () {
            setState(() {
              _numberView = _numberView == NumberView.dec
                  ? NumberView.hex
                  : NumberView.dec;
            });
          },
          child: Text(
            _numberView == NumberView.dec ? 'DECIMAL VIEW' : 'HEXIDECIMAL VIEW',
            style: Theme.of(context).textTheme.bodyMedium,
          ),
        ),
        Table(
          defaultColumnWidth: const FixedColumnWidth(160),
          defaultVerticalAlignment: TableCellVerticalAlignment.middle,
          border: TableBorder.all(),
          children: [
            for (var i = 0; i < 8; i++)
              TableRow(
                children: [
                  for (var j = 0; j < 4; j++)
                    Padding(
                      padding: const EdgeInsets.all(8),
                      child: Row(
                        children: [
                          Text(
                            '${widget.registers.keys.elementAt(i * 4 + j)}:'
                                .padLeft(4),
                            textAlign: TextAlign.left,
                            style: const TextStyle(
                              fontFeatures: [FontFeature.tabularFigures()],
                            ),
                          ),
                          const Spacer(),

                          // Display the value in hex or decimal
                          if (_numberView == NumberView.hex)
                            Text(
                              // ignore: lines_longer_than_80_chars, avoid_dynamic_calls
                              '0x${widget.registers.values.elementAt(i * 4 + j).toRadixString(16).padLeft(8, '0')}',
                              style: const TextStyle(
                                fontFeatures: [FontFeature.tabularFigures()],
                              ),
                            )
                          else
                            Text(
                              '${widget.registers.values.elementAt(i * 4 + j)}',
                              style: const TextStyle(
                                fontFeatures: [FontFeature.tabularFigures()],
                              ),
                            ),
                        ],
                      ),
                    )
                ],
              ),
          ],
        ),
      ],
    );
  }
}
