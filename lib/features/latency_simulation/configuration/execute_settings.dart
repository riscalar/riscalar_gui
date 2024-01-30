import 'package:flutter/cupertino.dart' hide OverlayVisibilityMode;
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:macos_ui/macos_ui.dart';

class ExecuteSettings extends ConsumerStatefulWidget {
  const ExecuteSettings({super.key});

  @override
  ConsumerState<ConsumerStatefulWidget> createState() =>
      _ExecuteSettingsState();
}

class _ExecuteSettingsState extends ConsumerState<ExecuteSettings> {
  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.all(20),
      child: Column(
        children: [
          CupertinoListSection.insetGrouped(
            backgroundColor: MacosColors.transparent,
            header: const Text('Functional Unit Pool'),
            children: [
              CupertinoListTile(
                title: const Text('Integer ALU'),
                padding: const EdgeInsets.all(15),
                subtitle: const Text(
                  'Integer ALU Configuration (Adds, Subs, Shifts, etc.)',
                ),
                trailing: SizedBox(
                  width: 400,
                  child: Row(
                    children: [
                      Flexible(
                        flex: 3,
                        child: MacosTextField(
                          decoration: const BoxDecoration(
                            color: MacosColors.transparent,
                          ),
                          prefix: Text(
                            'Issue:',
                            style: TextStyle(
                              color: CupertinoColors.secondaryLabel
                                  .resolveFrom(context),
                            ),
                          ),
                          prefixMode: OverlayVisibilityMode.editing,
                          controller: TextEditingController(text: '1'),
                          suffix: const Text('cycle(s)'),
                          suffixMode: OverlayVisibilityMode.editing,
                          placeholder: 'Issue Latency',
                          textAlign: TextAlign.end,
                        ),
                      ),
                      Flexible(
                        flex: 4,
                        child: MacosTextField(
                          decoration: const BoxDecoration(
                            color: MacosColors.transparent,
                          ),
                          prefix: Text(
                            'Operation:',
                            style: TextStyle(
                              color: CupertinoColors.secondaryLabel
                                  .resolveFrom(context),
                            ),
                          ),
                          prefixMode: OverlayVisibilityMode.editing,
                          controller: TextEditingController(text: '1'),
                          suffix: const Text('cycle(s)'),
                          suffixMode: OverlayVisibilityMode.editing,
                          placeholder: 'Operation Latency',
                          textAlign: TextAlign.end,
                        ),
                      ),
                      Flexible(
                        flex: 2,
                        child: MacosTextField(
                          decoration: const BoxDecoration(
                            color: MacosColors.transparent,
                          ),
                          controller: TextEditingController(text: '4'),
                          suffix: const Text('unit(s)'),
                          suffixMode: OverlayVisibilityMode.editing,
                          placeholder: 'Number of Units',
                          textAlign: TextAlign.end,
                        ),
                      ),
                    ],
                  ),
                ),
              ),
              CupertinoListTile(
                title: const Text('Integer Multiplier'),
                padding: const EdgeInsets.all(15),
                subtitle: const Text(
                  'Integer Multiplier Configuration',
                ),
                trailing: SizedBox(
                  width: 400,
                  child: Row(
                    children: [
                      Flexible(
                        flex: 3,
                        child: MacosTextField(
                          decoration: const BoxDecoration(
                            color: MacosColors.transparent,
                          ),
                          prefix: Text(
                            'Issue:',
                            style: TextStyle(
                              color: CupertinoColors.secondaryLabel
                                  .resolveFrom(context),
                            ),
                          ),
                          prefixMode: OverlayVisibilityMode.editing,
                          controller: TextEditingController(text: '1'),
                          suffix: const Text('cycle(s)'),
                          suffixMode: OverlayVisibilityMode.editing,
                          placeholder: 'Issue Latency',
                          textAlign: TextAlign.end,
                        ),
                      ),
                      Flexible(
                        flex: 4,
                        child: MacosTextField(
                          decoration: const BoxDecoration(
                            color: MacosColors.transparent,
                          ),
                          prefix: Text(
                            'Operation:',
                            style: TextStyle(
                              color: CupertinoColors.secondaryLabel
                                  .resolveFrom(context),
                            ),
                          ),
                          prefixMode: OverlayVisibilityMode.editing,
                          controller: TextEditingController(text: '3'),
                          suffix: const Text('cycle(s)'),
                          suffixMode: OverlayVisibilityMode.editing,
                          placeholder: 'Operation Latency',
                          textAlign: TextAlign.end,
                        ),
                      ),
                      Flexible(
                        flex: 2,
                        child: MacosTextField(
                          decoration: const BoxDecoration(
                            color: MacosColors.transparent,
                          ),
                          controller: TextEditingController(text: '1'),
                          suffix: const Text('unit(s)'),
                          suffixMode: OverlayVisibilityMode.editing,
                          placeholder: 'Number of Units',
                          textAlign: TextAlign.end,
                        ),
                      ),
                    ],
                  ),
                ),
              ),
              CupertinoListTile(
                title: const Text('Integer Divider'),
                padding: const EdgeInsets.all(15),
                subtitle: const Text(
                  'Integer Divider Configuration',
                ),
                trailing: SizedBox(
                  width: 400,
                  child: Row(
                    children: [
                      Flexible(
                        flex: 3,
                        child: MacosTextField(
                          decoration: const BoxDecoration(
                            color: MacosColors.transparent,
                          ),
                          prefix: Text(
                            'Issue:',
                            style: TextStyle(
                              color: CupertinoColors.secondaryLabel
                                  .resolveFrom(context),
                            ),
                          ),
                          prefixMode: OverlayVisibilityMode.editing,
                          controller: TextEditingController(text: '19'),
                          suffix: const Text('cycle(s)'),
                          suffixMode: OverlayVisibilityMode.editing,
                          placeholder: 'Issue Latency',
                          textAlign: TextAlign.end,
                        ),
                      ),
                      Flexible(
                        flex: 4,
                        child: MacosTextField(
                          decoration: const BoxDecoration(
                            color: MacosColors.transparent,
                          ),
                          prefix: Text(
                            'Operation:',
                            style: TextStyle(
                              color: CupertinoColors.secondaryLabel
                                  .resolveFrom(context),
                            ),
                          ),
                          prefixMode: OverlayVisibilityMode.editing,
                          controller: TextEditingController(text: '20'),
                          suffix: const Text('cycle(s)'),
                          suffixMode: OverlayVisibilityMode.editing,
                          placeholder: 'Operation Latency',
                          textAlign: TextAlign.end,
                        ),
                      ),
                      Flexible(
                        flex: 2,
                        child: MacosTextField(
                          decoration: const BoxDecoration(
                            color: MacosColors.transparent,
                          ),
                          controller: TextEditingController(text: '1'),
                          suffix: const Text('unit(s)'),
                          suffixMode: OverlayVisibilityMode.editing,
                          placeholder: 'Number of Units',
                          textAlign: TextAlign.end,
                        ),
                      ),
                    ],
                  ),
                ),
              ),
              CupertinoListTile(
                title: const Text('Load Unit'),
                padding: const EdgeInsets.all(15),
                subtitle: const Text(
                  'Load Unit Configuration (Memory Access)',
                ),
                trailing: SizedBox(
                  width: 400,
                  child: Row(
                    children: [
                      Flexible(
                        flex: 3,
                        child: MacosTextField(
                          decoration: const BoxDecoration(
                            color: MacosColors.transparent,
                          ),
                          prefix: Text(
                            'Issue:',
                            style: TextStyle(
                              color: CupertinoColors.secondaryLabel
                                  .resolveFrom(context),
                            ),
                          ),
                          prefixMode: OverlayVisibilityMode.editing,
                          controller: TextEditingController(text: '1'),
                          suffix: const Text('cycle(s)'),
                          suffixMode: OverlayVisibilityMode.editing,
                          placeholder: 'Issue Latency',
                          textAlign: TextAlign.end,
                        ),
                      ),
                      Flexible(
                        flex: 4,
                        child: MacosTextField(
                          decoration: const BoxDecoration(
                            color: MacosColors.transparent,
                          ),
                          prefix: Text(
                            'Operation:',
                            style: TextStyle(
                              color: CupertinoColors.secondaryLabel
                                  .resolveFrom(context),
                            ),
                          ),
                          prefixMode: OverlayVisibilityMode.editing,
                          controller: TextEditingController(text: '20'),
                          suffix: const Text('cycle(s)'),
                          suffixMode: OverlayVisibilityMode.editing,
                          placeholder: 'Operation Latency',
                          textAlign: TextAlign.end,
                        ),
                      ),
                      Flexible(
                        flex: 2,
                        child: MacosTextField(
                          decoration: const BoxDecoration(
                            color: MacosColors.transparent,
                          ),
                          controller: TextEditingController(text: '2'),
                          suffix: const Text('unit(s)'),
                          suffixMode: OverlayVisibilityMode.editing,
                          placeholder: 'Number of Units',
                          textAlign: TextAlign.end,
                        ),
                      ),
                    ],
                  ),
                ),
              ),
              CupertinoListTile(
                title: const Text('Store Unit'),
                padding: const EdgeInsets.all(15),
                subtitle: const Text(
                  'Store Unit Configuration (Memory Access)',
                ),
                trailing: SizedBox(
                  width: 400,
                  child: Row(
                    children: [
                      Flexible(
                        flex: 3,
                        child: MacosTextField(
                          decoration: const BoxDecoration(
                            color: MacosColors.transparent,
                          ),
                          prefix: Text(
                            'Issue:',
                            style: TextStyle(
                              color: CupertinoColors.secondaryLabel
                                  .resolveFrom(context),
                            ),
                          ),
                          prefixMode: OverlayVisibilityMode.editing,
                          controller: TextEditingController(text: '1'),
                          suffix: const Text('cycle(s)'),
                          suffixMode: OverlayVisibilityMode.editing,
                          placeholder: 'Issue Latency',
                          textAlign: TextAlign.end,
                        ),
                      ),
                      Flexible(
                        flex: 4,
                        child: MacosTextField(
                          decoration: const BoxDecoration(
                            color: MacosColors.transparent,
                          ),
                          prefix: Text(
                            'Operation:',
                            style: TextStyle(
                              color: CupertinoColors.secondaryLabel
                                  .resolveFrom(context),
                            ),
                          ),
                          prefixMode: OverlayVisibilityMode.editing,
                          controller: TextEditingController(text: '20'),
                          suffix: const Text('cycle(s)'),
                          suffixMode: OverlayVisibilityMode.editing,
                          placeholder: 'Operation Latency',
                          textAlign: TextAlign.end,
                        ),
                      ),
                      Flexible(
                        flex: 2,
                        child: MacosTextField(
                          decoration: const BoxDecoration(
                            color: MacosColors.transparent,
                          ),
                          controller: TextEditingController(text: '2'),
                          suffix: const Text('unit(s)'),
                          suffixMode: OverlayVisibilityMode.editing,
                          placeholder: 'Number of Units',
                          textAlign: TextAlign.end,
                        ),
                      ),
                    ],
                  ),
                ),
              ),
            ],
          ),
        ],
      ),
    );
  }
}
