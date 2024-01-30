import 'package:freezed_annotation/freezed_annotation.dart';

part 'category_stats.freezed.dart';
part 'category_stats.g.dart';

@freezed
// Class that contains statistics related to instructions executed
class CategoryStats with _$CategoryStats {
  const factory CategoryStats({
    required int total,
    required Map<String, int> classBreakdown,
  }) = _CategoryStats;

  factory CategoryStats.fromJson(Map<String, dynamic> json) =>
      _$CategoryStatsFromJson(json);
}
