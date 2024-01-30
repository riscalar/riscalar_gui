// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'category_stats.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

_$CategoryStatsImpl _$$CategoryStatsImplFromJson(Map<String, dynamic> json) =>
    _$CategoryStatsImpl(
      total: json['total'] as int,
      classBreakdown: Map<String, int>.from(json['classBreakdown'] as Map),
    );

Map<String, dynamic> _$$CategoryStatsImplToJson(_$CategoryStatsImpl instance) =>
    <String, dynamic>{
      'total': instance.total,
      'classBreakdown': instance.classBreakdown,
    };
