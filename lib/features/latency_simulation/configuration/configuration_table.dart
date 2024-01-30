import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:macos_ui/macos_ui.dart';
import 'package:riscalar_gui/features/latency_simulation/configuration/state.dart';
import 'package:riscalar_gui/src/rust/cpu/latency_core/latency_args.dart';

class LatencyConfigurationTable extends ConsumerWidget {
  const LatencyConfigurationTable({required this.scrollController, super.key});
  final ScrollController scrollController;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final cfg = ref.watch(configurationStateProvider);
    return SingleChildScrollView(
      controller: scrollController,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          Center(
            child: Padding(
              padding: const EdgeInsets.fromLTRB(8, 15, 8, 8),
              child: Text(
                'System Configuration',
                style: MacosTheme.of(context).typography.title1,
              ),
            ),
          ),
          DataTable(
            border: TableBorder(
              horizontalInside: BorderSide(
                color: CupertinoDynamicColor.resolve(
                  MacosColors.quaternaryLabelColor,
                  context,
                ),
              ),
            ),
            dividerThickness: 0.3,
            headingTextStyle: MacosTheme.of(context).typography.title3.copyWith(
                  color: CupertinoDynamicColor.resolve(
                    MacosColors.systemBlueColor,
                    context,
                  ),
                ),
            dataTextStyle: MacosTheme.of(context).typography.body,
            columns: const <DataColumn>[
              DataColumn(label: Expanded(child: Text('Parameter    '))),
              DataColumn(label: Expanded(child: Text('Configuration'))),
            ],
            rows: <DataRow>[
              DataRow(
                cells: <DataCell>[
                  const DataCell(Text('Program File')),
                  DataCell(Text(cfg.binaryPath.split('/').last)),
                ],
              ),
              if (cfg.runConfig.fastForward != null)
                DataRow(
                  cells: <DataCell>[
                    const DataCell(Text('Fast Forwarded Instructions')),
                    DataCell(Text('${cfg.runConfig.fastForward} Instructions')),
                  ],
                ),
              if (cfg.runConfig.maxInstrs != null)
                DataRow(
                  cells: <DataCell>[
                    const DataCell(Text('Early Terminatation')),
                    DataCell(Text('${cfg.runConfig.maxInstrs} Instructions')),
                  ],
                ),
              DataRow(
                cells: <DataCell>[
                  const DataCell(Text('Fetch Dispatch Queue Size')),
                  DataCell(
                    Text('${cfg.fetchConfig.fetchQueueSize} Instructions'),
                  ),
                ],
              ),
              DataRow(
                cells: <DataCell>[
                  const DataCell(Text('Fetch Speed')),
                  DataCell(Text('${cfg.fetchConfig.fetchSpeed}x')),
                ],
              ),
              DataRow(
                cells: <DataCell>[
                  const DataCell(Text('Fetch Branch Penalty')),
                  DataCell(
                    Text('${cfg.fetchConfig.fetchBranchPenalty} Cycles'),
                  ),
                ],
              ),
              const DataRow(
                cells: <DataCell>[
                  DataCell(Text('Branch Predictor')),
                  DataCell(Text('Perfect')),
                ],
              ),
              DataRow(
                cells: <DataCell>[
                  const DataCell(Text('Dispatch Stage Width')),
                  DataCell(
                    Text('${cfg.decodeConfig.decodeWidth} Instructions/Cycle'),
                  )
                ],
              ),
              DataRow(
                cells: <DataCell>[
                  const DataCell(Text('Reservation Station Queue Size')),
                  DataCell(Text('${cfg.rsqSize} Instructions'))
                ],
              ),
              DataRow(
                cells: <DataCell>[
                  const DataCell(Text('Load Store Queue Size')),
                  DataCell(Text('${cfg.lsqSize} Instructions'))
                ],
              ),
              DataRow(
                cells: <DataCell>[
                  const DataCell(Text('Issue Stage Width')),
                  DataCell(
                    Text('${cfg.issueConfig.issueWidth} Instructions/Cycle'),
                  ),
                ],
              ),
              DataRow(
                cells: <DataCell>[
                  const DataCell(Text('Issue Order')),
                  DataCell(
                    Text(
                      cfg.issueConfig.issueOrder == IssueOrder.outOrder
                          ? 'Out of Order'
                          : 'In Order',
                    ),
                  ),
                ],
              ),
              DataRow(
                cells: <DataCell>[
                  const DataCell(
                    Text('Disable Execution of Mispredicted Instructions'),
                  ),
                  DataCell(
                    Text(cfg.issueConfig.issueNoMisspec ? 'True' : 'False'),
                  ),
                ],
              ),
              DataRow(
                cells: <DataCell>[
                  const DataCell(
                    Text('Disable Execution of Mispredicted Instructions'),
                  ),
                  DataCell(
                    Text(cfg.issueConfig.issueNoMisspec ? 'True' : 'False'),
                  ),
                ],
              ),
              DataRow(
                cells: <DataCell>[
                  const DataCell(Text('Functional Unit - iALU')),
                  DataCell(
                    Text(
                      '${cfg.functionalUnitPoolConfig.ialu.numUnits} Units with '
                      'Issue Latency = '
                      '${cfg.functionalUnitPoolConfig.ialu.issueLatency} Cycles '
                      'and Operation Latency = '
                      '${cfg.functionalUnitPoolConfig.ialu.operationLatency} '
                      'Cycles',
                    ),
                  ),
                ],
              ),
              DataRow(
                cells: <DataCell>[
                  const DataCell(Text('Functional Unit - iMult')),
                  DataCell(
                    Text(
                      '${cfg.functionalUnitPoolConfig.imult.numUnits} Units with '
                      'Issue Latency = '
                      '${cfg.functionalUnitPoolConfig.imult.issueLatency} Cycles '
                      'and Operation Latency = '
                      '${cfg.functionalUnitPoolConfig.imult.operationLatency} '
                      'Cycles',
                    ),
                  ),
                ],
              ),
              DataRow(
                cells: <DataCell>[
                  const DataCell(Text('Functional Unit - iDiv')),
                  DataCell(
                    Text(
                      '${cfg.functionalUnitPoolConfig.idiv.numUnits} Units with '
                      'Issue Latency = '
                      '${cfg.functionalUnitPoolConfig.idiv.issueLatency} Cycles '
                      'and Operation Latency = '
                      '${cfg.functionalUnitPoolConfig.idiv.operationLatency} '
                      'Cycles',
                    ),
                  ),
                ],
              ),
              DataRow(
                cells: <DataCell>[
                  const DataCell(Text('Functional Unit - iDiv')),
                  DataCell(
                    Text(
                      '${cfg.functionalUnitPoolConfig.idiv.numUnits} Units with '
                      'Issue Latency = '
                      '${cfg.functionalUnitPoolConfig.idiv.issueLatency} Cycles '
                      'and Operation Latency = '
                      '${cfg.functionalUnitPoolConfig.idiv.operationLatency} '
                      'Cycles',
                    ),
                  ),
                ],
              ),
              DataRow(
                cells: <DataCell>[
                  const DataCell(Text('Functional Unit - Load')),
                  DataCell(
                    Text(
                      '${cfg.functionalUnitPoolConfig.load.numUnits} Units with '
                      'Issue Latency = '
                      '${cfg.functionalUnitPoolConfig.load.issueLatency} Cycles '
                      'and Operation Latency = '
                      '${cfg.functionalUnitPoolConfig.load.operationLatency} '
                      'Cycles',
                    ),
                  ),
                ],
              ),
              DataRow(
                cells: <DataCell>[
                  const DataCell(Text('Functional Unit - Store')),
                  DataCell(
                    Text(
                      '${cfg.functionalUnitPoolConfig.store.numUnits} Units with '
                      'Issue Latency = '
                      '${cfg.functionalUnitPoolConfig.store.issueLatency} Cycles '
                      'and Operation Latency = '
                      '${cfg.functionalUnitPoolConfig.store.operationLatency} '
                      'Cycles',
                    ),
                  ),
                ],
              ),
              DataRow(
                cells: <DataCell>[
                  const DataCell(Text('Commmit Stage Width')),
                  DataCell(
                    Text('${cfg.commitConfig.commitWidth} Instructions/Cycle'),
                  ),
                ],
              ),
              DataRow(
                cells: <DataCell>[
                  const DataCell(Text('Memory Bus Width')),
                  DataCell(Text('${cfg.memoryConfig.memoryBusWidth} Bytes')),
                ],
              ),
              DataRow(
                cells: <DataCell>[
                  const DataCell(Text('Memory Access Latency')),
                  DataCell(
                    Text(
                      '${cfg.memoryConfig.memoryLatency} - [First, Subseqent]',
                    ),
                  ),
                ],
              ),
              DataRow(
                cells: <DataCell>[
                  const DataCell(Text('L1 Cache Configuation')),
                  DataCell(
                    Text(
                      '${cfg.memoryConfig.memoryLatency} - [First, Subseqent]',
                    ),
                  ),
                ],
              ),
            ],
          ),
        ],
      ),
    );
  }
}
