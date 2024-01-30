// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'category_stats.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

CategoryStats _$CategoryStatsFromJson(Map<String, dynamic> json) {
  return _CategoryStats.fromJson(json);
}

/// @nodoc
mixin _$CategoryStats {
  int get total => throw _privateConstructorUsedError;
  Map<String, int> get classBreakdown => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $CategoryStatsCopyWith<CategoryStats> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $CategoryStatsCopyWith<$Res> {
  factory $CategoryStatsCopyWith(
          CategoryStats value, $Res Function(CategoryStats) then) =
      _$CategoryStatsCopyWithImpl<$Res, CategoryStats>;
  @useResult
  $Res call({int total, Map<String, int> classBreakdown});
}

/// @nodoc
class _$CategoryStatsCopyWithImpl<$Res, $Val extends CategoryStats>
    implements $CategoryStatsCopyWith<$Res> {
  _$CategoryStatsCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? total = null,
    Object? classBreakdown = null,
  }) {
    return _then(_value.copyWith(
      total: null == total
          ? _value.total
          : total // ignore: cast_nullable_to_non_nullable
              as int,
      classBreakdown: null == classBreakdown
          ? _value.classBreakdown
          : classBreakdown // ignore: cast_nullable_to_non_nullable
              as Map<String, int>,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$CategoryStatsImplCopyWith<$Res>
    implements $CategoryStatsCopyWith<$Res> {
  factory _$$CategoryStatsImplCopyWith(
          _$CategoryStatsImpl value, $Res Function(_$CategoryStatsImpl) then) =
      __$$CategoryStatsImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({int total, Map<String, int> classBreakdown});
}

/// @nodoc
class __$$CategoryStatsImplCopyWithImpl<$Res>
    extends _$CategoryStatsCopyWithImpl<$Res, _$CategoryStatsImpl>
    implements _$$CategoryStatsImplCopyWith<$Res> {
  __$$CategoryStatsImplCopyWithImpl(
      _$CategoryStatsImpl _value, $Res Function(_$CategoryStatsImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? total = null,
    Object? classBreakdown = null,
  }) {
    return _then(_$CategoryStatsImpl(
      total: null == total
          ? _value.total
          : total // ignore: cast_nullable_to_non_nullable
              as int,
      classBreakdown: null == classBreakdown
          ? _value._classBreakdown
          : classBreakdown // ignore: cast_nullable_to_non_nullable
              as Map<String, int>,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$CategoryStatsImpl implements _CategoryStats {
  const _$CategoryStatsImpl(
      {required this.total, required final Map<String, int> classBreakdown})
      : _classBreakdown = classBreakdown;

  factory _$CategoryStatsImpl.fromJson(Map<String, dynamic> json) =>
      _$$CategoryStatsImplFromJson(json);

  @override
  final int total;
  final Map<String, int> _classBreakdown;
  @override
  Map<String, int> get classBreakdown {
    if (_classBreakdown is EqualUnmodifiableMapView) return _classBreakdown;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableMapView(_classBreakdown);
  }

  @override
  String toString() {
    return 'CategoryStats(total: $total, classBreakdown: $classBreakdown)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$CategoryStatsImpl &&
            (identical(other.total, total) || other.total == total) &&
            const DeepCollectionEquality()
                .equals(other._classBreakdown, _classBreakdown));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(
      runtimeType, total, const DeepCollectionEquality().hash(_classBreakdown));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$CategoryStatsImplCopyWith<_$CategoryStatsImpl> get copyWith =>
      __$$CategoryStatsImplCopyWithImpl<_$CategoryStatsImpl>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$CategoryStatsImplToJson(
      this,
    );
  }
}

abstract class _CategoryStats implements CategoryStats {
  const factory _CategoryStats(
      {required final int total,
      required final Map<String, int> classBreakdown}) = _$CategoryStatsImpl;

  factory _CategoryStats.fromJson(Map<String, dynamic> json) =
      _$CategoryStatsImpl.fromJson;

  @override
  int get total;
  @override
  Map<String, int> get classBreakdown;
  @override
  @JsonKey(ignore: true)
  _$$CategoryStatsImplCopyWith<_$CategoryStatsImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
