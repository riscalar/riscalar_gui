// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.22.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'cache.freezed.dart';

enum BlockReplacement {
  lru,
  fifo,
  random,
}

@freezed
sealed class CacheOptions with _$CacheOptions {
  const factory CacheOptions.none() = CacheOptions_None;
  const factory CacheOptions.unified() = CacheOptions_Unified;
  const factory CacheOptions.configured({
    required int numSets,
    required int associativity,
    required int blockSize,
    required BlockReplacement replacementPolicy,
    required int latency,
  }) = CacheOptions_Configured;
}
