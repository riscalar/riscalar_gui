import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:riscalar_gui/src/rust/cpu/latency_core/components/cache.dart';
import 'package:riscalar_gui/src/rust/cpu/latency_core/components/functional_unit_pool.dart';
import 'package:riscalar_gui/src/rust/cpu/latency_core/latency_args.dart';

final configurationStateProvider =
    StateNotifierProvider<ConfigurationNotifier, LatencyArgs>(
  (ref) => ConfigurationNotifier(),
);

class ConfigurationNotifier extends StateNotifier<LatencyArgs> {
  ConfigurationNotifier()
      : super(
          LatencyArgs(
            binaryPath: '',
            lsqSize: 8,
            rsqSize: 16,
            runConfig: const RunConfig(),
            fetchConfig: const FetchConfig(
              fetchQueueSize: 4,
              fetchSpeed: 1,
              fetchBranchPenalty: 3,
            ),
            decodeConfig: const DecodeConfig(decodeWidth: 4),
            issueConfig: const IssueConfig(
              issueWidth: 4,
              issueOrder: IssueOrder.outOrder,
              issueNoMisspec: false,
            ),
            functionalUnitPoolConfig: const FunctionalUnitPoolConfig(
              ialu: FunctionalUnitGroupCfg(
                issueLatency: 1,
                operationLatency: 1,
                numUnits: 4,
              ),
              imult: FunctionalUnitGroupCfg(
                issueLatency: 1,
                operationLatency: 3,
                numUnits: 1,
              ),
              idiv: FunctionalUnitGroupCfg(
                issueLatency: 19,
                operationLatency: 20,
                numUnits: 1,
              ),
              load: FunctionalUnitGroupCfg(
                issueLatency: 1,
                operationLatency: 20,
                numUnits: 2,
              ),
              store: FunctionalUnitGroupCfg(
                issueLatency: 1,
                operationLatency: 20,
                numUnits: 2,
              ),
            ),
            commitConfig: const CommitConfig(commitWidth: 4),
            memoryConfig: MemoryConfig(
              memoryBusWidth: 8,
              memoryLatency: Uint64List.fromList([18, 2]),
            ),
            cacheConfig: const CacheConfig(
              il1: CacheOptions.configured(
                numSets: 512,
                associativity: 1,
                blockSize: 32,
                replacementPolicy: BlockReplacement.lru,
                latency: 1,
              ),
              dl1: CacheOptions.configured(
                numSets: 128,
                associativity: 4,
                blockSize: 32,
                replacementPolicy: BlockReplacement.lru,
                latency: 1,
              ),
              il2: CacheOptions.unified(),
              dl2: CacheOptions.configured(
                numSets: 1024,
                associativity: 4,
                blockSize: 64,
                replacementPolicy: BlockReplacement.lru,
                latency: 6,
              ),
            ),
          ),
        );

  void updateBinaryPath(String path) =>
      state = state.copyWith(binaryPath: path);

  void updateFastForward(int? instrs) =>
      state = state.copyWith.runConfig(fastForward: instrs);

  void updateEarlyTermination(int? instrs) =>
      state = state.copyWith.runConfig(maxInstrs: instrs);

  void updateStatFrequency(int? cycles) =>
      state = state.copyWith.runConfig(statFreq: cycles);

  void updateStatInterval(int? startCycle, int? endCycle) {
    if (startCycle == null || endCycle == null) {
      state = state.copyWith.runConfig(statInterval: null);
    } else {
      state = state.copyWith
          .runConfig(statInterval: Uint64List.fromList([startCycle, endCycle]));
    }
  }

  void updateFetchDispatchQueueSize(int size) =>
      state = state.copyWith.fetchConfig(fetchQueueSize: size);

  void updateFetchSpeed(int speed) =>
      state = state.copyWith.fetchConfig(fetchSpeed: speed);

  void updateFetchBranchPenalty(int penalty) =>
      state = state.copyWith.fetchConfig(fetchBranchPenalty: penalty);

  void updateLsqSize(int size) => state = state.copyWith(lsqSize: size);

  void updateRsqSize(int size) => state = state.copyWith(rsqSize: size);

  void updateDecodeWidth(int width) =>
      state = state.copyWith.decodeConfig(decodeWidth: width);

  void updateIssueWidth(int width) =>
      state = state.copyWith.issueConfig(issueWidth: width);

  void updateIssueOrder(IssueOrder order) =>
      state = state.copyWith.issueConfig(issueOrder: order);

  void updateIssueNoMisspec({required bool noMisspec}) =>
      state = state.copyWith.issueConfig(issueNoMisspec: noMisspec);

  void updateCommitWidth(int width) =>
      state = state.copyWith.commitConfig(commitWidth: width);
}
