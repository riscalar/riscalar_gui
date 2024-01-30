// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'simple.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

/// @nodoc
mixin _$SimulationResult {
  Object get field0 => throw _privateConstructorUsedError;
  String get field1 => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(
            XRegisterFile field0, String field1, String field2)
        success,
    required TResult Function(String field0, String field1) error,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(XRegisterFile field0, String field1, String field2)?
        success,
    TResult? Function(String field0, String field1)? error,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(XRegisterFile field0, String field1, String field2)?
        success,
    TResult Function(String field0, String field1)? error,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(SimulationResult_Success value) success,
    required TResult Function(SimulationResult_Error value) error,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(SimulationResult_Success value)? success,
    TResult? Function(SimulationResult_Error value)? error,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(SimulationResult_Success value)? success,
    TResult Function(SimulationResult_Error value)? error,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $SimulationResultCopyWith<SimulationResult> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $SimulationResultCopyWith<$Res> {
  factory $SimulationResultCopyWith(
          SimulationResult value, $Res Function(SimulationResult) then) =
      _$SimulationResultCopyWithImpl<$Res, SimulationResult>;
  @useResult
  $Res call({String field1});
}

/// @nodoc
class _$SimulationResultCopyWithImpl<$Res, $Val extends SimulationResult>
    implements $SimulationResultCopyWith<$Res> {
  _$SimulationResultCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field1 = null,
  }) {
    return _then(_value.copyWith(
      field1: null == field1
          ? _value.field1
          : field1 // ignore: cast_nullable_to_non_nullable
              as String,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$SimulationResult_SuccessImplCopyWith<$Res>
    implements $SimulationResultCopyWith<$Res> {
  factory _$$SimulationResult_SuccessImplCopyWith(
          _$SimulationResult_SuccessImpl value,
          $Res Function(_$SimulationResult_SuccessImpl) then) =
      __$$SimulationResult_SuccessImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({XRegisterFile field0, String field1, String field2});

  $XRegisterFileCopyWith<$Res> get field0;
}

/// @nodoc
class __$$SimulationResult_SuccessImplCopyWithImpl<$Res>
    extends _$SimulationResultCopyWithImpl<$Res, _$SimulationResult_SuccessImpl>
    implements _$$SimulationResult_SuccessImplCopyWith<$Res> {
  __$$SimulationResult_SuccessImplCopyWithImpl(
      _$SimulationResult_SuccessImpl _value,
      $Res Function(_$SimulationResult_SuccessImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
    Object? field1 = null,
    Object? field2 = null,
  }) {
    return _then(_$SimulationResult_SuccessImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as XRegisterFile,
      null == field1
          ? _value.field1
          : field1 // ignore: cast_nullable_to_non_nullable
              as String,
      null == field2
          ? _value.field2
          : field2 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $XRegisterFileCopyWith<$Res> get field0 {
    return $XRegisterFileCopyWith<$Res>(_value.field0, (value) {
      return _then(_value.copyWith(field0: value));
    });
  }
}

/// @nodoc

class _$SimulationResult_SuccessImpl implements SimulationResult_Success {
  const _$SimulationResult_SuccessImpl(this.field0, this.field1, this.field2);

  @override
  final XRegisterFile field0;
  @override
  final String field1;
  @override
  final String field2;

  @override
  String toString() {
    return 'SimulationResult.success(field0: $field0, field1: $field1, field2: $field2)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SimulationResult_SuccessImpl &&
            (identical(other.field0, field0) || other.field0 == field0) &&
            (identical(other.field1, field1) || other.field1 == field1) &&
            (identical(other.field2, field2) || other.field2 == field2));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0, field1, field2);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$SimulationResult_SuccessImplCopyWith<_$SimulationResult_SuccessImpl>
      get copyWith => __$$SimulationResult_SuccessImplCopyWithImpl<
          _$SimulationResult_SuccessImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(
            XRegisterFile field0, String field1, String field2)
        success,
    required TResult Function(String field0, String field1) error,
  }) {
    return success(field0, field1, field2);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(XRegisterFile field0, String field1, String field2)?
        success,
    TResult? Function(String field0, String field1)? error,
  }) {
    return success?.call(field0, field1, field2);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(XRegisterFile field0, String field1, String field2)?
        success,
    TResult Function(String field0, String field1)? error,
    required TResult orElse(),
  }) {
    if (success != null) {
      return success(field0, field1, field2);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(SimulationResult_Success value) success,
    required TResult Function(SimulationResult_Error value) error,
  }) {
    return success(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(SimulationResult_Success value)? success,
    TResult? Function(SimulationResult_Error value)? error,
  }) {
    return success?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(SimulationResult_Success value)? success,
    TResult Function(SimulationResult_Error value)? error,
    required TResult orElse(),
  }) {
    if (success != null) {
      return success(this);
    }
    return orElse();
  }
}

abstract class SimulationResult_Success implements SimulationResult {
  const factory SimulationResult_Success(
      final XRegisterFile field0,
      final String field1,
      final String field2) = _$SimulationResult_SuccessImpl;

  @override
  XRegisterFile get field0;
  @override
  String get field1;
  String get field2;
  @override
  @JsonKey(ignore: true)
  _$$SimulationResult_SuccessImplCopyWith<_$SimulationResult_SuccessImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$SimulationResult_ErrorImplCopyWith<$Res>
    implements $SimulationResultCopyWith<$Res> {
  factory _$$SimulationResult_ErrorImplCopyWith(
          _$SimulationResult_ErrorImpl value,
          $Res Function(_$SimulationResult_ErrorImpl) then) =
      __$$SimulationResult_ErrorImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String field0, String field1});
}

/// @nodoc
class __$$SimulationResult_ErrorImplCopyWithImpl<$Res>
    extends _$SimulationResultCopyWithImpl<$Res, _$SimulationResult_ErrorImpl>
    implements _$$SimulationResult_ErrorImplCopyWith<$Res> {
  __$$SimulationResult_ErrorImplCopyWithImpl(
      _$SimulationResult_ErrorImpl _value,
      $Res Function(_$SimulationResult_ErrorImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
    Object? field1 = null,
  }) {
    return _then(_$SimulationResult_ErrorImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
      null == field1
          ? _value.field1
          : field1 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$SimulationResult_ErrorImpl implements SimulationResult_Error {
  const _$SimulationResult_ErrorImpl(this.field0, this.field1);

  @override
  final String field0;
  @override
  final String field1;

  @override
  String toString() {
    return 'SimulationResult.error(field0: $field0, field1: $field1)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SimulationResult_ErrorImpl &&
            (identical(other.field0, field0) || other.field0 == field0) &&
            (identical(other.field1, field1) || other.field1 == field1));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0, field1);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$SimulationResult_ErrorImplCopyWith<_$SimulationResult_ErrorImpl>
      get copyWith => __$$SimulationResult_ErrorImplCopyWithImpl<
          _$SimulationResult_ErrorImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(
            XRegisterFile field0, String field1, String field2)
        success,
    required TResult Function(String field0, String field1) error,
  }) {
    return error(field0, field1);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(XRegisterFile field0, String field1, String field2)?
        success,
    TResult? Function(String field0, String field1)? error,
  }) {
    return error?.call(field0, field1);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(XRegisterFile field0, String field1, String field2)?
        success,
    TResult Function(String field0, String field1)? error,
    required TResult orElse(),
  }) {
    if (error != null) {
      return error(field0, field1);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(SimulationResult_Success value) success,
    required TResult Function(SimulationResult_Error value) error,
  }) {
    return error(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(SimulationResult_Success value)? success,
    TResult? Function(SimulationResult_Error value)? error,
  }) {
    return error?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(SimulationResult_Success value)? success,
    TResult Function(SimulationResult_Error value)? error,
    required TResult orElse(),
  }) {
    if (error != null) {
      return error(this);
    }
    return orElse();
  }
}

abstract class SimulationResult_Error implements SimulationResult {
  const factory SimulationResult_Error(
      final String field0, final String field1) = _$SimulationResult_ErrorImpl;

  @override
  String get field0;
  @override
  String get field1;
  @override
  @JsonKey(ignore: true)
  _$$SimulationResult_ErrorImplCopyWith<_$SimulationResult_ErrorImpl>
      get copyWith => throw _privateConstructorUsedError;
}
