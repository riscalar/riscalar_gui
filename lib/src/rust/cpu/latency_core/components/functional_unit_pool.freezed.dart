// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'functional_unit_pool.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

/// @nodoc
mixin _$FunctionalUnitGroupCfg {
  int get operationLatency => throw _privateConstructorUsedError;
  int get issueLatency => throw _privateConstructorUsedError;
  int get numUnits => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $FunctionalUnitGroupCfgCopyWith<FunctionalUnitGroupCfg> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $FunctionalUnitGroupCfgCopyWith<$Res> {
  factory $FunctionalUnitGroupCfgCopyWith(FunctionalUnitGroupCfg value,
          $Res Function(FunctionalUnitGroupCfg) then) =
      _$FunctionalUnitGroupCfgCopyWithImpl<$Res, FunctionalUnitGroupCfg>;
  @useResult
  $Res call({int operationLatency, int issueLatency, int numUnits});
}

/// @nodoc
class _$FunctionalUnitGroupCfgCopyWithImpl<$Res,
        $Val extends FunctionalUnitGroupCfg>
    implements $FunctionalUnitGroupCfgCopyWith<$Res> {
  _$FunctionalUnitGroupCfgCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? operationLatency = null,
    Object? issueLatency = null,
    Object? numUnits = null,
  }) {
    return _then(_value.copyWith(
      operationLatency: null == operationLatency
          ? _value.operationLatency
          : operationLatency // ignore: cast_nullable_to_non_nullable
              as int,
      issueLatency: null == issueLatency
          ? _value.issueLatency
          : issueLatency // ignore: cast_nullable_to_non_nullable
              as int,
      numUnits: null == numUnits
          ? _value.numUnits
          : numUnits // ignore: cast_nullable_to_non_nullable
              as int,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$FunctionalUnitGroupCfgImplCopyWith<$Res>
    implements $FunctionalUnitGroupCfgCopyWith<$Res> {
  factory _$$FunctionalUnitGroupCfgImplCopyWith(
          _$FunctionalUnitGroupCfgImpl value,
          $Res Function(_$FunctionalUnitGroupCfgImpl) then) =
      __$$FunctionalUnitGroupCfgImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({int operationLatency, int issueLatency, int numUnits});
}

/// @nodoc
class __$$FunctionalUnitGroupCfgImplCopyWithImpl<$Res>
    extends _$FunctionalUnitGroupCfgCopyWithImpl<$Res,
        _$FunctionalUnitGroupCfgImpl>
    implements _$$FunctionalUnitGroupCfgImplCopyWith<$Res> {
  __$$FunctionalUnitGroupCfgImplCopyWithImpl(
      _$FunctionalUnitGroupCfgImpl _value,
      $Res Function(_$FunctionalUnitGroupCfgImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? operationLatency = null,
    Object? issueLatency = null,
    Object? numUnits = null,
  }) {
    return _then(_$FunctionalUnitGroupCfgImpl(
      operationLatency: null == operationLatency
          ? _value.operationLatency
          : operationLatency // ignore: cast_nullable_to_non_nullable
              as int,
      issueLatency: null == issueLatency
          ? _value.issueLatency
          : issueLatency // ignore: cast_nullable_to_non_nullable
              as int,
      numUnits: null == numUnits
          ? _value.numUnits
          : numUnits // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _$FunctionalUnitGroupCfgImpl implements _FunctionalUnitGroupCfg {
  const _$FunctionalUnitGroupCfgImpl(
      {required this.operationLatency,
      required this.issueLatency,
      required this.numUnits});

  @override
  final int operationLatency;
  @override
  final int issueLatency;
  @override
  final int numUnits;

  @override
  String toString() {
    return 'FunctionalUnitGroupCfg(operationLatency: $operationLatency, issueLatency: $issueLatency, numUnits: $numUnits)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$FunctionalUnitGroupCfgImpl &&
            (identical(other.operationLatency, operationLatency) ||
                other.operationLatency == operationLatency) &&
            (identical(other.issueLatency, issueLatency) ||
                other.issueLatency == issueLatency) &&
            (identical(other.numUnits, numUnits) ||
                other.numUnits == numUnits));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, operationLatency, issueLatency, numUnits);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$FunctionalUnitGroupCfgImplCopyWith<_$FunctionalUnitGroupCfgImpl>
      get copyWith => __$$FunctionalUnitGroupCfgImplCopyWithImpl<
          _$FunctionalUnitGroupCfgImpl>(this, _$identity);
}

abstract class _FunctionalUnitGroupCfg implements FunctionalUnitGroupCfg {
  const factory _FunctionalUnitGroupCfg(
      {required final int operationLatency,
      required final int issueLatency,
      required final int numUnits}) = _$FunctionalUnitGroupCfgImpl;

  @override
  int get operationLatency;
  @override
  int get issueLatency;
  @override
  int get numUnits;
  @override
  @JsonKey(ignore: true)
  _$$FunctionalUnitGroupCfgImplCopyWith<_$FunctionalUnitGroupCfgImpl>
      get copyWith => throw _privateConstructorUsedError;
}
