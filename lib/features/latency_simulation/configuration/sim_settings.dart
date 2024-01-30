import 'package:file_selector/file_selector.dart';
import 'package:flutter/cupertino.dart';
import 'package:flutter/services.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:macos_ui/macos_ui.dart';
import 'package:riscalar_gui/features/latency_simulation/configuration/state.dart';

class PerformanceSimulationSettingsPanel extends ConsumerStatefulWidget {
  const PerformanceSimulationSettingsPanel({super.key});

  @override
  ConsumerState<ConsumerStatefulWidget> createState() =>
      _PerformanceSimulationSettingsPanelState();
}

class _PerformanceSimulationSettingsPanelState
    extends ConsumerState<PerformanceSimulationSettingsPanel> {
  bool fastForward = false;
  bool terminateEarly = false;
  bool timeStats = false;
  TextEditingController statIntervalStart = TextEditingController();
  TextEditingController statIntervalEnd = TextEditingController();
  String? timeStat1;
  String? timeStat2;
  String? timeStat3;

  XFile? binary;
  @override
  Widget build(BuildContext context) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        CupertinoListSection.insetGrouped(
          header: const Text('Simulation Binary'),
          backgroundColor: MacosTheme.of(context).canvasColor,
          children: [
            CupertinoListTile(
              title: const Text('Compiled Program File'),
              padding: const EdgeInsets.all(15),
              backgroundColor: MacosColors.controlColor,
              trailing: PushButton(
                controlSize: ControlSize.large,
                color: binary == null
                    ? MacosColors.systemBlueColor
                    : MacosColors.systemRedColor,
                onPressed: () async {
                  if (binary != null) {
                    ref
                        .read(configurationStateProvider.notifier)
                        .updateBinaryPath('');
                    setState(() => binary = null);
                    return;
                  } else {
                    final result = await openFile(
                      acceptedTypeGroups: [const XTypeGroup()],
                    );
                    if (result == null) return;
                    ref
                        .read(configurationStateProvider.notifier)
                        .updateBinaryPath(result.path);
                    setState(() => binary = result);
                  }
                },
                child: Text(binary == null ? 'Upload' : 'Remove'),
              ),
              // subtitle: Text('Instructions to Fast Forward'),
            ),
            if (binary != null)
              CupertinoListTile(
                leading: const MacosIcon(
                  CupertinoIcons.doc_plaintext,
                  color: MacosColors.textColor,
                ),
                title: Text(
                  binary!.name,
                  style: MacosTheme.of(context).typography.title3,
                ),
                backgroundColor: MacosColors.controlColor,
                // subtitle: Text('Instructions to Fast Forward'),
              ),
          ],
        ),
        CupertinoListSection.insetGrouped(
          // clipBehavior: Clip.antiAlias,
          hasLeading: false,
          backgroundColor: MacosTheme.of(context).canvasColor,
          header: const Text('Simulation Settings'),
          children: [
            CupertinoListTile(
              backgroundColor: MacosColors.controlColor,
              padding: const EdgeInsets.all(15),
              title: const Text('Fast Forward Instructions'),
              subtitle: const Text('Executes first N instructions '
                  'functionally'),
              trailing: MacosSwitch(
                value: fastForward,
                onChanged: (value) {
                  if (value == false) {
                    ref
                        .read(configurationStateProvider.notifier)
                        .updateFastForward(null);
                  }
                  setState(() => fastForward = value);
                },
              ),
            ),
            if (fastForward)
              CupertinoListTile(
                backgroundColor: MacosColors.controlColor,
                title: MacosTextField(
                  decoration: const BoxDecoration(
                    color: MacosColors.transparent,
                  ),
                  prefix: Text(
                    '    # of Instructions: ',
                    style: MacosTheme.of(context).typography.title3,
                  ),
                  textAlign: TextAlign.end,
                  placeholder: 'Enter a Number',
                  inputFormatters: [FilteringTextInputFormatter.digitsOnly],
                  onChanged: (value) => ref
                      .read(configurationStateProvider.notifier)
                      .updateFastForward(int.parse(value)),
                  readOnly: !fastForward,
                ),
              ),
            CupertinoListTile(
              title: const Text('Terminate Early'),
              backgroundColor: MacosColors.controlColor,
              subtitle: const Text('Stops latency simulation after '
                  'N instructions'),
              padding: const EdgeInsets.all(15),
              trailing: MacosSwitch(
                value: terminateEarly,
                onChanged: (value) {
                  if (value == false) {
                    ref
                        .read(configurationStateProvider.notifier)
                        .updateEarlyTermination(null);
                  }
                  setState(() => terminateEarly = value);
                },
              ),
            ),
            if (terminateEarly)
              CupertinoListTile(
                backgroundColor: MacosColors.controlColor,
                title: MacosTextField(
                  decoration: const BoxDecoration(
                    color: MacosColors.transparent,
                  ),
                  prefix: Text(
                    '    # of Instructions: ',
                    style: MacosTheme.of(context).typography.title3,
                  ),
                  textAlign: TextAlign.end,
                  placeholder: 'Enter a Number',
                  inputFormatters: [FilteringTextInputFormatter.digitsOnly],
                  onChanged: (value) => ref
                      .read(configurationStateProvider.notifier)
                      .updateEarlyTermination(int.parse(value)),
                  readOnly: !terminateEarly,
                ),
              ),
          ],
        ),
        CupertinoListSection.insetGrouped(
          // clipBehavior: Clip.antiAlias,
          hasLeading: false,
          backgroundColor: MacosTheme.of(context).canvasColor,
          header: const Text('Time-Varying Statistics Collection'),
          children: [
            CupertinoListTile(
              backgroundColor: MacosColors.controlColor,
              padding: const EdgeInsets.all(15),
              title: const Text('Collect Time-Varying Statistics'),
              subtitle: const Text('Periodically collect statistics samples'),
              trailing: MacosSwitch(
                value: timeStats,
                onChanged: (value) {
                  if (value == false) {
                    ref.read(configurationStateProvider.notifier)
                      ..updateStatFrequency(null)
                      ..updateStatInterval(null, null);
                  }
                  setState(() => timeStats = value);
                },
              ),
            ),
            if (timeStats)
              CupertinoListTile(
                backgroundColor: MacosColors.controlColor,
                title: MacosTextField(
                  decoration: const BoxDecoration(
                    color: MacosColors.transparent,
                  ),
                  prefix: Text(
                    '    Period (Cycles): ',
                    style: MacosTheme.of(context).typography.title3,
                  ),
                  textAlign: TextAlign.end,
                  placeholder: 'Collect every N cycles',
                  inputFormatters: [FilteringTextInputFormatter.digitsOnly],
                  onChanged: (value) => ref
                      .read(configurationStateProvider.notifier)
                      .updateStatFrequency(int.tryParse(value)),
                  readOnly: !timeStats,
                ),
              ),
            if (timeStats)
              CupertinoListTile(
                backgroundColor: MacosColors.controlColor,
                title: Row(
                  children: [
                    Flexible(
                      flex: 7,
                      child: Text(
                        '      Collection Interval: ',
                        style: MacosTheme.of(context).typography.title3,
                      ),
                    ),
                    const Spacer(),
                    Flexible(
                      flex: 3,
                      child: MacosTextField(
                        decoration: const BoxDecoration(
                          color: MacosColors.transparent,
                        ),
                        placeholder: 'Start',
                        textAlign: TextAlign.end,
                        inputFormatters: [
                          FilteringTextInputFormatter.digitsOnly
                        ],
                        controller: statIntervalStart,
                        onChanged: (value) {
                          if (statIntervalEnd.text.isNotEmpty) {
                            ref
                                .read(configurationStateProvider.notifier)
                                .updateStatInterval(
                                  int.tryParse(statIntervalStart.text),
                                  int.tryParse(statIntervalEnd.text),
                                );
                          }
                        },
                        readOnly: !timeStats,
                      ),
                    ),
                    Flexible(
                      flex: 3,
                      child: MacosTextField(
                        decoration: const BoxDecoration(
                          color: MacosColors.transparent,
                        ),
                        placeholder: 'End',
                        textAlign: TextAlign.end,
                        inputFormatters: [
                          FilteringTextInputFormatter.digitsOnly
                        ],
                        controller: statIntervalEnd,
                        onChanged: (value) {
                          if (statIntervalStart.text.isNotEmpty) {
                            ref
                                .read(configurationStateProvider.notifier)
                                .updateStatInterval(
                                  int.tryParse(statIntervalStart.text),
                                  int.tryParse(statIntervalEnd.text),
                                );
                          }
                        },
                        readOnly: !timeStats,
                      ),
                    ),
                  ],
                ),
              ),
            if (timeStats)
              CupertinoListTile(
                backgroundColor: MacosColors.controlColor,
                title: Row(
                  children: [
                    Text(
                      '      Statistic 1: ',
                      style: MacosTheme.of(context).typography.title3,
                    ),
                    const Spacer(),
                    MacosPopupButton<String>(
                      value: timeStat1,
                      onChanged: (String? newValue) {
                        setState(() => timeStat1 = newValue);
                      },
                      items:
                          stats.map<MacosPopupMenuItem<String>>((String value) {
                        return MacosPopupMenuItem<String>(
                          value: value,
                          child: Text(value),
                        );
                      }).toList(),
                    ),
                  ],
                ),
              ),
            if (timeStats)
              CupertinoListTile(
                backgroundColor: MacosColors.controlColor,
                // subtitle: Text('Instructions to Fast Forward'),
                title: Row(
                  children: [
                    Text(
                      '      Statistic 2: ',
                      style: MacosTheme.of(context).typography.title3,
                    ),
                    const Spacer(),
                    MacosPopupButton<String>(
                      value: timeStat2,
                      onChanged: (String? newValue) {
                        setState(() => timeStat2 = newValue);
                      },
                      items:
                          stats.map<MacosPopupMenuItem<String>>((String value) {
                        return MacosPopupMenuItem<String>(
                          value: value,
                          child: Text(value),
                        );
                      }).toList(),
                    ),
                  ],
                ),
              ),
            if (timeStats)
              CupertinoListTile(
                backgroundColor: MacosColors.controlColor,
                title: Row(
                  children: [
                    Text(
                      '      Statistic 3: ',
                      style: MacosTheme.of(context).typography.title3,
                    ),
                    const Spacer(),
                    MacosPopupButton<String>(
                      value: timeStat3,
                      onChanged: (String? newValue) {
                        setState(() => timeStat3 = newValue);
                      },
                      items:
                          stats.map<MacosPopupMenuItem<String>>((String value) {
                        return MacosPopupMenuItem<String>(
                          value: value,
                          child: Text(value),
                        );
                      }).toList(),
                    ),
                  ],
                ),
              ),
          ],
        )
      ],
    );
  }
}

const stats = [
  '',
  'IPC',
  'CPI',
  'Committed Instructions',
  'Executed Instructions',
  'LSQ Entries',
  'RSQ Entries',
  'L1I Hits',
  'L1I Misses',
  'L1D Hits',
  'L1D Misses',
  'L1U Hits',
  'L1U Misses',
  'L2I Hits',
  'L2I Misses',
  'L2D Hits',
  'L2D Misses',
];
