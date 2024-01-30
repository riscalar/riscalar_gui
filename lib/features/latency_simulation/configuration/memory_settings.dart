import 'package:flutter/cupertino.dart' hide OverlayVisibilityMode;
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:macos_ui/macos_ui.dart';

class MemorySettings extends ConsumerStatefulWidget {
  const MemorySettings({super.key});

  @override
  ConsumerState<ConsumerStatefulWidget> createState() => _MemorySettingsState();
}

class _MemorySettingsState extends ConsumerState<MemorySettings> {
  int busWidth = 8;
  final _l1Controller = MacosTabController(length: 3);
  final _l2Controller = MacosTabController(length: 3);

  @override
  Widget build(BuildContext context) {
    _l1Controller.addListener(() {
      setState(() {});
    });
    _l2Controller.addListener(() {
      setState(() {});
    });
    return Padding(
      padding: const EdgeInsets.all(20),
      child: Column(
        children: [
          CupertinoListSection.insetGrouped(
            backgroundColor: MacosColors.transparent,
            header: const Text('Memory Configuration'),
            children: [
              CupertinoListTile(
                title: const Text('Memory Bus Width'),
                padding: const EdgeInsets.all(15),
                subtitle: const Text(
                  'Memory Bus Width (bytes)',
                ),
                trailing: MacosPopupButton<int>(
                  value: busWidth,
                  menuMaxHeight: 240,
                  onChanged: (int? newValue) {
                    setState(() => busWidth = newValue!);
                  },
                  items: <int>[1, 2, 4, 8, 16, 32, 64, 128]
                      .map<MacosPopupMenuItem<int>>((int value) {
                    return MacosPopupMenuItem<int>(
                      value: value,
                      child: Text('$value Bytes'),
                    );
                  }).toList(),
                ),
              ),
              CupertinoListTile(
                title: const Text('Access Latency'),
                padding: const EdgeInsets.all(15),
                subtitle: const Text(
                  'Total Access Latency = FIRST + SUBSEQUENT cycles',
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
                            'First:',
                            style: TextStyle(
                              color: CupertinoColors.secondaryLabel
                                  .resolveFrom(context),
                            ),
                          ),
                          prefixMode: OverlayVisibilityMode.editing,
                          controller: TextEditingController(text: '18'),
                          suffix: const Text('cycle(s)'),
                          suffixMode: OverlayVisibilityMode.editing,
                          placeholder: 'First Chunk Latency',
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
                            'Subsequent:',
                            style: TextStyle(
                              color: CupertinoColors.secondaryLabel
                                  .resolveFrom(context),
                            ),
                          ),
                          prefixMode: OverlayVisibilityMode.editing,
                          controller: TextEditingController(text: '2'),
                          suffix: const Text('cycle(s)'),
                          suffixMode: OverlayVisibilityMode.editing,
                          placeholder: 'Subsequent Chunk Latency',
                          textAlign: TextAlign.end,
                        ),
                      ),
                    ],
                  ),
                ),
              ),
            ],
          ),
          CupertinoListSection.insetGrouped(
            backgroundColor: MacosColors.transparent,
            header: const Text('L1 Cache Configuration'),
            children: [
              CupertinoListTile(
                title: const Text('Type'),
                padding: const EdgeInsets.all(15),
                trailing: MacosSegmentedControl(
                  controller: _l1Controller,
                  tabs: [
                    MacosTab(
                      label: 'None',
                      active: _l1Controller.index == 0,
                    ),
                    MacosTab(
                      label: 'Unified',
                      active: _l1Controller.index == 1,
                    ),
                    MacosTab(
                      label: 'Seperate',
                      active: _l1Controller.index == 2,
                    )
                  ],
                ),
              ),
              if (_l1Controller.index == 1)
                const CacheConfigWidget(cacheName: 'Unified L1 Cache'),
              if (_l1Controller.index == 2)
                const CacheConfigWidget(cacheName: 'L1 Instruction Cache'),
              if (_l1Controller.index == 2)
                const CacheConfigWidget(cacheName: 'L1 Data Cache'),
            ],
          ),
          if (_l1Controller.index != 0)
            CupertinoListSection.insetGrouped(
              backgroundColor: MacosColors.transparent,
              header: const Text('L2 Cache Configuration'),
              children: [
                CupertinoListTile(
                  title: const Text('Type'),
                  padding: const EdgeInsets.all(15),
                  trailing: MacosSegmentedControl(
                    controller: _l2Controller,
                    tabs: [
                      MacosTab(
                        label: 'None',
                        active: _l2Controller.index == 0,
                      ),
                      MacosTab(
                        label: 'Unified',
                        active: _l2Controller.index == 1,
                      ),
                      MacosTab(
                        label: 'Seperate',
                        active: _l2Controller.index == 2,
                      )
                    ],
                  ),
                ),
                if (_l2Controller.index == 1)
                  const CacheConfigWidget(cacheName: 'Unified L2 Cache'),
                if (_l2Controller.index == 2)
                  const CacheConfigWidget(cacheName: 'L2 Instruction Cache'),
                if (_l2Controller.index == 2)
                  const CacheConfigWidget(cacheName: 'L2 Data Cache'),
              ],
            ),
        ],
      ),
    );
  }
}

class CacheConfigWidget extends StatefulWidget {
  const CacheConfigWidget({
    required this.cacheName,
    super.key,
  });
  final String cacheName;

  @override
  State<CacheConfigWidget> createState() => _CacheConfigWidgetState();
}

class _CacheConfigWidgetState extends State<CacheConfigWidget> {
  String replacementPolicy = 'LRU';
  @override
  Widget build(BuildContext context) {
    return CupertinoListTile(
      title: Text(widget.cacheName),
      padding: const EdgeInsets.all(15),
      subtitle: Column(
        children: [
          Row(
            children: [
              Flexible(
                child: MacosTextField(
                  decoration: const BoxDecoration(
                    color: MacosColors.transparent,
                  ),
                  prefix: Text(
                    'Number of Sets:',
                    style: MacosTheme.of(context).typography.title3,
                  ),
                  prefixMode: OverlayVisibilityMode.editing,
                  controller: TextEditingController(text: '512'),
                  placeholder: 'Number of Sets',
                  textAlign: TextAlign.end,
                ),
              ),
              Flexible(
                child: MacosTextField(
                  decoration: const BoxDecoration(
                    color: MacosColors.transparent,
                  ),
                  prefix: Text(
                    'Block Size:',
                    style: MacosTheme.of(context).typography.title3,
                  ),
                  prefixMode: OverlayVisibilityMode.editing,
                  suffix: const Text('Bytes'),
                  suffixMode: OverlayVisibilityMode.editing,
                  controller: TextEditingController(text: '32'),
                  placeholder: 'Block Size',
                  textAlign: TextAlign.end,
                ),
              ),
              Flexible(
                child: MacosTextField(
                  decoration: const BoxDecoration(
                    color: MacosColors.transparent,
                  ),
                  prefix: Text(
                    'Associativity:',
                    style: MacosTheme.of(context).typography.title3,
                  ),
                  prefixMode: OverlayVisibilityMode.editing,
                  controller: TextEditingController(text: '4'),
                  placeholder: 'Number of Sets',
                  textAlign: TextAlign.end,
                ),
              ),
            ],
          ),
          Row(
            children: [
              Text(
                '  Replacement Policy:',
                style: MacosTheme.of(context).typography.title3,
              ),
              const Spacer(),
              MacosPopupButton<String>(
                value: replacementPolicy,
                menuMaxHeight: 240,
                onChanged: (String? newValue) => setState(
                  () => replacementPolicy = newValue!,
                ),
                items: <String>['LRU', 'FIFO', 'Random']
                    .map<MacosPopupMenuItem<String>>(
                      (String value) => MacosPopupMenuItem<String>(
                        value: value,
                        child: Text(value),
                      ),
                    )
                    .toList(),
              ),
            ],
          ),
          MacosTextField(
            decoration: const BoxDecoration(
              color: MacosColors.transparent,
            ),
            prefix: Text(
              'Access Latency:',
              style: MacosTheme.of(context).typography.title3,
            ),
            prefixMode: OverlayVisibilityMode.editing,
            controller: TextEditingController(text: '1'),
            suffix: const Text('cycle(s)'),
            suffixMode: OverlayVisibilityMode.editing,
            placeholder: 'Access Latency',
            textAlign: TextAlign.end,
          ),
        ],
      ),
    );
  }
}
