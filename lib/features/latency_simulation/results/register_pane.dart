import 'package:flutter/cupertino.dart';
import 'package:macos_ui/macos_ui.dart';
import 'package:riscalar_gui/src/rust/cpu/register.dart';

class ResultsRegisterPane extends StatefulWidget {
  const ResultsRegisterPane(
    this.registers, {
    required this.scrollController,
    super.key,
  });

  final ScrollController scrollController;
  final XRegisterFile registers;

  @override
  State<ResultsRegisterPane> createState() => _ResultsRegisterPaneState();
}

class _ResultsRegisterPaneState extends State<ResultsRegisterPane> {
  final controller = MacosTabController(length: 2);

  @override
  Widget build(BuildContext context) {
    controller.addListener(() {
      setState(() {});
    });
    final radix = controller.index == 0 ? 16 : 10;

    return Center(
      child: SingleChildScrollView(
        controller: widget.scrollController,
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Padding(
              padding: const EdgeInsets.fromLTRB(8, 15, 8, 0),
              child: Text(
                'Registers',
                style: MacosTheme.of(context).typography.title1,
              ),
            ),
            Padding(
              padding: const EdgeInsets.all(8),
              child: MacosSegmentedControl(
                tabs: const [
                  MacosTab(label: 'Hex'),
                  MacosTab(label: 'Dec'),
                ],
                controller: controller,
              ),
            ),
            CupertinoListSection.insetGrouped(
              header: Text(
                'Zero Register',
                style: MacosTheme.of(context).typography.title3,
              ),
              backgroundColor: MacosTheme.of(context).canvasColor,
              children: [
                CupertinoListTile(
                  leading: const Text('X0'),
                  title: const Text('ZERO'),
                  backgroundColor: MacosColors.controlColor,
                  trailing:
                      Text(widget.registers.regs[00].toRadixString(radix)),
                )
              ],
            ),
            CupertinoListSection.insetGrouped(
              header: Text(
                'Return Address',
                style: MacosTheme.of(context).typography.title3,
              ),
              backgroundColor: MacosTheme.of(context).canvasColor,
              children: [
                CupertinoListTile(
                  leading: const Text('X1'),
                  title: const Text('RA'),
                  backgroundColor: MacosColors.controlColor,
                  trailing:
                      Text(widget.registers.regs[01].toRadixString(radix)),
                )
              ],
            ),
            CupertinoListSection.insetGrouped(
              header: Text(
                'Pointers',
                style: MacosTheme.of(context).typography.title3,
              ),
              backgroundColor: MacosTheme.of(context).canvasColor,
              children: [
                CupertinoListTile(
                  leading: const Text('X2'),
                  title: const Text('SP'),
                  backgroundColor: MacosColors.controlColor,
                  trailing:
                      Text(widget.registers.regs[02].toRadixString(radix)),
                ),
                CupertinoListTile(
                  leading: const Text('X3'),
                  title: const Text('GP'),
                  backgroundColor: MacosColors.controlColor,
                  trailing:
                      Text(widget.registers.regs[03].toRadixString(radix)),
                ),
                CupertinoListTile(
                  leading: const Text('X4'),
                  title: const Text('TP'),
                  backgroundColor: MacosColors.controlColor,
                  trailing:
                      Text(widget.registers.regs[04].toRadixString(radix)),
                )
              ],
            ),
            CupertinoListSection.insetGrouped(
              header: Text(
                'Temporary Registers',
                style: MacosTheme.of(context).typography.title3,
              ),
              backgroundColor: MacosTheme.of(context).canvasColor,
              children: [
                CupertinoListTile(
                  leading: const Text('X5'),
                  title: const Text('T0'),
                  backgroundColor: MacosColors.controlColor,
                  trailing:
                      Text(widget.registers.regs[05].toRadixString(radix)),
                ),
                CupertinoListTile(
                  leading: const Text('X6'),
                  title: const Text('T1'),
                  backgroundColor: MacosColors.controlColor,
                  trailing:
                      Text(widget.registers.regs[06].toRadixString(radix)),
                ),
                CupertinoListTile(
                  leading: const Text('X7'),
                  title: const Text('T2'),
                  backgroundColor: MacosColors.controlColor,
                  trailing:
                      Text(widget.registers.regs[07].toRadixString(radix)),
                )
              ],
            ),
            CupertinoListSection.insetGrouped(
              header: Text(
                'Saved Register / Frame Pointer',
                style: MacosTheme.of(context).typography.title3,
              ),
              backgroundColor: MacosTheme.of(context).canvasColor,
              children: [
                CupertinoListTile(
                  leading: const Text('X8'),
                  title: const Text('S0'),
                  backgroundColor: MacosColors.controlColor,
                  trailing:
                      Text(widget.registers.regs[08].toRadixString(radix)),
                )
              ],
            ),
            CupertinoListSection.insetGrouped(
              header: Text(
                'Saved Register',
                style: MacosTheme.of(context).typography.title3,
              ),
              backgroundColor: MacosTheme.of(context).canvasColor,
              children: [
                CupertinoListTile(
                  leading: const Text('X9'),
                  title: const Text('S1'),
                  backgroundColor: MacosColors.controlColor,
                  trailing:
                      Text(widget.registers.regs[09].toRadixString(radix)),
                )
              ],
            ),
            CupertinoListSection.insetGrouped(
              header: Text(
                'Function Arguments/Return Values',
                style: MacosTheme.of(context).typography.title3,
              ),
              backgroundColor: MacosTheme.of(context).canvasColor,
              children: [
                CupertinoListTile(
                  leading: const Text('X10'),
                  title: const Text('A0'),
                  backgroundColor: MacosColors.controlColor,
                  trailing:
                      Text(widget.registers.regs[10].toRadixString(radix)),
                ),
                CupertinoListTile(
                  leading: const Text('X11'),
                  title: const Text('A1'),
                  backgroundColor: MacosColors.controlColor,
                  trailing:
                      Text(widget.registers.regs[11].toRadixString(radix)),
                )
              ],
            ),
            CupertinoListSection.insetGrouped(
              header: Text(
                'Function Arguments',
                style: MacosTheme.of(context).typography.title3,
              ),
              backgroundColor: MacosTheme.of(context).canvasColor,
              children: [
                CupertinoListTile(
                  leading: const Text('X12'),
                  title: const Text('A2'),
                  backgroundColor: MacosColors.controlColor,
                  trailing:
                      Text(widget.registers.regs[12].toRadixString(radix)),
                ),
                CupertinoListTile(
                  leading: const Text('X13'),
                  title: const Text('A3'),
                  backgroundColor: MacosColors.controlColor,
                  trailing:
                      Text(widget.registers.regs[13].toRadixString(radix)),
                ),
                CupertinoListTile(
                  leading: const Text('X14'),
                  title: const Text('A4'),
                  backgroundColor: MacosColors.controlColor,
                  trailing:
                      Text(widget.registers.regs[14].toRadixString(radix)),
                ),
                CupertinoListTile(
                  leading: const Text('X15'),
                  title: const Text('A5'),
                  backgroundColor: MacosColors.controlColor,
                  trailing:
                      Text(widget.registers.regs[15].toRadixString(radix)),
                ),
                CupertinoListTile(
                  leading: const Text('X16'),
                  title: const Text('A6'),
                  backgroundColor: MacosColors.controlColor,
                  trailing:
                      Text(widget.registers.regs[16].toRadixString(radix)),
                ),
                CupertinoListTile(
                  leading: const Text('X17'),
                  title: const Text('A7'),
                  backgroundColor: MacosColors.controlColor,
                  trailing:
                      Text(widget.registers.regs[17].toRadixString(radix)),
                )
              ],
            ),
            CupertinoListSection.insetGrouped(
              header: Text(
                'Saved Registers',
                style: MacosTheme.of(context).typography.title3,
              ),
              backgroundColor: MacosTheme.of(context).canvasColor,
              children: [
                CupertinoListTile(
                  leading: const Text('X18'),
                  title: const Text('S2'),
                  backgroundColor: MacosColors.controlColor,
                  trailing:
                      Text(widget.registers.regs[18].toRadixString(radix)),
                ),
                CupertinoListTile(
                  leading: const Text('X19'),
                  title: const Text('S3'),
                  backgroundColor: MacosColors.controlColor,
                  trailing:
                      Text(widget.registers.regs[19].toRadixString(radix)),
                ),
                CupertinoListTile(
                  leading: const Text('X20'),
                  title: const Text('S4'),
                  backgroundColor: MacosColors.controlColor,
                  trailing:
                      Text(widget.registers.regs[20].toRadixString(radix)),
                ),
                CupertinoListTile(
                  leading: const Text('X21'),
                  title: const Text('S5'),
                  backgroundColor: MacosColors.controlColor,
                  trailing:
                      Text(widget.registers.regs[21].toRadixString(radix)),
                ),
                CupertinoListTile(
                  leading: const Text('X22'),
                  title: const Text('S6'),
                  backgroundColor: MacosColors.controlColor,
                  trailing:
                      Text(widget.registers.regs[22].toRadixString(radix)),
                ),
                CupertinoListTile(
                  leading: const Text('X23'),
                  title: const Text('S7'),
                  backgroundColor: MacosColors.controlColor,
                  trailing:
                      Text(widget.registers.regs[23].toRadixString(radix)),
                ),
                CupertinoListTile(
                  leading: const Text('X24'),
                  title: const Text('S8'),
                  backgroundColor: MacosColors.controlColor,
                  trailing:
                      Text(widget.registers.regs[24].toRadixString(radix)),
                ),
                CupertinoListTile(
                  leading: const Text('X25'),
                  title: const Text('S9'),
                  backgroundColor: MacosColors.controlColor,
                  trailing:
                      Text(widget.registers.regs[25].toRadixString(radix)),
                ),
                CupertinoListTile(
                  leading: const Text('X26'),
                  title: const Text('S10'),
                  backgroundColor: MacosColors.controlColor,
                  trailing:
                      Text(widget.registers.regs[26].toRadixString(radix)),
                ),
                CupertinoListTile(
                  leading: const Text('X27'),
                  title: const Text('S11'),
                  backgroundColor: MacosColors.controlColor,
                  trailing:
                      Text(widget.registers.regs[27].toRadixString(radix)),
                )
              ],
            ),
            CupertinoListSection.insetGrouped(
              header: Text(
                'Temporary Registers',
                style: MacosTheme.of(context).typography.title3,
              ),
              backgroundColor: MacosTheme.of(context).canvasColor,
              children: [
                CupertinoListTile(
                  leading: const Text('X28'),
                  title: const Text('T3'),
                  backgroundColor: MacosColors.controlColor,
                  trailing:
                      Text(widget.registers.regs[28].toRadixString(radix)),
                ),
                CupertinoListTile(
                  leading: const Text('X29'),
                  title: const Text('T4'),
                  backgroundColor: MacosColors.controlColor,
                  trailing:
                      Text(widget.registers.regs[29].toRadixString(radix)),
                ),
                CupertinoListTile(
                  leading: const Text('X30'),
                  title: const Text('T5'),
                  backgroundColor: MacosColors.controlColor,
                  trailing:
                      Text(widget.registers.regs[30].toRadixString(radix)),
                ),
                CupertinoListTile(
                  leading: const Text('X31'),
                  title: const Text('T6'),
                  backgroundColor: MacosColors.controlColor,
                  trailing:
                      Text(widget.registers.regs[31].toRadixString(radix)),
                )
              ],
            ),
          ],
        ),
      ),
    );
  }
}
